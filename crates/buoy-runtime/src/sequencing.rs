use avian3d::prelude::*;
use bevy::prelude::*;

use crate::RuntimeState;

pub(crate) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(RuntimeState::Stopped), pause);
    app.add_systems(OnExit(RuntimeState::Stopped), unpause);

}

pub fn pause(mut physics_time: ResMut<Time<Physics>>, mut next_state: ResMut<NextState<RuntimeState>>) {
    physics_time.as_mut().pause();
    debug!("pausing physics time");
    next_state.set(RuntimeState::Stopped);
}

pub fn unpause(
    mut physics_time: ResMut<Time<Physics>>,
    mut next_state: ResMut<NextState<RuntimeState>>,
) {
    physics_time.as_mut().unpause();
    debug!("unpausing physics time");
    next_state.set(RuntimeState::Running);
}

// /// TODO: Split the runtime into system sets to keep physics and rendering consistent.
// #[derive(SystemSet, Default, Clone, Copy, Hash, PartialEq, Eq)]
// pub enum RuntimeSequenceSet {
//     Physics, // This includes the entire physics pipeline
//     Render, // Updates the renderer to match the current state of the scene
//     Gui, // Updates the UI and processes inputs
// }
