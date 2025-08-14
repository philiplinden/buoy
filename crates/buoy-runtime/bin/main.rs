#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

use std::time::Duration;
use bevy::{app::ScheduleRunnerPlugin, prelude::*};
use buoy_runtime::BuoyDefaultPlugins;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
                    1.0 / 60.0,
                ))),
            BuoyDefaultPlugins,
        ))
        .run();
}
