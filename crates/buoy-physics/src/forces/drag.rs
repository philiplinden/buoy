//! Forces applied to rigid bodies.
use avian3d::{math::Scalar, prelude::*};
use bevy::prelude::*;
use uom::si::{area::square_meter, f32::*, mass_density::kilogram_per_cubic_meter};

pub(crate) fn plugin(_app: &mut App) {}

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
