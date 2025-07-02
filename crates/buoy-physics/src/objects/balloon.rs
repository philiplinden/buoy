use bevy::prelude::*;

#[derive(Component, Default)]
#[require(Transform)]
pub struct Balloon;

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
}

#[derive(Bundle)]
pub struct BalloonBundle {
    name: Name,
    balloon: Balloon,
    transform: Transform,
    collider: Collider,
    rigid_body: RigidBody,
}
