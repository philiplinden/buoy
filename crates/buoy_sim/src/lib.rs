pub mod map;
pub mod prelude {
    pub use buoy_common::*;
    pub use crate::BuoySimulationPlugins;
}

use bevy::prelude::*;

#[derive(Default)]
pub struct BuoySimulationPlugins;

impl Plugin for BuoySimulationPlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins(map::plugin);
    }
}
