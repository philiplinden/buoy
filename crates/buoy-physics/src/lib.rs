pub mod atmosphere;
pub mod config;
pub mod constants;
pub mod format;
pub mod geometry;
pub mod ideal_gas;
pub mod forces;
pub mod objects;
pub mod sequencing;
pub mod prelude {
    pub use crate::{
        atmosphere::Atmosphere,
        forces::{Mass, Velocity},
        geometry::Shape,
        ideal_gas::{GasSpecies, IdealGas},
    };
}

use bevy::{
    app::{PluginGroup, PluginGroupBuilder},
    prelude::*,
    time::TimePlugin,
};

pub struct BuoyPhysicsPlugin;

impl Plugin for BuoyPhysicsPlugin {
    fn build(&self, app: &mut App) {
        // `forces::plugin` runs in FixedUpdate, which needs `TimePlugin` for
        // its `Time<Fixed>`/`Time<Virtual>` resources and the fixed-schedule
        // runner. We used to get this for free from Avian's `PhysicsPlugins`;
        // now we own the requirement directly.
        if !app.is_plugin_added::<TimePlugin>() {
            app.add_plugins(TimePlugin);
        }
        app.add_plugins((atmosphere::plugin, ideal_gas::plugin, forces::plugin));
    }
}

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
