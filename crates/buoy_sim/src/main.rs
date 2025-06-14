// Disable console on Windows for non-dev builds.
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

use bevy::prelude::*;
use buoy_sim::prelude::*;

fn main() -> AppExit {
    App::new().add_plugins((
        DefaultPlugins,
        BuoyCommonPlugins,
    )).run()
}
