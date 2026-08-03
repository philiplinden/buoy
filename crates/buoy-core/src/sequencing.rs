//! Simulation pipeline organization and system sets
//!
//! This module provides a structured approach to organizing the simulation pipeline
//! using Bevy system sets. The `AppState` enum defines the execution order
//! and dependencies between different parts of the simulation.

use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_repl::prelude::*;

#[derive(States, Debug, Default, Clone, Copy, Hash, PartialEq, Eq)]
pub enum AppState {
    #[default]
    Setup,
    Configured,
    PhysicsPaused,
    PhysicsRunning,
    Shutdown,
}

pub(crate) fn plugin(app: &mut App) {
    app.init_state::<AppState>();
    app.add_plugins((AppSetupPlugin, AppRunningPlugin));
}

struct AppSetupPlugin;

impl Plugin for AppSetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, app_setup);
        app.add_systems(Update, app_post_configuration.run_if(in_state(AppState::Configured)));
    }
}

fn app_setup(mut app_state: ResMut<NextState<AppState>>) {
    info!("setting up...");
    app_state.set(AppState::Setup)
}

fn app_post_configuration(
    mut app_state: ResMut<NextState<AppState>>,
) {
    info!("app configured. ready to run physics");
    app_state.set(AppState::PhysicsPaused);
}

struct AppRunningPlugin;

impl Plugin for AppRunningPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (
                // Make sure the REPL commands run before physics
                ReplSet::All.before(PhysicsSet::Prepare),
                // Only step the simulation forward if the simulation is running
                PhysicsSet::StepSimulation.run_if(in_state(AppState::PhysicsRunning)),
            ),
        );

        // Add pause and unpause triggers
        app.add_event::<PauseEvent>();
        app.add_observer(pause);
        app.add_event::<UnpauseEvent>();
        app.add_observer(unpause);
    }
}

/// Triggered to pause physics time
#[derive(Event)]
pub struct PauseEvent;

/// Pauses physics time
fn pause(
    _trigger: Trigger<PauseEvent>,
    mut physics_time: ResMut<Time<Physics>>,
    app_state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    physics_time.as_mut().pause();
    match app_state.get() {
        AppState::PhysicsRunning => {
            next_state.set(AppState::PhysicsPaused);
            debug!("pausing physics time");
        },
        AppState::PhysicsPaused => {
            debug!("tried to pause physics time, but physics is already paused");
        },
        _ => {
            warn!("tried to pause physics time, but physics is not running");
        }
    }
}

/// Triggered to unpause physics time
#[derive(Event)]
pub struct UnpauseEvent;

/// Unpauses physics time
fn unpause(
    _trigger: Trigger<UnpauseEvent>,
    mut physics_time: ResMut<Time<Physics>>,
    app_state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    physics_time.as_mut().unpause();
    match app_state.get() {
        AppState::PhysicsPaused => {
            next_state.set(AppState::PhysicsRunning);
            debug!("unpausing physics time");
        },
        AppState::PhysicsRunning => {
            debug!("tried to unpause physics time, but physics is already running");
        },
        _ => {
            warn!("tried to unpause physics time, but physics is not set up");
        }
    }
}
