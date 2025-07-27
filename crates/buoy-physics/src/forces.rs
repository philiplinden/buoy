use avian3d::{math::Scalar, prelude::*};
use bevy::prelude::*;
use uom::si::{
    acceleration::meter_per_second_squared, area::square_meter, f32::*, length::meter,
    mass::kilogram, mass_density::kilogram_per_cubic_meter, ratio::ratio, volume::cubic_meter,
    velocity::meter_per_second,
};

use crate::atmosphere::Atmosphere;
use crate::constants::{EARTH_RADIUS_M, STANDARD_GRAVITY};
use crate::geometry::{projected_area, collider_volume};

pub(crate) fn plugin(app: &mut App) {
    app.insert_resource(Gravity(Vec3::ZERO));
    app.add_systems(FixedUpdate, (net_force).in_set(PhysicsStepSet::First));
}

fn net_force(
    mut query: Query<(
        &mut ExternalForce,
        &Position,
        &ComputedMass,
        &Collider,
        &LinearVelocity,
        &DragCoefficient,
    )>,
    atmosphere: Res<Atmosphere>,
) {
    for (mut external_force, position, computed_mass, collider, velocity, drag_coefficient) in
        query.iter_mut()
    {
        let mass = uom::si::f32::Mass::new::<kilogram>(computed_mass.value());
        let drag_area =
            uom::si::f32::Area::new::<square_meter>(projected_area(collider, velocity.0));
        let gravity = local_gravity(uom::si::f32::Length::new::<meter>(position.y));
        let weight = weight(mass, gravity);
        let buoyancy = buoyancy(gravity, uom::si::f32::Volume::new::<cubic_meter>(collider_volume(collider)), atmosphere.density(position.0));
        let drag = drag(
            velocity.0,
            atmosphere.density(position.0),
            drag_area,
            drag_coefficient.0,
        );
        let net_force = weight + buoyancy + drag;

        external_force.apply_force(net_force);
    }
}

/// Force (N) due to drag as a solid body moves through a fluid.
pub fn drag(
    velocity: Vec3,
    ambient_density: MassDensity,
    drag_area: Area,
    drag_coefficient: Scalar,
) -> Vec3 {
    let velocity_magnitude = velocity.length();
    if velocity_magnitude < f32::EPSILON {
        return Vec3::ZERO;
    }

    -0.5 * drag_coefficient
        * ambient_density.get::<kilogram_per_cubic_meter>()
        * drag_area.get::<square_meter>()
        * velocity_magnitude * velocity_magnitude
        * velocity / velocity_magnitude
}

/// Upward force (N) vector due to atmosphere displaced by the given gas volume.
/// The direction of this force is always world-space up (it opposes gravity).
pub fn buoyancy(
    gravity_acceleration: Acceleration,
    displaced_volume: Volume,
    ambient_density: MassDensity,
) -> Vec3 {
    Vec3::Y
        * (displaced_volume.get::<cubic_meter>()
            * ambient_density.get::<kilogram_per_cubic_meter>()
            * gravity_acceleration.get::<meter_per_second_squared>())
}

/// Fraction of standard gravity at an altitude (m) above mean sea level.
pub fn local_gravity(altitude: Length) -> Acceleration {
    *EARTH_RADIUS_M / (*EARTH_RADIUS_M + altitude) * *STANDARD_GRAVITY
}

fn weight(mass: uom::si::f32::Mass, gravity: Acceleration) -> Vec3 {
    Vec3::NEG_Y * mass.get::<kilogram>() * gravity.get::<meter_per_second_squared>()
}

#[derive(Component, Default)]
pub struct DragCoefficient(pub Scalar);
