//! This module is temporary. It sets up a basic scene with a fluid volume and
//! some other objects to test simulation features.
//!
//! It should be replaced with a system that allows a scene to be loaded from a
//! config file or spawned at runtime.

use bevy::prelude::*;

use buoy_core::{
    fluid_volume::FluidVolumeBuilder,
    grid::RootGrid,
};

pub(crate) fn plugin(app: &mut App) {
    app.add_systems(PostStartup, (setup_scene, spawn_fluid_volume));
}

#[derive(Component)]
struct DebugObject;

fn setup_scene(mut commands: Commands) {
    commands.spawn((
        DebugObject,
        Transform::default(),
        Name::new("Debug Object"),
    ));
}

fn spawn_fluid_volume(mut commands: Commands, root_grid: Query<Entity, With<RootGrid>>) {
    let builder = FluidVolumeBuilder::new();
    builder.spawn(&mut commands, root_grid.single());
}
