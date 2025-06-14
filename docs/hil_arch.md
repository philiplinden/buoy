# Hardware-in-the-Loop Balloon Simulation Design Document

## Overview
A (faster than) real-time physics simulation server for high-altitude balloon
testing using Bevy game engine with Bevy Remote Protocol (BRP) for hardware
integration.

## Architecture

All physics calculations and simulated behaviors are defined in
[`buoy-core`](../crates/buoy_core). 

The [`buoy-server`](../crates/buoy_server/) application spools up an environment
that simulates all bodies instanced within it with
[`buoy-core`](../crates/buoy_core/). The purpose of the server crate is to
bridge virtual and hardware-in-the-loop devices to a shared simulated
environment. The server handles communication to and from the simulation and
devices or clients.

Instances of [`buoy-client`](../crates/buoy_client/) connect to the server and
allow humans to interact with the simulated objects and environment. The client
handles rendering and visualizing simulation data.

The reason for choosing server-client architecture is primarily driven by the
future vision of running hardware-in-the-loop tests and simulated flights that
send mock data to sensors and actuators to "fool" them into believing it is a
real flight. Before incorporating hardware, there is also an opportunity to run
real flight software on fully virtual devices within the simulation, as long as
the flight software applications are capable of interfacing across the server
interface.

```
┌─────────────────┐    JSON-RPC/HTTP   ┌──────────────────┐
│ Flight Computer │◄──────────────────►│ Bevy HIL Server  │
│ (Hardware)      │                    │ (Physics Engine) │
└─────────────────┘                    └──────────────────┘
                                                │
                                                │ TCP/WebSocket
                                                ▼
                                       ┌──────────────────┐
                                       │ Visualization    │
                                       │ Client (Optional)│
                                       └──────────────────┘
```

This does increase complexity quite a bit, so early builds of the server will
have client tools integrated into a single app until the physics simulations are
stable. See [sim_arch.md](./sim_arc.md) for detailed discussion of the
physics simulation architecture.

Below is a concept design for the full app architecture for standalone and
hardware-in-the-loop modes of the server-client system.

### Simulation Modes

#### 1. Standalone Mode (No Hardware)
**Purpose**: Development, testing, and experimentation without flight hardware

**Features**:
- **Automated flight profile**: Pre-programmed ascent/descent sequences
- **Virtual autopilot**: Simple PID controllers for altitude/position hold
- **Parameter sweeps**: Batch testing of balloon configurations
- **Monte Carlo runs**: Statistical analysis with varying conditions
- **Interactive control**: Keyboard/mouse input for real-time experimentation

**Configuration**:
```rust
enum SimulationMode {
    Standalone {
        flight_profile: FlightProfile,
        virtual_autopilot: Option<VirtualAutopilot>,
        interactive_controls: bool,
    },
    HardwareInLoop {
        brp_enabled: true,
        hardware_interface: HardwareConfig,
    },
}
```

#### 2. Hardware-in-the-Loop Mode
**Purpose**: Integration testing with real flight computers and autopilots
- Full BRP JSON-RPC interface enabled
- External hardware control via HTTP requests
- Real-time telemetry streaming

### Communication Protocol
- **Primary Interface**: JSON-RPC 2.0 over HTTP (via BRP)
- **Port**: 15702 (Bevy default)
- **Transport**: RESTful HTTP requests for hardware integration
- **Data Format**: JSON with typed component serialization

## Core Systems

### 1. Physics Engine (Avian3D Integration)
**Responsibility**: Real-time balloon dynamics simulation using Avian3D physics engine

**Avian3D Components**:
- **RigidBody**: Dynamic rigid body with 6DOF motion
- **Collider**: Mesh-based collision shape from balloon geometry
- **Mass**: Computed from balloon envelope + payload + lift gas
- **LinearDamping/AngularDamping**: Atmospheric drag effects

**Custom Components**:
- **Balloon**: Physical properties (envelope volume, mass, burst altitude, mesh geometry)
- **BalloonPhysics**: Simulation state (buoyant force, gas volume, atmospheric conditions)
- **MeshDragCalculator**: Surface-based drag computation from balloon mesh

**Systems**:
- `atmospheric_model`: Standard atmosphere calculations (pressure, temperature, density)
- `buoyancy_system`: Apply Archimedes principle forces via Avian3D force API
- `mesh_drag_system`: Calculate drag forces based on mesh surface normals and flow vectors
- `gas_expansion_system`: Update balloon volume and mesh scale with altitude/temperature
- `wind_forces_system`: Apply wind loads to mesh surface elements

**Mesh-Based Drag Implementation**:
- Load balloon 3D mesh as Avian3D Collider
- Calculate surface normals for each mesh triangle
- Compute drag coefficient per surface element based on flow angle
- Apply distributed forces across balloon surface
- Results in realistic tumbling, rotation, and asymmetric drag effects

### 2. Atmospheric Model
**Responsibility**: Environmental conditions affecting balloon flight

**Features**:
- Standard atmosphere model (pressure, temperature, density vs altitude)
- Configurable sea-level conditions
- Wind velocity fields with turbulence
- Thermal effects on gas volume (Charles's Law)

**Parameters**:
- Sea level pressure: 101,325 Pa
- Temperature lapse rate: 6.5 K/km
- Wind profiles: Configurable by altitude layers

### 3. Hardware Interface (BRP)
**Responsibility**: External system communication

**Endpoints**:
- `bevy/get`: Query component data (position, telemetry, status)
- `bevy/insert`: Add components (commands, waypoints)
- `bevy/remove`: Remove components (abort commands)
- `bevy/run_system`: Trigger specific behaviors (cutdown, ballast drop)

**Data Exchange**:
```json
// Get balloon telemetry
POST /bevy/rpc
{
  "method": "bevy/get",
  "params": {
    "entity": "balloon_001",
    "components": ["Transform", "Telemetry", "BalloonPhysics"]
  }
}

// Send flight command
POST /bevy/rpc
{
  "method": "bevy/insert",
  "params": {
    "entity": "balloon_001",
    "components": {
      "FlightCommand": {
        "command_type": "SetDescentRate",
        "parameters": {"rate": 5.0}
      }
    }
  }
}
```

### 4. Telemetry System
**Responsibility**: Data logging and state monitoring

**Telemetry Data**:
- Position: GPS coordinates (lat, lon, altitude)
- Atmospheric: Pressure, temperature, air density
- Dynamics: Ascent/descent rate, ground speed, heading
- System: Battery voltage, internal temperature, GPS fix quality

**Update Rate**: 1-10 Hz (configurable based on hardware requirements)

## Integration Patterns

### Flight Computer Integration
Think of this like a flight simulator's instrument panel - your autopilot reads the "instruments" (telemetry) and sends "control inputs" (commands) to the simulation.

**Typical Integration Flow**:
1. **Initialization**: Flight computer requests balloon configuration
2. **Control Loop**: 
   - Read current state via BRP GET requests
   - Execute flight control algorithms
   - Send commands via BRP INSERT requests
   - Log telemetry data
3. **Mission Events**: Trigger systems like cutdown, ballast release

### Real-time Constraints
- **Physics Timestep**: Fixed 50Hz (20ms) for deterministic simulation
- **Network Latency**: < 10ms for local HTTP communication
- **State Consistency**: All component updates are atomic per frame

## Configuration Management

### Balloon Parameters
```rust
struct BalloonConfig {
    envelope_volume: f32,      // m³ at burst
    payload_mass: f32,         // kg
    envelope_mass: f32,        // kg  
    lift_gas: GasType,         // Helium, Hydrogen
    burst_altitude: f32,       // m
    mesh_path: String,         // Path to balloon 3D mesh file
    material_properties: MaterialConfig, // Surface roughness, permeability
}

struct MaterialConfig {
    surface_roughness: f32,    // Surface roughness factor
    drag_coefficient_base: f32, // Base Cd for smooth flow
    reynolds_scaling: f32,     // Reynolds number dependency
    mesh_resolution: u32,      // Triangle count for drag calculations
}
```

### Mission Parameters
```rust
struct MissionConfig {
    simulation_mode: SimulationMode,
    launch_location: GeoPoint,
    target_altitude: f32,
    flight_duration: Duration,
    termination_conditions: Vec<TerminationCondition>,
    atmospheric_conditions: AtmosphereProfile,
}

enum SimulationMode {
    Standalone {
        control_mode: ControlMode,
        enable_gui: bool,
        log_output: Option<PathBuf>,
    },
    HardwareInLoop {
        brp_port: u16,
        hardware_timeout: Duration,
    },
}

struct FlightProfile {
    phases: Vec<FlightPhase>,
}

struct FlightPhase {
    name: String,
    duration: Option<Duration>,     // None = altitude-triggered
    target_altitude: Option<f32>,   // None = time-triggered
    ascent_rate: f32,
    actions: Vec<FlightAction>,     // Ballast drops, cutdowns, etc.
}
```

## Testing Strategy

### Unit Testing
- Individual physics calculations (buoyancy, gas expansion, mesh drag integration)
- Avian3D force application validation
- Mesh surface normal calculations
- Component serialization/deserialization
- BRP endpoint validation

### Integration Testing
- Hardware-in-the-loop with mock flight computer
- Avian3D physics pipeline validation
- Real-time performance with mesh-based drag
- Network communication reliability

### Validation Testing
- Compare against known balloon flight data
- Mesh drag vs. analytical drag model comparison
- Atmospheric model accuracy verification
- Wind drift prediction with realistic balloon geometry

## Performance Considerations

### Computational Complexity
- **Physics**: O(1) per balloon per timestep (Avian3D handles integration)
- **Mesh Drag**: O(n) where n = triangle count in balloon mesh
- **Atmospheric**: O(1) lookup tables for standard atmosphere
- **Network**: O(1) JSON-RPC request handling

### Scalability
- Single balloon: ~0.5ms CPU per timestep (including mesh drag calculations)
- Memory usage: ~5KB per balloon entity (including mesh data)
- Network throughput: ~2KB/s at 10Hz telemetry rate (additional mesh data)
- Mesh resolution: 1000-5000 triangles recommended for real-time performance

### Real-time Guarantees
- Fixed timestep physics simulation
- Bounded network response times
- Graceful degradation under load

## Development Phases

### Phase 1: Standalone Foundation
- Avian3D integration and basic balloon dynamics
- Standard atmosphere model
- Mesh loading and collision shape generation
- Interactive control system (keyboard/mouse)
- Configuration file support

### Phase 2: Advanced Standalone Features
- Virtual autopilot with PID controllers
- Pre-programmed flight profiles
- Batch testing and parameter sweeps
- Monte Carlo simulation capability
- CSV logging and data analysis tools

### Phase 3: Hardware Integration  
- BRP protocol implementation
- Serial/USB communication layer
- MAVLink protocol support (if applicable)
- Flight computer interface validation
- Real-time performance optimization

### Phase 4: Advanced Aerodynamics
- Mesh-based drag calculation implementation
- Surface normal computation and force distribution
- Gas expansion effects on balloon geometry
- Realistic tumbling and rotation dynamics

### Phase 5: Production Features
- Multi-balloon support
- Weather data integration
- Predictive trajectory modeling
- Web-based mission control dashboard

### Phase 4: Visualization & Monitoring
- Real-time 3D visualization client
- Web-based mission control dashboard
- Historical flight data replay

## Dependencies

### Core Dependencies
- `bevy`: Game engine and ECS framework
- `avian3d`: Physics engine for rigid body dynamics and collision detection
- `bevy_remote`: BRP protocol implementation  
- `serde`: JSON serialization
- `tokio`: Async runtime for networking
- `bevy_asset`: 3D mesh loading for balloon geometry

### Hardware Integration
- `serialport`: Serial communication
- `mavlink`: Autopilot protocol (optional)
- `chrono`: Timestamp handling

### Optional Dependencies
- `bevy_egui`: Debug UI
- `plotters`: Real-time graphing
- `geo`: Geographic calculations

## Risk Mitigation

### Simulation Accuracy
- **Risk**: Physics model deviates from reality
- **Mitigation**: Validate against historical flight data, configurable model parameters

### Real-time Performance
- **Risk**: Simulation cannot maintain real-time execution
- **Mitigation**: Performance profiling, fixed timestep with catch-up, graceful degradation

### Hardware Communication
- **Risk**: Network latency affects control loop stability  
- **Mitigation**: Local HTTP server, bounded response times, heartbeat monitoring

### Data Integrity
- **Risk**: Component state corruption during updates
- **Mitigation**: Atomic transactions, state validation, rollback capability

This architecture provides a clean separation between physics simulation and hardware integration, enabling realistic balloon flight testing while maintaining the flexibility to integrate with various flight computer systems.
