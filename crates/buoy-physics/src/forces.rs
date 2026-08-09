use glam::Vec3;
#[cfg(feature = "bevy")]
use bevy::prelude::{App, Component, FixedUpdate, Query, Res, Time, Transform};
use uom::si::{
    acceleration::meter_per_second_squared,
    area::square_meter,
    f32::{Acceleration, Area, Length, MassDensity, Volume},
    mass_density::kilogram_per_cubic_meter,
    volume::cubic_meter,
};
#[cfg(feature = "bevy")]
use uom::si::length::meter;

#[cfg(feature = "bevy")]
use crate::atmosphere::Atmosphere;
use crate::constants::{EARTH_RADIUS_M, STANDARD_GRAVITY};
#[cfg(feature = "bevy")]
use crate::geometry::Shape;

#[cfg(feature = "bevy")]
pub(crate) fn plugin(app: &mut App) {
    app.add_systems(FixedUpdate, integrate);
}

/// Linear velocity (m/s), integrated from net force each fixed tick.
#[cfg_attr(feature = "bevy", derive(Component))]
#[derive(Default)]
pub struct Velocity(pub Vec3);

/// Mass (kg) of a body, used to convert net force into acceleration.
#[cfg_attr(feature = "bevy", derive(Component))]
pub struct Mass(pub f32);

/// Integrates net force (weight + buoyancy + drag) into velocity and
/// position using semi-implicit Euler.
#[cfg(feature = "bevy")]
fn integrate(
    time: Res<Time>,
    mut bodies: Query<(&mut Transform, &mut Velocity, &Mass, &Shape, &DragCoefficient)>,
    atmosphere: Res<Atmosphere>,
) {
    let dt = time.delta_secs();
    for (mut transform, mut velocity, mass, shape, drag_coefficient) in &mut bodies {
        let position = transform.translation;
        let drag_area = Area::new::<square_meter>(shape.projected_area(velocity.0));
        let gravity = local_gravity(Length::new::<meter>(position.y));
        let weight = weight(mass.0, gravity);
        let buoyancy = buoyancy(
            gravity,
            Volume::new::<cubic_meter>(shape.volume()),
            atmosphere.density(position),
        );
        let drag = drag(
            velocity.0,
            atmosphere.density(position),
            drag_area,
            drag_coefficient.0,
        );
        let net_force = weight + buoyancy + drag;

        let acceleration = net_force / mass.0;
        velocity.0 += acceleration * dt;
        transform.translation += velocity.0 * dt;
    }
}

/// Force (N) due to drag as a solid body moves through a fluid.
pub fn drag(
    velocity: Vec3,
    ambient_density: MassDensity,
    drag_area: Area,
    drag_coefficient: f32,
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

fn weight(mass_kg: f32, gravity: Acceleration) -> Vec3 {
    Vec3::NEG_Y * mass_kg * gravity.get::<meter_per_second_squared>()
}

#[cfg_attr(feature = "bevy", derive(Component))]
#[derive(Default)]
pub struct DragCoefficient(pub f32);
