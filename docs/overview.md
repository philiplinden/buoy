# API Overview

This document provides an overview of the Buoy API and its main components.

## Core Components

### BuoyantBody
The fundamental component for simulating buoyant objects.

```rust
pub struct BuoyantBody {
    pub mass: f32,
    pub volume: f32,
    pub buoyancy_force: Vec3,
}
```

### AeroBody
Handles aerodynamic calculations for bodies in fluid.

```rust
pub struct AeroBody {
    pub profile: AeroProfile,
    pub surface: SurfaceProperties,
    pub total_force: Vec3,
}
```

## Simulation Systems

### Physics Systems
- `buoyancy_system`: Calculates buoyancy forces
- `aerodynamics_system`: Computes aerodynamic forces
- `integration_system`: Updates positions and velocities

### Atmosphere Systems
- `atmosphere_system`: Updates atmospheric conditions
- `wind_system`: Handles wind effects

## Plugins

### BuoySimPlugin
The main plugin that sets up the simulation environment.

```rust
pub struct BuoySimPlugin;

impl Plugin for BuoySimPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_simulation)
           .add_systems(Update, (
               buoyancy_system,
               aerodynamics_system,
               integration_system,
           ));
    }
}
```

### AeroPlugin
Plugin for aerodynamic calculations.

```rust
pub struct AeroPlugin;

impl Plugin for AeroPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            atmosphere_system,
            wind_system,
        ));
    }
}
```

## Examples

See the [examples](../examples) directory for complete usage examples:

- [Basic Simulation](../examples/basic_simulation.rs)
- [Advanced Aerodynamics](../examples/advanced_aerodynamics.rs)