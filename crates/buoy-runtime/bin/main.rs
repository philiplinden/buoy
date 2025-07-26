// Disable console on Windows for non-dev builds.
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

use bevy::prelude::*;

fn main() {
    let mut app = App::new();

    app.add_plugins((
        buoy_runtime::BuoyDefaultPlugins,
        buoy_physics::BuoyPhysicsPlugin,
    ));

    #[cfg(feature = "gui")]
    app.add_plugins((
        buoy_ui::BuoyUiPlugin,
        bevy_egui::EguiPlugin::default(),
    ));
    #[cfg(feature = "debug-ui")]
    app.add_plugins(bevy_inspector_egui::quick::WorldInspectorPlugin::new());

    app.run();
}
