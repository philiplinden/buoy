use bevy::prelude::*;
use bevy_repl::prelude::*;
use clap::Parser;

use crate::sequencing::AppState;

pub(super) fn plugin(app: &mut App) {
    app.add_repl_command::<ForceConfigured>();
    app.add_observer(force_configured);
    app.add_repl_command::<Pause>();
    app.add_observer(pause);
    app.add_repl_command::<Unpause>();
    app.add_observer(unpause);
}

#[derive(Parser, ReplCommand, Event, Clone, Debug, Default)]
#[command(name = "configured")]
pub struct ForceConfigured;

fn force_configured(
    _trigger: Trigger<ForceConfigured>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    app_state.set(AppState::Configured);
}

#[derive(Parser, ReplCommand, Event, Clone, Debug, Default)]
#[command(name = "pause")]
pub struct Pause;

fn pause(
    _trigger: Trigger<Pause>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    app_state.set(AppState::PhysicsPaused);
}

#[derive(Parser, ReplCommand, Event, Clone, Debug, Default)]
#[command(name = "unpause")]
pub struct Unpause;

fn unpause(
    _trigger: Trigger<Unpause>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    app_state.set(AppState::PhysicsRunning);
}
