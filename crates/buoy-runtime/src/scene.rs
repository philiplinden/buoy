//! This module is temporary. It sets up a basic scene with a fluid volume and
//! some other objects to test simulation features.
//!
//! It should be replaced with a system that allows a scene to be loaded from a
//! config file or spawned at runtime.

use bevy::prelude::*;
use buoy_common::objects::balloon::spawn_balloon;

pub struct SimpleScenePlugin;

impl Plugin for SimpleScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, setup_scene);
    }
}

fn setup_scene(mut commands: Commands) {
    spawn_balloon(&mut commands);
}
