use bevy::prelude::*;
use tracing;
use tracing_subscriber::prelude::*;

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

/// Plugin that initializes tracing
pub struct TracingPlugin;

impl Plugin for TracingPlugin {
    fn build(&self, app: &mut App) {
        // Initialize tracing if not already initialized
        if !app.world().contains_resource::<TracingDispatch>() {
            let dispatch = init_tracing();
            app.insert_resource(dispatch);
        }
    }
}

/// Wrapper for tracing Dispatch to implement Bevy's Resource trait
#[derive(Resource)]
pub struct TracingDispatch(tracing::Dispatch);

/// Initialize tracing with Bevy's logging system
pub fn init_tracing() -> TracingDispatch {
    let filter = tracing_subscriber::EnvFilter::from_default_env()
        .add_directive("buoy_common=info".parse().unwrap());

    let registry = tracing_subscriber::registry()
        .with(filter)
        .with(tracing_subscriber::fmt::layer());

    TracingDispatch(registry.into())
}
