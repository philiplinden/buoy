pub mod format;
// pub mod sequencing;
// pub mod objects;
// pub mod config;
// pub mod collider;

use bevy::{app::PluginGroupBuilder, prelude::*};
use bevy_repl::prelude::*;

/// A custom flavor of Bevy's DefaultPlugins that includes common plugins used by Buoy.
pub struct BuoyDefaultPlugins;

impl PluginGroup for BuoyDefaultPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(RuntimePlugin)
            .add(format::PrettyPrintPlugin)
            .add_group(ReplPlugins)
    }
}

struct RuntimePlugin;

impl Plugin for RuntimePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<RuntimeState>();
    }
}

#[derive(States, Debug, Default, Clone, Copy, Hash, PartialEq, Eq)]
pub enum RuntimeState {
    Stopped,
    #[default]
    Running,
    Faulted,
}

// pub struct BuoyPhysicsPlugin;

// impl Plugin for BuoyPhysicsPlugin {
//     fn build(&self, app: &mut App) {
//         app.add_plugins((
//             PhysicsPlugins::default(),
//             buoy_physics::atmosphere::plugin,
//             buoy_physics::ideal_gas::plugin,
//             buoy_physics::forces::plugin,
//         ));
//     }
// }
