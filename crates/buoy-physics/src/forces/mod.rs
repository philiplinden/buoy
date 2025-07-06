pub mod buoyancy;
pub mod drag;
pub mod weight;

use bevy::prelude::*;

pub(crate) fn plugin(app: &mut App) {
    app.add_plugins((
        buoyancy::plugin,
        drag::plugin,
        weight::plugin,
    ));
}
