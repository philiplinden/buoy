use bevy::prelude::*;
use buoy::BuoyPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins,
            BuoyPlugin,
        ))
        .run();
}
