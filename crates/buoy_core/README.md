# Buoy Core

Legacy core physics and simulation code. This crate contains the original implementation of physics calculations, aerodynamics, and simulation orchestration. It is being gradually refactored into separate crates for better modularity.

## Features

- Legacy physics calculations
- Legacy aerodynamics modeling
- Legacy simulation orchestration

## Usage

This crate is being phased out in favor of more modular crates:
- `buoy_physics`: Core physics calculations
- `buoy_aero`: Advanced aerodynamics modeling
- `buoy_sim`: Core simulation engine
- `buoy_atmo`: Atmosphere model
- `buoy_common`: Common utilities

## Dependencies

- `bevy`: Game engine and ECS framework
- `nalgebra`: Vector and matrix operations
- `serde`: Serialization support

## Testing

The crate includes comprehensive unit tests for all legacy components, validated against known solutions and experimental data.

## Contributing

This crate is being phased out. Please contribute to the new modular crates instead.
