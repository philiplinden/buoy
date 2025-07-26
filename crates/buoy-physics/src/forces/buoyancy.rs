//! Forces applied to rigid bodies.
use avian3d::{math::Scalar, prelude::*};
use bevy::prelude::*;
use uom::si::{
    acceleration::meter_per_second_squared, area::square_meter, f32::*,
    mass_density::kilogram_per_cubic_meter, volume::cubic_meter,
};

pub(crate) fn plugin(_app: &mut App) {}

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
