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

#[inline]
fn calculate_drag_force(
    velocity: Vec3,
    ambient_density: f32,
    frontal_area: f32,
    drag_coef: f32,
) -> Vec3 {
    let velocity_mag = velocity.length();
    if velocity_mag <= 0.001 {
        return Vec3::ZERO;
    }
    
    let force_magnitude = -0.5 * drag_coef * ambient_density * frontal_area * velocity_mag;
    velocity.normalize() * force_magnitude
}

#[inline]
fn calculate_reynolds(velocity: f32, radius: f32, density: MassDensity) -> f32 {
    velocity * radius * 2.0 * density.get::<uom::si::mass_density::kilogram_per_cubic_meter>() / 1.81e-5
}

#[inline]
fn calculate_drag_coefficient(reynolds: f32, drag_calc: &MeshDragCalculator) -> f32 {
    if reynolds < 1e5 {
        drag_calc.drag_coefficient_base * (1.0 + drag_calc.reynolds_scaling)
    } else {
        drag_calc.drag_coefficient_base * (1.0 - drag_calc.reynolds_scaling)
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
    // Collect all calculations first
    let mut forces: Vec<(Entity, Vec3)> = Vec::with_capacity(query.iter().len());
    
    for (entity, (_, drag_calc, physics, velocity, position)) in query.iter() {
        let position_vec = Vec3::new(position.x, position.y, position.z);
        let ambient_density = atmosphere.density(position_vec);
        
        let frontal_area = std::f32::consts::PI * physics.radius.powi(2);
        let reynolds = calculate_reynolds(velocity.0.length(), physics.radius, ambient_density);
        let drag_coef = calculate_drag_coefficient(reynolds, drag_calc);
        
        let force = calculate_drag_force(
            velocity.0,
            ambient_density.get::<uom::si::mass_density::kilogram_per_cubic_meter>(),
            frontal_area,
            drag_coef,
        );
        
        forces.push((entity, force));
    }
    
    // Apply forces in batch
    for (entity, force) in forces {
        if let Ok((mut external_force, _, _, _, _)) = query.get_mut(entity) {
            external_force.apply_force(force);
        }
    }
}
