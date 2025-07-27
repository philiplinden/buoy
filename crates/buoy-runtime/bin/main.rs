// Disable console on Windows for non-dev builds.
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

use bevy::prelude::*;
use avian3d::prelude::*;
use buoy_runtime::objects::balloon::{Balloon, BalloonConfig};

fn main() {
    let mut app = App::new();

    app.add_plugins((
        buoy_runtime::BuoyDefaultPlugins,
        buoy_physics::BuoyPhysicsPlugin,
        bevy_common_assets::ron::RonAssetPlugin::<buoy_physics::ideal_gas::GasPropertiesConfig>::new(&["configs/properties.ron"]),
    ));

    #[cfg(feature = "gui")]
    app.add_plugins((buoy_gui::BuoyGuiPlugin, bevy_egui::EguiPlugin::default()));
    #[cfg(feature = "debug-ui")]
    app.add_plugins(bevy_inspector_egui::quick::WorldInspectorPlugin::new());

    app.add_systems(PostStartup, setup_scenario);
    app.add_systems(FixedUpdate, balloon_props);
    app.run();
}

fn setup_scenario(
    mut commands: Commands,
) {
    commands.spawn((
        Balloon::new_from_config(
            &BalloonConfig {
                lift_gas_species: "helium".to_string(),
                lift_gas_mass: 10.0,
                balloon_mass: 1.0,
                payload_mass: 1.0,
                drag_coefficient: 0.47,
            },
        ),
        RigidBody::Dynamic,
        ExternalForce::default().with_persistence(false),
    ));
}

fn balloon_props(
    balloon: Query<(&Transform, &LinearVelocity), (With<Balloon>, Changed<Transform>)>,
) {
    for (transform, velocity) in balloon.iter() {
        info!("Position: {:?}, Velocity: {:?}", transform.translation, velocity.0);
    }
}
