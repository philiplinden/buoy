// Disable console on Windows for non-dev builds.
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

use bevy::prelude::*;

fn main() {
    let mut app = App::new();

    app.add_plugins((
        buoy_runtime::BuoyDefaultPlugins,
        buoy_runtime::ConsolePlugin,
        buoy_physics::BuoyPhysicsPlugin,
        bevy_common_assets::ron::RonAssetPlugin::<buoy_physics::ideal_gas::GasPropertiesConfig>::new(&["configs/properties.ron"]),
    ));

    #[cfg(feature = "gui")]
    app.add_plugins((buoy_gui::BuoyGuiPlugin, bevy_egui::EguiPlugin::default()));
    #[cfg(feature = "debug-ui")]
    app.add_plugins(bevy_inspector_egui::quick::WorldInspectorPlugin::new());

    app.run();
}
