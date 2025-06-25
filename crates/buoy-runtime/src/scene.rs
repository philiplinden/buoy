//! This module is temporary. It sets up a basic scene with a fluid volume and
//! some other objects to test simulation features.
//!
//! It should be replaced with a system that allows a scene to be loaded from a
//! config file or spawned at runtime.

use bevy::prelude::*;
use buoy_physics::objects::balloon::Balloon;
use buoy_physics::mesh_utils::{create_icosphere_mesh, Strain, RestState};

pub struct SimpleScenePlugin;

impl Plugin for SimpleScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, setup_scene);
    }
}


#[cfg(feature = "gui")]
fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut asset_server: Res<AssetServer>,
) {
    // FIXME: for some reason many subdivisions end up being super slow
    let balloon_mesh = create_icosphere_mesh(1.0, 0);
    commands.spawn((
        Balloon,
        Transform::from_xyz(0.0, 0.5, 0.0),
        Strain::default(),
        RestState::from_mesh(&balloon_mesh),
        Mesh3d(meshes.add(balloon_mesh)),
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.1, 0.2))),
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


#[cfg(not(feature = "gui"))]
fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut asset_server: Res<AssetServer>,
) {
    // FIXME: for some reason many subdivisions end up being super slow
    let balloon_mesh = create_icosphere_mesh(1.0, 0);
    commands.spawn((
        Balloon,
        Transform::from_xyz(0.0, 0.5, 0.0),
        Strain::default(),
        RestState::from_mesh(&balloon_mesh),
        Mesh3d(meshes.add(balloon_mesh)),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(5.0, 5.0))),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}
