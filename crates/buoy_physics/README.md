# Buoy Physics

Core physics calculations for buoyant bodies in a fluid medium. This crate provides
the fundamental physics models needed for simulating buoyant objects like balloons
and submersibles.

## Features

- Buoyancy force calculations using Archimedes' principle
- Gas expansion modeling (Charles's Law)
- Basic drag force calculations
- Mass and inertia computations
- Unit conversion utilities

## Usage

```rust
use buoy_physics::{Buoyancy, GasExpansion, Drag};

// Calculate buoyant force
let buoyancy = Buoyancy::new(volume, fluid_density);
let force = buoyancy.calculate_force();

// Model gas expansion
let gas = GasExpansion::new(initial_volume, initial_temp);
let new_volume = gas.expand(new_temp);

// Calculate drag force
let drag = Drag::new(velocity, area, drag_coefficient);
let force = drag.calculate_force();
```

## Dependencies

- `nalgebra`: Vector and matrix operations
- `serde`: Serialization support

## Testing

The crate includes comprehensive unit tests for all physics calculations,
validated against known solutions and experimental data.

## Contributing

When adding new physics models:
1. Include mathematical derivation in documentation
2. Add unit tests with known solutions
3. Benchmark performance impact
4. Update this README with new features 