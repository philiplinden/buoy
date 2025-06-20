mod atmosphere;
mod constants;
mod forces;
mod geometry;
mod ideal_gas;
mod time;

use avian3d::prelude::{PhysicsInterpolationPlugin, PhysicsPlugins};
use bevy::prelude::*;

pub(crate) fn plugin(app: &mut App) {
    app.add_plugins((
        PhysicsPlugins::default().set(PhysicsInterpolationPlugin::interpolate_all()),
        ideal_gas::plugin,
        atmosphere::plugin,
        forces::plugin,
        time::plugin,
    ));
}
