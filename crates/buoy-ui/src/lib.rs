mod camera;
mod controls;
mod lighting;
mod scene;
mod shell;

#[cfg(feature = "dev")]
mod debug;

use bevy::prelude::*;

pub struct BuoyUiPlugin;

impl Plugin for BuoyUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            controls::plugin,
            camera::plugin,
            lighting::plugin,
            scene::plugin,
        ));

        #[cfg(feature = "dev")]
        app.add_plugins(debug::plugin);

        #[cfg(feature = "egui")]
        app.add_plugins(shell::plugin);

        #[cfg(feature = "inspect")]
        app.add_plugins(bevy_inspector_egui::quick::WorldInspectorPlugin::new());
    }
}
