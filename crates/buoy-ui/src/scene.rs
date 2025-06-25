//! This module is temporary. It sets up a basic scene with a fluid volume and
//! some other objects to test simulation features.
//!
//! It should be replaced with a system that allows a scene to be loaded from a
//! config file or spawned at runtime.

use bevy::prelude::*;

use buoy_physics::mesh_utils::{create_icosphere_mesh, MeshHandle, RestState, Strain};

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
    // Create the mesh
    let mesh = create_icosphere_mesh(1.0, 2);

    // Store the rest state
    let rest_state = RestState::from_mesh(&mesh);

    // Add the mesh to the asset server and get a handle
    let mesh_handle = meshes.add(mesh);

    // Spawn the mesh entity with its rest state and strain tracking
    commands.spawn((
        Mesh3d(mesh_handle.clone()),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.8, 0.7, 0.6),
            ..default()
        })),
        MeshHandle(mesh_handle),
        rest_state,
        Strain::default(), // Initialize with zero strain
        Transform::from_xyz(0.0, 1.0, 0.0),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(5.0, 5.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
        Transform::from_xyz(0.0, 0.0, 0.0),
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
