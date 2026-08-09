#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

use std::time::Duration;
use bevy::{app::ScheduleRunnerPlugin, prelude::*};
use buoy_physics::{
    forces::Velocity,
    objects::balloon::{Balloon, BalloonConfig},
    BuoyDefaultPlugins, BuoyPhysicsPlugin,
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
                    1.0 / 60.0,
                ))),
            BuoyDefaultPlugins,
            BuoyPhysicsPlugin,
        ))
        .add_systems(PostStartup, setup_scenario)
        .add_systems(FixedUpdate, log_balloon_state)
        .run();
}

fn setup_scenario(mut commands: Commands) {
    commands.spawn(Balloon::new_from_config(&BalloonConfig {
        lift_gas_species: "helium".to_string(),
        lift_gas_mass: 10.0,
        balloon_mass: 1.0,
        payload_mass: 1.0,
        drag_coefficient: 0.47,
    }));
}

fn log_balloon_state(
    balloon: Query<(&Transform, &Velocity), (With<Balloon>, Changed<Transform>)>,
) {
    for (transform, velocity) in &balloon {
        info!(
            "altitude={:.2}m velocity={:.3}m/s",
            transform.translation.y, velocity.0.y
        );
    }
}
