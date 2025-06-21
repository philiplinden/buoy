//! This module is temporary. It sets up a basic scene with a fluid volume and
//! some other objects to test simulation features.
//!
//! It should be replaced with a system that allows a scene to be loaded from a
//! config file or spawned at runtime.

use bevy::prelude::*;

#[cfg(feature = "grid_space")]
use buoy_physics::{
    fluid_volume::FluidVolumeBuilder,
    grid::{Precision, RootGrid},
};

pub(crate) fn plugin(app: &mut App) {
    app.add_systems(PostStartup, setup_scene);

    #[cfg(feature = "grid_space")]
    app.add_systems(PostStartup, spawn_fluid_volume);
}

#[derive(Component)]
struct DebugObject;

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(5.0, 5.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
    ));
    commands.spawn((
        PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..Default::default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
}

#[cfg(feature = "grid_space")]
fn spawn_fluid_volume(mut commands: Commands, root_grid: Query<Entity, With<RootGrid>>) {
    let builder = FluidVolumeBuilder::new();
    builder.spawn(&mut commands, root_grid.single());
}
