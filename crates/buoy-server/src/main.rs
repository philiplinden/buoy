// Support configuring Bevy lints within code.
#![cfg_attr(bevy_lint, feature(register_tool), register_tool(bevy))]
// // Disable console on Windows for non-dev builds.
// #![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

use bevy::{
    asset::AssetMetaCheck,
    prelude::*,
};
use avian3d::prelude::*;

use buoy_core::prelude::{
    BuoyPlugin,
    BalloonPhysics,
    units,
    // SimulationConfig,
    // SimulationMode,
    // FlightProfile,
    // RemotePlugin,
    // RemoteConfig,
    // RemoteSettings,
};

fn main() -> AppExit {
    App::new().add_plugins(AppPlugin).run()
}

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        // // Configure the simulation mode
        // let simulation_config = SimulationConfig {
        //     simulation_mode: SimulationMode::Standalone {
        //         flight_profile: FlightProfile { phases: vec![] },
        //         virtual_autopilot: None,
        //         interactive_controls: false, // No interactive controls in headless mode
        //     },
        //     // Set other configuration values as needed
        //     ..default()
        // };

        // Add only the plugins needed for headless operation
        app.add_plugins((
            DefaultPlugins
                .build()
                .set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                }),
            BuoyPlugin,
        ));

        // // Add the Remote plugin if in hardware-in-the-loop mode
        // match &simulation_config.simulation_mode {
        //     SimulationMode::HardwareInLoop { brp_enabled, hardware_interface } => {
        //         if *brp_enabled {
        //             // Use bevy_remote in headless mode
        //             app.add_plugins(RemotePlugin::default());
        //             app.insert_resource(RemoteConfig::new(RemotePort(hardware_interface.port)));
        //             app.insert_resource(RemoteSettings::default());
        //         }
        //     },
        //     _ => {}
        // }

        // // Add simulation configuration
        // app.insert_resource(simulation_config);

        // Order the app systems
        app.configure_sets(
            Update,
            (
                AppSystems::TickTimers,
                AppSystems::RecordInput,
                AppSystems::Update,
            ).chain(),
        );

        // Add startup system to setup the balloon
        app.add_systems(Startup, setup_balloon);

        // Add a system to log balloon state periodically
        app.add_systems(Update, log_balloon_state.run_if(every_seconds(1.0)));
    }
}

/// High-level groupings of systems for the app in the `Update` schedule.
#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum AppSystems {
    TickTimers,
    RecordInput,
    Update,
}

/// Setup the initial balloon without visual components
fn setup_balloon(mut commands: Commands) {
    info!("Initializing balloon in headless mode");

    // Create a balloon entity with physics components but no visual representation
    commands.spawn((
        // // Balloon component with physical properties
        // Balloon {
        //     envelope_volume: Volume::new::<cubic_meter>(1000.0),
        //     envelope_mass: Mass::new::<kilogram>(2.0),
        //     payload_mass: Mass::new::<kilogram>(1.5),
        //     lift_gas: GasSpecies::helium(),
        //     burst_altitude: Length::new::<meter>(30000.0),
        //     mesh_path: "models/balloon.glb".to_string(),
        //     ..default()
        // },
        // // Physics state component
        // BalloonPhysics::default(),
        // // Drag calculator
        // MeshDragCalculator::default(),
        // // Telemetry component for hardware interface
        // Telemetry::default(),
        // // Physics body components from Avian3D
        // RigidBody::Dynamic,
        // Position::default(),
        // Rotation::default(),
        // LinearVelocity::default(),
        // AngularVelocity::default(),
        // ExternalForce::default(),
        // GravityScale(1.0),
        // Name for identification
        Name::new("Balloon"),
    ));

    info!("Balloon simulation initialized in headless mode");
}

/// Log the state of the balloon periodically
fn log_balloon_state(
    balloon_query: Query<(&BalloonPhysics, &Position)>
) {
    for (physics, position) in balloon_query.iter() {
        info!(
            "Balloon state - Position: ({:.2}, {:.2}, {:.2}) m, Volume: {:.2} m³, Buoyant force: {:.2} N, Burst: {}",
            position.x, position.y, position.z,
            physics.current_volume.get::<units::volume::cubic_meter>(),
            physics.buoyant_force.get::<units::force::newton>(),
            physics.burst
        );
    }
}

/// Helper function to run systems at a given frequency
fn every_seconds(seconds: f32) -> impl FnMut(Res<Time>) -> bool + Clone {
    let mut accumulated_time = 0.0;
    move |time: Res<Time>| {
        accumulated_time += time.delta_secs();
        if accumulated_time >= seconds {
            accumulated_time = 0.0;
            true
        } else {
            false
        }
    }
}
