use bevy::prelude::*;

pub struct RuntimeControllerPlugin;

impl Plugin for RuntimeControllerPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<RuntimeState>();

        app.configure_sets(Update, PausableSystems.run_if(in_state(RuntimeState::Running)));
        app.add_systems(
            FixedUpdate,
            (update_gravity)
                .chain()
                .in_set(PhysicsStepSet::First)
                .in_set(PausableSystems),
        );
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