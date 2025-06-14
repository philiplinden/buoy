//! Forces applied to rigid bodies.
use avian3d::math::Scalar;
use bevy::prelude::*;
use uom::si::{
    acceleration::meter_per_second_squared, area::square_meter, f32::*, length::meter,
    mass_density::kilogram_per_cubic_meter, ratio::ratio, volume::cubic_meter,
};

use crate::constants::EARTH_RADIUS_M;

/// Fraction of standard gravity at an altitude (m) above mean sea level.
pub fn scale_gravity(altitude_meters: Scalar) -> Scalar {
    let scale = *EARTH_RADIUS_M / (*EARTH_RADIUS_M + Length::new::<meter>(altitude_meters));
    scale.get::<ratio>()
}

/// Force (N) due to drag as a solid body moves through a fluid.
pub fn drag(
    velocity: Vec3,
    ambient_density: MassDensity,
    drag_area: Area,
    drag_coefficient: Scalar,
) -> Vec3 {
    -0.5 * drag_coefficient
        * ambient_density.get::<kilogram_per_cubic_meter>()
        * drag_area.get::<square_meter>()
        * velocity.length()
        * velocity
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
