use bevy::prelude::*;
use avian3d::prelude::*;
use serde::Deserialize;
use bevy::asset::Asset;
use bevy::reflect::TypePath;

#[derive(Component, Default)]
#[require(Transform)]
pub struct Balloon;

#[derive(Deserialize, Debug, Asset, TypePath)]
pub struct BalloonConfig {
    pub balloon_size: f32, // meters (radius)
    pub lift_gas_species: String,
    pub lift_gas_mass: f32, // kg
    pub balloon_mass: f32, // kg
    pub payload_mass: f32, // kg
}

impl Balloon {
    pub fn new() -> BalloonBundle {
        let balloon = Balloon;
        let radius = 1.0;
        BalloonBundle {
            name: Name::new("Balloon"),
            balloon,
            transform: Transform::from_xyz(0.0, radius*2.0, 0.0),
            collider: Collider::sphere(radius),
            rigid_body: RigidBody::Dynamic,
        }
    }
    pub fn new_from_config(config: &BalloonConfig) -> BalloonBundle {
        let balloon = Balloon;
        let radius = config.balloon_size;
        BalloonBundle {
            name: Name::new("Balloon"),
            balloon,
            transform: Transform::from_xyz(0.0, radius*2.0, 0.0),
            collider: Collider::sphere(radius),
            rigid_body: RigidBody::Dynamic,
        }
    }
}

#[derive(Bundle)]
pub struct BalloonBundle {
    name: Name,
    balloon: Balloon,
    transform: Transform,
    collider: Collider,
    rigid_body: RigidBody,
}
