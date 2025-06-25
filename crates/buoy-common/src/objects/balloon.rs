use bevy::prelude::*;

#[derive(Component, Default)]
#[require(Transform)]
pub struct Balloon {}

// Builder for the PlayerBundle
struct BalloonBuilder {
    position: Vec3,
}

impl Default for BalloonBuilder {
    fn default() -> Self {
        Self {
            position: Vec3::ZERO,
        }
    }
}

impl BalloonBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_position(mut self, position: Vec3) -> Self {
        self.position = position;
        self
    }

    pub fn spawn(self, commands: &mut Commands) {
        debug!("Spawning balloon at {:?}", self.position);
        commands.spawn((
            Balloon::default(),
            Transform::from_translation(self.position),
        ));
    }
}

pub fn spawn_balloon(commands: &mut Commands) {
    BalloonBuilder::new()
        .with_position(Vec3::new(100.0, 50.0, 0.0))
        .spawn(commands);
}
