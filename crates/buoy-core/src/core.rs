use std::fmt::Display;

use super::*;
use avian3d::prelude::{PhysicsInterpolationPlugin, PhysicsPlugins, PhysicsSet};
use bevy::{
    app::{PluginGroup, PluginGroupBuilder},
    prelude::*,
};
use uom::si::{f32::*, Quantity};

pub struct BuoyPlugin;

impl Plugin for BuoyPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            CoreSystemsPlugin,
            CorePhysicsPlugin,
        ));
    }
}

struct CorePhysicsPlugin;

impl Plugin for CorePhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            PhysicsPlugins::default().set(PhysicsInterpolationPlugin::interpolate_all()),
            ideal_gas::plugin,
            atmosphere::plugin,
            forces::plugin,
            balloon::plugin,
            mesh_drag::plugin,
        ));
    }
}

/// The plugin that handles the overall simulation state.
struct CoreSystemsPlugin;

impl Plugin for CoreSystemsPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<SimState>();
        app.add_plugins((
            format::plugin,
        ));

        // Set up the `Pause` state.
        app.init_state::<Pause>();
        app.configure_sets(Update, PausableSystems.run_if(in_state(Pause(false))));
    }
}

#[derive(States, Debug, Default, Clone, Copy, Hash, PartialEq, Eq)]
pub enum SimState {
    #[default]
    Initializing,
    Running,
    Stopped,
    Paused,
    HardwareWaiting,
    Faulted,
}

/// Whether or not the game is paused.
#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
#[states(scoped_entities)]
struct Pause(pub bool);

/// A system set for systems that shouldn't run while the game is paused.
#[derive(SystemSet, Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct PausableSystems;
