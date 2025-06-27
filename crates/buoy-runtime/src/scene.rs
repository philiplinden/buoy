//! This module is temporary. It sets up a basic scene with a fluid volume and
//! some other objects to test simulation features.
//!
//! It should be replaced with a system that allows a scene to be loaded from a
//! config file or spawned at runtime.

use bevy::prelude::*;
use avian3d::prelude::*;
use buoy_physics::objects::*;
// use buoy_physics::mesh_utils::{create_icosphere_mesh, Strain, RestState};

pub struct SimpleScenePlugin;

impl Plugin for SimpleScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, setup_scene);
        app.add_systems(FixedUpdate, environment::update_ground_plane_collider);
        app.add_systems(FixedUpdate, log_rigid_body_changes);
    }
}

fn setup_scene(
    mut commands: Commands,
) {
    // FIXME: for some reason many subdivisions end up being super slow
    // let balloon_mesh = create_icosphere_mesh(1.0, 0);
    commands.spawn(Balloon::new());
    commands.spawn(GroundPlane::new((10.0, 10.0)));
}

fn log_rigid_body_changes(
    mut query: Query<(Entity, &Transform), (With<RigidBody>, Changed<Transform>)>,
) {
    for (entity, transform) in query.iter_mut() {
        info!("Entity: {:?} moved to: {:?}", entity, transform);
    }
}
