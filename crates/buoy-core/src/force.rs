use glam::Vec3;
use uom::si::{
    acceleration::meter_per_second_squared, area::square_meter, f32::*, length::meter,
    mass::kilogram, mass_density::kilogram_per_cubic_meter, ratio::ratio, volume::cubic_meter,
    velocity::meter_per_second,
};

use crate::constants::{EARTH_RADIUS_M, STANDARD_GRAVITY};

/// Force (N) due to drag as a solid body moves through a fluid.
pub fn drag(
    velocity: Vec3,
    ambient_density: MassDensity,
    drag_area: Area,
    drag_coefficient: f32,
) -> Vec3 {
    let velocity_magnitude: f32 = velocity.length();
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

#[allow(dead_code)]
pub fn weight(mass: uom::si::f32::Mass, gravity: Acceleration) -> Vec3 {
    Vec3::NEG_Y * mass.get::<kilogram>() * gravity.get::<meter_per_second_squared>()
}
