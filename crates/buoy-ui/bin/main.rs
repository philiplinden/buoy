// Disable console on Windows for non-dev builds.
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

use bevy::prelude::*;
use buoy_physics::forces::Velocity;
use buoy_physics::objects::balloon::{Balloon, BalloonConfig};

fn main() {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "buoy".to_string(),
                ..default()
            }),
            ..default()
        }),
        buoy_physics::BuoyDefaultPlugins,
        buoy_physics::BuoyPhysicsPlugin,
        buoy_ui::BuoyUiPlugin,
        bevy_common_assets::ron::RonAssetPlugin::<buoy_physics::ideal_gas::GasPropertiesConfig>::new(&["configs/properties.ron"]),
        bevy_egui::EguiPlugin::default(),
    ));

    #[cfg(feature = "dev")]
    app.add_plugins(bevy_inspector_egui::quick::WorldInspectorPlugin::new());

    app.add_systems(PostStartup, setup_scenario);
    app.add_systems(FixedUpdate, balloon_props);
    app.run();
}

fn setup_scenario(
    mut commands: Commands,
) {
    commands.spawn(
        Balloon::new_from_config(
            &BalloonConfig {
                lift_gas_species: "helium".to_string(),
                lift_gas_mass: 10.0,
                balloon_mass: 1.0,
                payload_mass: 1.0,
                drag_coefficient: 0.47,
            },
        ),
    );
}

fn balloon_props(
    balloon: Query<(&Transform, &Velocity), (With<Balloon>, Changed<Transform>)>,
) {
    for (transform, velocity) in balloon.iter() {
        info!("Position: {:?}, Velocity: {:?}", transform.translation, velocity.0);
    }
}
