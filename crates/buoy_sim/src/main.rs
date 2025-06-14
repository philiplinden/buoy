// Disable console on Windows for non-dev builds.
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

use bevy::prelude::*;
use buoy_sim::prelude::*;

fn main() -> AppExit {
    // Configure logging based on the environment
    if cfg!(debug_assertions) {
        if let Err(e) = buoy_sim::logging::configure_dev_logging() {
            eprintln!("Failed to configure development logging: {}", e);
        }
    } else if let Err(e) = buoy_sim::logging::configure_logging() {
        eprintln!("Failed to configure logging: {}", e);
    }

    App::new().add_plugins((
        DefaultPlugins,
        BuoyCommonPlugins,
    )).run()
}
