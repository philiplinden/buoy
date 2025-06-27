mod camera;
mod lighting;
mod shell;
mod colors;

#[cfg(feature = "dev")]
mod debug;

use bevy::prelude::*;

pub struct BuoyUiPlugin;

impl Plugin for BuoyUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            shell::plugin,
            camera::plugin,
            lighting::plugin,
        ));

        #[cfg(feature = "dev")]
        app.add_plugins(debug::plugin);
    }
}
