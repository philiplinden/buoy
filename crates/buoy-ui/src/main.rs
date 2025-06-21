// Disable console on Windows for non-dev builds.
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

use bevy::prelude::*;

fn main() {
    let mut app = App::new();

    app.add_plugins((
        buoy_common::BuoyDefaultPlugins.set(WindowPlugin {
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
        buoy_physics::BuoyPhysicsPlugin,
        buoy_ui::BuoyUiPlugin,
    ));

    #[cfg(feature = "egui")]
    app.add_plugins(bevy_egui::EguiPlugin {
        enable_multipass_for_primary_context: true,
    });

    app.run();
}
