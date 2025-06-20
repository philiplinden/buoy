pub mod physics;
pub mod ui;
pub mod scenario;

use bevy::prelude::*;


/// The plugin that handles the overall simulation state.
pub struct BuoyPlugin;

impl Plugin for BuoyPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<SimState>();
        app.add_plugins((
            #[cfg(feature = "render")]
            ui::plugin,
            physics::plugin,
            scenario::plugin,
        ));
    }
}

#[derive(States, Debug, Default, Clone, Copy, Hash, PartialEq, Eq)]
pub enum SimState {
    Stopped,
    #[default]
    Running,
    Faulted,
}
