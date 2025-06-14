use bevy::prelude::*;

pub struct PausableSystemsPlugin;

impl Plugin for PausableSystemsPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<RuntimeState>();
        app.configure_sets(Update, PausableSystems.run_if(in_state(RuntimeState::Running)));
    }
}


#[derive(States, Debug, Default, Clone, Copy, Hash, PartialEq, Eq)]
pub enum RuntimeState {
    #[default]
    Initializing,
    Running,
    Stopped,
    Paused,
    HardwareWaiting,
    Faulted,
}

/// A system set for systems that shouldn't run while the game is paused.
#[derive(SystemSet, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct PausableSystems;