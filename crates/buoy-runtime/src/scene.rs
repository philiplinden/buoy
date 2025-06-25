//! This module is temporary. It sets up a basic scene with a fluid volume and
//! some other objects to test simulation features.
//!
//! It should be replaced with a system that allows a scene to be loaded from a
//! config file or spawned at runtime.

use bevy::prelude::*;
use buoy_common::objects::balloon::spawn_balloon;

#[cfg(feature = "grid_space")]
use buoy_physics::{
    fluid_volume::FluidVolumeBuilder,
    grid::{Precision, RootGrid},
};

pub struct SimpleScenePlugin;

impl Plugin for SimpleScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, setup_scene);

        #[cfg(feature = "grid_space")]
        app.add_systems(PostStartup, spawn_fluid_volume);
    }
}

fn setup_scene(mut commands: Commands) {
    spawn_balloon(&mut commands);
}

#[cfg(feature = "grid_space")]
fn spawn_fluid_volume(mut commands: Commands, root_grid: Query<Entity, With<RootGrid>>) {
    let builder = FluidVolumeBuilder::new();
    builder.spawn(&mut commands, root_grid.single());
}
