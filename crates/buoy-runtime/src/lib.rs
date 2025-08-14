pub mod format;
pub mod sequencing;
pub mod objects;
pub mod config;

use bevy::{app::PluginGroupBuilder, prelude::*};

/// A custom flavor of Bevy's DefaultPlugins that includes common plugins used by Buoy.
pub struct BuoyDefaultPlugins;

impl PluginGroup for BuoyDefaultPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(RuntimePlugin)
            .add(sequencing::plugin)
            .add(format::PrettyPrintPlugin)
            .add_group(bevy_repl::prelude::ReplPlugins)
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
