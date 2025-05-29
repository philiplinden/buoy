use bevy::prelude::*;
use uom::si::{f32::{Pressure, ThermodynamicTemperature}};
use std::path::PathBuf;
use std::time::Duration;

pub(crate) fn plugin(app: &mut App) {
    app.register_type::<SimulationConfig>()
       .register_type::<SimulationMode>()
       .register_type::<FlightProfile>()
       .register_type::<FlightPhase>()
       .register_type::<FlightAction>()
       .register_type::<GeoPoint>()
       .init_resource::<SimulationConfig>();
}

#[derive(Resource, Debug, Clone, Reflect)]
pub struct SimulationConfig {
    pub simulation_mode: SimulationMode,
    pub launch_location: GeoPoint,
    pub target_altitude: f32,
    pub flight_duration: Duration,
    pub atmospheric_conditions: AtmosphereProfile,
}

#[derive(Debug, Clone, Reflect)]
pub enum SimulationMode {
    Standalone {
        flight_profile: FlightProfile,
        virtual_autopilot: Option<VirtualAutopilot>,
        interactive_controls: bool,
    },
    HardwareInLoop {
        brp_enabled: bool,
        hardware_interface: HardwareConfig,
    },
}

#[derive(Debug, Clone, Reflect)]
pub struct FlightProfile {
    pub phases: Vec<FlightPhase>,
}

#[derive(Debug, Clone, Reflect)]
pub struct FlightPhase {
    pub name: String,
    pub duration: Option<Duration>,
    pub target_altitude: Option<f32>,
    pub ascent_rate: f32,
    pub actions: Vec<FlightAction>,
}

#[derive(Debug, Clone, Reflect)]
pub enum FlightAction {
    Cutdown,
    BallastDrop { amount: f32 },
    ChangeGasAmount { amount: f32 },
    Custom { name: String, parameters: String },
}

#[derive(Debug, Clone, Reflect)]
pub struct GeoPoint {
    pub latitude: f32,
    pub longitude: f32,
    pub altitude: f32,
}

#[derive(Debug, Clone, Reflect)]
pub struct AtmosphereProfile {
    pub sea_level_pressure: Pressure,
    pub sea_level_temperature: ThermodynamicTemperature,
    pub wind_layers: Vec<WindLayer>,
}

#[derive(Debug, Clone, Reflect)]
pub struct WindLayer {
    pub min_altitude: f32,
    pub max_altitude: f32,
    pub direction: f32,  // degrees from north
    pub speed: f32,      // m/s
    pub turbulence: f32, // intensity factor 0-1
}

#[derive(Debug, Clone, Reflect)]
pub struct VirtualAutopilot {
    pub control_mode: ControlMode,
    pub pid_parameters: PidParameters,
}

#[derive(Debug, Clone, Reflect)]
pub enum ControlMode {
    AltitudeHold,
    AscentRateControl,
    PositionHold,
    Waypoint,
}

#[derive(Debug, Clone, Reflect)]
pub struct PidParameters {
    pub p: f32,
    pub i: f32,
    pub d: f32,
}

#[derive(Debug, Clone, Reflect)]
pub struct HardwareConfig {
    pub timeout_ms: u64,
    pub port: u16,
}

impl Default for SimulationConfig {
    fn default() -> Self {
        Self {
            simulation_mode: SimulationMode::Standalone {
                flight_profile: FlightProfile { phases: vec![] },
                virtual_autopilot: None,
                interactive_controls: true,
            },
            launch_location: GeoPoint {
                latitude: 0.0,
                longitude: 0.0,
                altitude: 0.0,
            },
            target_altitude: 30000.0,
            flight_duration: Duration::from_secs(3600 * 3), // 3 hours default
            atmospheric_conditions: AtmosphereProfile {
                sea_level_pressure: Pressure::standard(),
                sea_level_temperature: ThermodynamicTemperature::standard(),
                wind_layers: vec![],
            },
        }
    }
}
