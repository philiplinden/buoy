use std::fmt::Display;

use super::*;
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
            ideal_gas::plugin,
            atmosphere::plugin,
            forces::plugin,
            time::plugin,
            #[cfg(feature = "grid_space")]
            fluid_volume::plugin,
            #[cfg(feature = "grid_space")]
            grid::plugin,
        ));
    }
}
