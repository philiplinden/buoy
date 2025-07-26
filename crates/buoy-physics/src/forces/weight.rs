//! Forces applied to rigid bodies.
use avian3d::{math::Scalar, prelude::*};
use bevy::prelude::*;
use uom::si::{acceleration::meter_per_second_squared, f32::*, length::meter, ratio::ratio};

use crate::constants::{EARTH_RADIUS_M, STANDARD_GRAVITY};

pub(crate) fn plugin(app: &mut App) {
    app.insert_resource(Gravity(
        Vec3::NEG_Y * STANDARD_GRAVITY.get::<meter_per_second_squared>(),
    ));
    app.add_systems(FixedUpdate, (update_gravity).in_set(PhysicsStepSet::First));
}

/// Fraction of standard gravity at an altitude (m) above mean sea level.
pub fn scale_gravity(altitude_meters: Scalar) -> Scalar {
    let scale = *EARTH_RADIUS_M / (*EARTH_RADIUS_M + Length::new::<meter>(altitude_meters));
    scale.get::<ratio>()
}

fn update_gravity(mut query: Query<(&mut GravityScale, &Position)>) {
    for (mut gravity_scale, position) in query.iter_mut() {
        gravity_scale.0 = scale_gravity(position.y);
    }
}
