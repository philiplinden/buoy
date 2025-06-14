# Buoy Aero

Advanced aerodynamics modeling for buoyant bodies. This crate provides mesh-based drag calculations, surface normal computations, and wind effects for realistic aerodynamics simulation.

## Features

- Mesh-based drag force calculations
- Surface normal computations
- Wind effects on bodies
- Turbulence modeling

## Usage

```rust
use buoy_aero::{MeshDrag, WindEffects};

// Calculate mesh-based drag
let mesh_drag = MeshDrag::new(mesh_data);
let drag_force = mesh_drag.calculate_drag(velocity, fluid_density);

// Apply wind effects
let wind = WindEffects::new(wind_velocity);
let wind_force = wind.calculate_force(body_position, body_velocity);
```

## Dependencies

- `bevy`: Game engine and ECS framework
- `nalgebra`: Vector and matrix operations
- `serde`: Serialization support

## Testing

The crate includes comprehensive unit tests for all aerodynamics calculations, validated against known solutions and experimental data.

## Contributing

When adding new aerodynamics models:
1. Include mathematical derivation in documentation
2. Add unit tests with known solutions
3. Benchmark performance impact
4. Update this README with new features

## Note on Separation

This crate is kept separate from `buoy_atmo` to maintain a clear separation of concerns. `buoy_aero` focuses on advanced aerodynamics modeling, including mesh-based drag, surface normals, and wind effects. It depends on `buoy_atmo` for atmospheric conditions but is not part of it. This separation allows for easier testing, maintenance, and future extensions, such as using different atmosphere models without affecting aerodynamics calculations. 