use bevy::prelude::*;
use bevy_remote::RemotePlugin;

use crate::balloon::BalloonPhysics;
use crate::core::SimState;
use crate::config::{SimulationConfig, SimulationMode};

pub(crate) fn plugin(app: &mut App) {
    // Add BRP support - this exposes your simulation via JSON-RPC
    app.add_plugins(RemotePlugin::default())
       // Add systems for hardware-in-the-loop functionality
       .add_systems(Update, process_flight_commands)
       .add_systems(Update, update_telemetry)
       .add_systems(
        Update, 
        check_hardware_connection.run_if(in_state(SimState::HardwareWaiting))
    );
}

/// System to check hardware connection status
fn check_hardware_connection(
    mut next_state: ResMut<NextState<SimState>>,
    config: Res<SimulationConfig>,
    time: Res<Time>,
) {
    // This is a placeholder for actual hardware connection logic
    // In a real implementation, this would:
    // 1. Check if the hardware is connected
    // 2. Transition to Running if connected, or stay in HardwareWaiting
    
    // For now, just transition after a short delay to simulate hardware connection
    if let SimulationMode::HardwareInLoop { .. } = &config.simulation_mode {
        if time.elapsed_seconds() > 2.0 {
            info!("Hardware connected");
            next_state.set(SimState::Running);
        }
    }
}

/// Component for telemetry data that will be exposed to hardware
#[derive(Component, Debug, Clone, Reflect, Default)]
pub struct Telemetry {
    pub latitude: f32,
    pub longitude: f32,
    pub altitude: f32,
    pub pressure: f32,
    pub temperature: f32,
    pub ascent_rate: f32,
    pub heading: f32,
}

/// Component for commands received from hardware
#[derive(Component, Debug, Clone, Reflect)]
pub struct FlightCommand {
    pub command_type: CommandType,
    pub parameters: String, // JSON parameters
    pub timestamp: f64,
}

#[derive(Debug, Clone, Reflect)]
pub enum CommandType {
    Cutdown,
    ReleaseBallast,
    ChangeGasAmount,
    Abort,
}

/// System to process commands received via BRP
fn process_flight_commands(
    mut commands: Commands,
    query: Query<(Entity, &FlightCommand)>,
) {
    for (entity, flight_command) in query.iter() {
        info!("Processing command: {:?}", flight_command.command_type);
        
        // Implement the command logic
        match flight_command.command_type {
            CommandType::Cutdown => {
                // TODO: Implement cutdown logic
            }
            CommandType::ReleaseBallast => {
                // TODO: Implement ballast release
            }
            CommandType::ChangeGasAmount => {
                // TODO: Implement gas amount change
            }
            CommandType::Abort => {
                // TODO: Implement abort logic
            }
        }
        
        // Remove the command after processing
        commands.entity(entity).remove::<FlightCommand>();
    }
}

/// System to update telemetry data for BRP clients
fn update_telemetry(
    mut query: Query<(&mut Telemetry, &Transform, &BalloonPhysics)>,
) {
    for (mut telemetry, transform, physics) in query.iter_mut() {
        // Update telemetry with current simulation state
        telemetry.altitude = transform.translation.y;
        telemetry.pressure = physics.air_pressure;
        telemetry.temperature = physics.air_temperature;
        
        // Calculate ascent rate and heading
        // (in a real implementation, you'd track previous positions and calculate these)
        telemetry.ascent_rate = 0.0; // placeholder
        telemetry.heading = 0.0; // placeholder
        
        // In a real implementation, you'd convert world coordinates to GPS
        telemetry.latitude = 0.0; // placeholder
        telemetry.longitude = 0.0; // placeholder
    }
}
