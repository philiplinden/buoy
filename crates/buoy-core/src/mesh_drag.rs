use avian3d::prelude::*;
use bevy::prelude::*;

use crate::atmosphere::Atmosphere;
use crate::balloon::BalloonPhysics;

pub(crate) fn plugin(app: &mut App) {
    app.register_type::<MeshDragCalculator>()
       .add_systems(FixedUpdate, calculate_mesh_drag);
}

/// Component for calculating drag based on mesh geometry
#[derive(Component, Debug, Clone, Reflect)]
pub struct MeshDragCalculator {
    pub drag_coefficient_base: f32,
    pub reynolds_scaling: f32,
    pub surface_roughness: f32,
}

impl Default for MeshDragCalculator {
    fn default() -> Self {
        Self {
            drag_coefficient_base: 0.47, // Sphere default
            reynolds_scaling: 0.1,
            surface_roughness: 0.01,
        }
    }
}

fn calculate_mesh_drag(
    mut query: Query<(
        &mut ExternalForce,
        &MeshDragCalculator,
        &BalloonPhysics,
        &LinearVelocity,
        &Position,
    )>,
    atmosphere: Res<Atmosphere>,
) {
    for (mut external_force, drag_calc, physics, velocity, position) in query.iter_mut() {
        // For a first implementation, use a simplified approach based on frontal area
        // Later, this would be expanded to use the actual mesh triangles

        let position_vec = Vec3::new(position.x, position.y, position.z);
        let ambient_density = atmosphere.density(position_vec);

        // Frontal area is approximated as a circle with the balloon's radius
        let frontal_area = std::f32::consts::PI * physics.radius.powi(2);

        // Calculate Reynolds number (simplified)
        let velocity_mag = velocity.0.length();
        let reynolds = velocity_mag * physics.radius * 2.0 * ambient_density.get::<uom::si::mass_density::kilogram_per_cubic_meter>() / 1.81e-5;

        // Adjust drag coefficient based on Reynolds number
        let drag_coef = if reynolds < 1e5 {
            drag_calc.drag_coefficient_base * (1.0 + drag_calc.reynolds_scaling)
        } else {
            drag_calc.drag_coefficient_base * (1.0 - drag_calc.reynolds_scaling)
        };

        // Calculate drag force
        let drag_force = if velocity_mag > 0.001 {
            -0.5 * drag_coef * ambient_density.get::<uom::si::mass_density::kilogram_per_cubic_meter>() * frontal_area * velocity_mag * velocity.0.normalize()
        } else {
            Vec3::ZERO
        };

        // Apply the drag force
        external_force.apply_force(drag_force);
    }
}
