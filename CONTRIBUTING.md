# Contributing to Buoy

Thank you for your interest in contributing to Buoy! This document outlines the standards and conventions we follow.

## Code Organization

### Crate Structure
- Each crate should have a clear, single responsibility
- Place example files at the top level of each crate
- Use `src/tests/` directory for integration tests
- Keep unit tests in the same file as the code they test
- Inherit author and license fields from workspace Cargo.toml

### Documentation
- Use rustdoc for all documentation
- Include a module-level doc comment in `lib.rs`
- Document all public APIs
- Include examples in doc comments where appropriate
- Keep README.md up to date with crate overview and usage
- Documentation is automatically generated and deployed to GitHub Pages

### Code Style
- Follow Rust's standard formatting (use `cargo fmt`)
- Run `cargo clippy` before submitting PRs
- Use meaningful variable and function names
- Keep functions focused and small
- Use type aliases for complex types
- Prefer explicit over implicit
- Follow Bevy's coding style for Bevy-related code

### Testing
- Write unit tests for all new functionality
- Include property-based tests for physics calculations
- Maintain test coverage for critical paths
- Use `#[cfg(test)]` for test-only code
- Add tests for edge cases and error conditions
- Use proptest for physics calculations

### Error Handling
- Use `tracing` for logging and error reporting
- Return `Result` types with descriptive error messages
- Use custom error types where appropriate
- Log errors with appropriate severity levels
- Include context in error messages
- Handle errors at appropriate levels
- Use `?` operator for error propagation
- Document error conditions in public APIs

### Performance
- Profile code before optimizing
- Use appropriate data structures
- Consider memory layout for hot paths
- Document performance characteristics
- Use Bevy's performance best practices
- Minimize allocations in hot paths

## Pull Request Process

1. Create a feature branch from `main`
2. Make your changes following the standards above
3. Add or update tests as needed
4. Update documentation
5. Run `cargo test` and `cargo clippy`
6. Submit PR with clear description of changes
7. Ensure CI passes before requesting review

## Crate Responsibilities

### Core Components
- `buoy_physics`: Physics calculations, simulations, and core data structures
- `buoy_atmo`: Atmospheric modeling, calculations, and data structures
- `buoy_aero`: Aerodynamic calculations, models, and data structures

### Application
- `buoy_sim`: Main simulation application and Bevy integration

### Utilities
- `buoy_common`: Shared utilities, types, formatting, and configuration management
