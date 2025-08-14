//! Simulation pipeline organization and system sets
//!
//! This module provides a structured approach to organizing the simulation pipeline
//! using Bevy system sets. The `SimulationPipeline` enum defines the execution order
//! and dependencies between different parts of the simulation.
//!
//! ## Pipeline Stages
//!
//! 1. **Input** - User input, events, and command parsing
//! 2. **Physics** - Physics simulation and calculations
//! 4. **Render** - Scene rendering and visual updates
//! 5. **Ui** - UI rendering and interaction processing
//!
//! ## Usage
//!
//! ```rust
//! use crate::sequencing::SimulationPipeline;
//!
//! // Add a system to a specific pipeline stage
//! app.add_systems(Update, my_system.in_set(SimulationPipeline::Physics));
//! ```

use avian3d::prelude::*;
use bevy::prelude::*;

use crate::RuntimeState;

pub(crate) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(RuntimeState::Stopped), pause);
    app.add_systems(OnExit(RuntimeState::Stopped), unpause);

    // Configure system sets for different runtime states
    app.configure_sets(Update, (
        SimulationPipeline::Input,
        SimulationPipeline::Physics,
        SimulationPipeline::Render,
        SimulationPipeline::Ui,
    ).chain());

    // Configure conditional system sets based on runtime state
    app.configure_sets(Update,
        SimulationPipeline::Physics
            .run_if(in_state(RuntimeState::Running))
    );
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

/// System sets to organize the simulation pipeline
#[derive(SystemSet, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum SimulationPipeline {
    /// Input processing - handles user input, events, and command parsing
    Input,

    /// Physics simulation - includes the entire physics pipeline
    Physics,

    /// Rendering - updates the renderer to match the current state of the scene
    Render,

    /// GUI updates - UI rendering and interaction processing
    Ui,
}
