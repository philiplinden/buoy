mod camera;
mod controls;
mod lighting;
mod shell;

#[cfg(feature = "dev")]
mod debug;

use bevy::prelude::*;

pub struct BuoyUiPlugin;

impl Plugin for BuoyUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            shell::plugin,
            controls::plugin,
            camera::plugin,
            lighting::plugin,
        ));

        #[cfg(feature = "dev")]
        app.add_plugins(debug::plugin);
    }
}
