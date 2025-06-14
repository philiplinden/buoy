# Buoy Physics

This crate provides physics simulation components for the Buoy project, handling all physics-related calculations and simulations.

## Features

- Force calculations and dynamics
- Material properties and constants
- Ideal gas law implementations
- Physical constants and unit conversions

## Usage

```rust
use buoy_physics::{forces, ideal_gas, material_properties};

// Calculate forces
let force = forces::calculate_force(mass, acceleration);

// Use ideal gas law
let pressure = ideal_gas::calculate_pressure(volume, temperature, moles);

// Get material properties
let density = material_properties::get_density(material);
```

## Examples

See the `examples` directory for complete usage examples.

## Testing

This crate uses both unit tests and property-based tests to ensure correctness:

```bash
# Run all tests
cargo test

# Run property-based tests
cargo test --features proptest
```

## Contributing

Please see the main [CONTRIBUTING.md](../../CONTRIBUTING.md) for guidelines.

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](../../LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](../../LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option. 