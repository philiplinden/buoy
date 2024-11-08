mod assets;
mod simulator;
mod ui;

#[cfg(feature = "dev")]
mod dev_tools;

use bevy::{asset::AssetMetaCheck, prelude::*};

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    Splash,
    Loading,
    Running,
}

pub struct AppCorePlugin;

impl Plugin for AppCorePlugin {
    fn build(&self, app: &mut App) {
        // Add new `AppSet` variants by adding them here:
        app.configure_sets(
            Update,
            (AppSet::TickTimers, AppSet::RecordInput, AppSet::Update).chain(),
        );
        // app.init_state::<AppState>();

        // Spawn the main camera.
        app.add_systems(Startup, spawn_camera);

        // Add Bevy plugins.
        app.add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    // Wasm builds will check for meta files (that don't exist)
                    // if this isn't set. This causes errors and even panics on
                    // web build on itch. See
                    // https://github.com/bevyengine/bevy_github_ci_template/issues/48.
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Window {
                        title: "🎈".to_string(),
                        canvas: Some("#bevy".to_string()),
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: true,
                        ..default()
                    }
                    .into(),
                    ..default()
                }),
        );

        // Add other plugins.
        app.add_plugins((
            // ui::InterfacePlugins,
            // assets::AssetTrackingPlugin,
            // assets::ConfigLoaderPlugin,
            // simulator::SimulatorPlugins,
        ));

        // Enable dev tools for dev builds.
        #[cfg(feature = "dev")]
        app.add_plugins(dev_tools::plugin);
    }
}

/// High-level groupings of systems for the app in the `Update` schedule. When
/// adding a new variant, make sure to order it in the `configure_sets` call
/// above.
#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum AppSet {
    /// Tick timers.
    TickTimers,
    /// Record player input.
    RecordInput,
    /// Do everything else (consider splitting this into further variants).
    Update,
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("Camera"),
        Camera3dBundle::default(),
    ));
}
