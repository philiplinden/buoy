# Contributing to Buoy

Thank you for your interest in contributing to Buoy! This document outlines the standards and conventions we follow.

## Code Organization

### Crate Structure
- Each crate should have a clear, single responsibility
- Place example files at the top level of each crate
- Use `src/tests/` directory for integration tests
- Keep unit tests in the same file as the code they test

### Documentation
- Use rustdoc for all documentation
- Include a module-level doc comment in `lib.rs`
- Document all public APIs
- Include examples in doc comments where appropriate
- Keep README.md up to date with crate overview and usage

### Code Style
- Follow Rust's standard formatting (use `cargo fmt`)
- Run `cargo clippy` before submitting PRs
- Use meaningful variable and function names
- Keep functions focused and small
- Use type aliases for complex types
- Prefer explicit over implicit

### Testing
- Write unit tests for all new functionality
- Include property-based tests for physics calculations
- Maintain test coverage for critical paths
- Use `#[cfg(test)]` for test-only code

### Error Handling
- Use `thiserror` for error types
- Provide meaningful error messages
- Use `?` operator for error propagation
- Document error conditions

### Performance
- Profile code before optimizing
- Use appropriate data structures
- Consider memory layout for hot paths
- Document performance characteristics

## Pull Request Process

1. Create a feature branch from `main`
2. Make your changes following the standards above
3. Add or update tests as needed
4. Update documentation
5. Run `cargo test` and `cargo clippy`
6. Submit PR with clear description of changes

## Crate Responsibilities

- `buoy_core`: Core data structures and utilities
- `buoy_physics`: Physics calculations and simulations
- `buoy_atmo`: Atmospheric modeling
- `buoy_aero`: Aerodynamic calculations
- `buoy_sim`: Simulation orchestration
- `buoy_common`: Shared utilities and types

## Questions?

Feel free to open an issue for any questions about contributing! 