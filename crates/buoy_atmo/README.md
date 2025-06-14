# Buoy Atmo

U.S. Standard Atmosphere 1976 implementation for buoy simulation. This crate provides temperature, pressure, and density calculations for altitudes between -57 to 85,000 kilometers above sea level.

## Features

- U.S. Standard Atmosphere 1976 implementation
- Temperature, pressure, and density calculations
- Configurable sea-level conditions
- No wind or humidity modeling

## Usage

```rust
use buoy_atmo::{Atmosphere, Altitude};

// Create an atmosphere model
let atmosphere = Atmosphere::new();

// Calculate temperature at altitude
let altitude = Altitude::from_meters(1000.0);
let temperature = atmosphere.temperature(altitude);

// Calculate pressure at altitude
let pressure = atmosphere.pressure(altitude);

// Calculate density at altitude
let density = atmosphere.density(altitude);
```

## Dependencies

- `bevy`: Game engine and ECS framework
- `serde`: Serialization support

## Testing

The crate includes comprehensive unit tests for all atmosphere calculations, validated against known solutions and experimental data.

## Contributing

When adding new atmosphere models:
1. Include mathematical derivation in documentation
2. Add unit tests with known solutions
3. Benchmark performance impact
4. Update this README with new features

## Note on Separation

This crate is kept separate from `buoy_aero` to maintain a clear separation of concerns. `buoy_atmo` focuses solely on the U.S. Standard Atmosphere 1976 model, providing temperature, pressure, and density calculations. It is a standalone model with no dependencies on aerodynamics. This separation allows for easier testing, maintenance, and future extensions, such as adding different atmosphere models without affecting aerodynamics calculations.