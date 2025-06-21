use std::fmt::Display;

use avian3d::prelude::{PhysicsInterpolationPlugin, PhysicsPlugins, PhysicsSet};
use bevy::{
    app::{PluginGroup, PluginGroupBuilder},
    prelude::*,
};
use uom::si::{f32::*, Quantity};

pub struct BuoyPhysicsPlugin;

impl Plugin for BuoyPhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            PhysicsPlugins::default().set(PhysicsInterpolationPlugin::interpolate_all()),
            super::ideal_gas::plugin,
            super::atmosphere::plugin,
            super::forces::plugin,
            super::time::plugin,
            #[cfg(feature = "grid_space")]
            super::fluid_volume::plugin,
            #[cfg(feature = "grid_space")]
            super::grid::plugin,
        ));
    }
}
