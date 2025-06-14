# Buoy Sim

Core simulation engine for buoyant bodies. This crate orchestrates the physics, aerodynamics, and atmosphere models to create a complete simulation environment.

## Features

- Entity management
- Physics system coordination
- Time management
- Configuration handling
- Event system

## Usage

```rust
use buoy_sim::{Simulation, Config};

// Create a simulation with configuration
let config = Config::default();
let mut simulation = Simulation::new(config);

// Run the simulation
simulation.run();
```

## Dependencies

- `bevy`: Game engine and ECS framework
- `buoy_physics`: Core physics calculations
- `buoy_aero`: Advanced aerodynamics modeling
- `buoy_atmo`: Atmosphere model
- `buoy_common`: Common utilities
- `serde`: Serialization support

## Testing

The crate includes comprehensive unit tests for all simulation components, validated against known solutions and experimental data.

## Contributing

When adding new simulation features:
1. Include mathematical derivation in documentation
2. Add unit tests with known solutions
3. Benchmark performance impact
4. Update this README with new features 