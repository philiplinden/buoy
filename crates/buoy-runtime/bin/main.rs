// Disable console on Windows for non-dev builds.
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

use bevy::prelude::*;
use buoy_common::prelude::*;

fn main() {
    let mut app = App::new();

    #[cfg(not(feature = "gui"))]
    app.add_plugins(BuoyDefaultPlugins);

    #[cfg(feature = "gui")]
    app.add_plugins((
        BuoyDefaultPlugins.set(
            WindowPlugin {
                primary_window: Window {
                    title: "buoy 🛟".to_string(),
                    canvas: Some("#bevy".to_string()),
                    fit_canvas_to_parent: true,
                    prevent_default_event_handling: true,
                    ..default()
                }
                .into(),
                ..default()
            }),
        buoy_ui::BuoyUiPlugin,
        bevy_egui::EguiPlugin {
            enable_multipass_for_primary_context: true,
        },
    ));
    #[cfg(feature = "debug-ui")]
    app.add_plugins(bevy_inspector_egui::quick::WorldInspectorPlugin::new());

    app.add_plugins((
        buoy_physics::BuoyPhysicsPlugin,
        buoy_runtime::SimpleScenePlugin,
    ));

    app.run();
}
