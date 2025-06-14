# Buoy - Buoyancy Simulation Engine

## Overview

A (faster than) real-time physics simulation engine for solid and hollow bodies
within a fluid volume, such as balloons and submersibles. Primarily focuses on
lighter-than-air crafts in a standard atmosphere. Built with [Bevy](https://bevyengine.org/) in
Rust.

## Architecture

The simulation engine is built from modular components that can be used independently
or together to create a complete simulation environment.

### Core Physics Components

#### `buoy_atmo`
- U.S. Standard Atmosphere 1976 implementation
- Temperature, pressure, and density calculations
- Configurable sea-level conditions
- No wind or humidity modeling

#### `buoy_physics`
- Fundamental physics calculations
- Buoyancy force computation
- Gas expansion (Charles's Law)
- Basic drag models
- Mass and inertia calculations

#### `buoy_aero`
- Advanced aerodynamics modeling
- Mesh-based drag calculations
- Surface normal computations
- Wind effects on bodies
- Turbulence modeling

### Simulation Engine

#### `buoy_sim`
- Core simulation orchestration
- Entity management
- Physics system coordination
- Time management
- Configuration handling
- Event system

### Client/Server Infrastructure

#### `buoy_server`
- Physics simulation hosting
- Hardware interface management
- Network protocol handling
- Real-time constraints enforcement
- State synchronization

#### `buoy_client`
- 3D visualization
- UI components
- Telemetry display
- Interactive controls
- Data logging

### Hardware Integration

#### `buoy_hil`
- Hardware-in-loop interface
- BRP protocol implementation
- Hardware device management
- Real-time constraints
- Device synchronization

### Common Utilities

#### `buoy_common`
- Shared data structures
- Unit conversions
- Math utilities
- Common traits
- Error handling

## Dependencies

Each crate should minimize its dependencies on other crates in the project.
External dependencies should be carefully selected to avoid unnecessary bloat.

### Core Dependencies
- `bevy`: Game engine and ECS framework
- `serde`: Serialization
- `nalgebra`: Math operations
- `tokio`: Async runtime (for server/client)

### Optional Dependencies
- `bevy_egui`: Debug UI
- `bevy_remote`: BRP protocol
- `plotters`: Data visualization

## Development Phases

### Phase 1: Core Physics
- Implement atmosphere model
- Basic buoyancy calculations
- Simple drag models
- Unit tests and validation

### Phase 2: Simulation Engine
- Entity management
- Physics system integration
- Time management
- Configuration system

### Phase 3: Client/Server
- Basic visualization
- Network protocol
- State synchronization
- Interactive controls

### Phase 4: Hardware Integration
- BRP protocol
- Device management
- Real-time constraints
- Validation testing

## Testing Strategy

Each crate should include:
- Unit tests for all calculations
- Integration tests for component interaction
- Validation against known data
- Performance benchmarks

## Performance Considerations

- Fixed timestep physics
- Efficient memory usage
- Minimal allocations
- Parallel computation where possible
- Real-time guarantees

## Future Extensions

- Weather data integration
- Multi-body interactions
- Advanced aerodynamics
- Machine learning integration
- Distributed simulation

## Release Process

The project uses semantic versioning and automated release workflows. Here's how to create a new release:

### 1. Prepare Release
```bash
# Update version in Cargo.toml
# Make sure all changes are committed
git add Cargo.toml
git commit -m "chore: bump version to X.Y.Z"
```

### 2. Create and Push Tag
```bash
git tag vX.Y.Z
git push origin vX.Y.Z
```

### 3. Automated Steps
The following steps are handled automatically by GitHub Actions:
- Changelog generation from conventional commits
- Creation of GitHub Release with changelog
- PR creation with changelog updates
- Build and packaging of release artifacts

### 4. Review and Publish
- Review and merge the changelog PR
- The release will be automatically populated with:
  - Changelog in description
  - CHANGELOG.md as artifact
  - Build artifacts from release workflow

### Commit Convention
All commits should follow the [Conventional Commits](https://www.conventionalcommits.org/) specification:
- `feat:` - New features
- `fix:` - Bug fixes
- `docs:` - Documentation changes
- `style:` - Code style changes
- `refactor:` - Code refactoring
- `perf:` - Performance improvements
- `test:` - Adding or modifying tests
- `chore:` - Maintenance tasks

## Documentation

The project documentation is automatically generated and deployed to GitHub Pages with each release. Documentation includes:

- API documentation for all crates
- README and additional documentation from the `docs/` directory
- Code examples and usage guides

### Documentation Structure

- `docs/` - Additional documentation and guides
  - `architecture.md` - System architecture overview
  - `development.md` - Development setup and guidelines
  - `api/` - API-specific documentation
  - `examples/` - Code examples and tutorials

### Writing Documentation

1. **Code Documentation**
   ```rust
   /// Brief description
   ///
   /// Detailed explanation
   /// 
   /// # Examples
   /// ```
   /// use my_crate::MyStruct;
   /// let instance = MyStruct::new();
   /// ```
   pub struct MyStruct;
   ```

2. **Additional Documentation**
   - Place markdown files in the `docs/` directory
   - Use relative links to other documentation
   - Include code examples where relevant

### Viewing Documentation

- Latest release: [https://philiplinden.github.io/buoy/](https://philiplinden.github.io/buoy/)
- Local development: `cargo doc --open`

## License

Except where noted (below and/or in individual files), all code in this
repository is dual-licensed under either:

* MIT License ([LICENSE-MIT](LICENSE-MIT) or
  [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))

at your option. This means you can select the license you prefer! This
dual-licensing approach is the de-facto standard in the Rust ecosystem and there
are [very good reasons](https://github.com/bevyengine/bevy/issues/2373) to
include both.
