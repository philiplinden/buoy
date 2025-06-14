# Buoy Common

Common utilities for buoy simulation. This crate provides shared data structures, unit conversions, math utilities, common traits, and error handling for the entire project.

## Features

- Shared data structures
- Unit conversion utilities
- Math utilities
- Common traits
- Error handling

## Usage

```rust
use buoy_common::{UnitConverter, Error};

// Convert units
let converter = UnitConverter::new();
let meters = converter.convert(100.0, Unit::Feet, Unit::Meters);

// Handle errors
let result = some_operation().map_err(Error::from);
```

## Dependencies

- `serde`: Serialization support

## Testing

The crate includes comprehensive unit tests for all utilities, validated against known solutions and experimental data.

## Contributing

When adding new utilities:
1. Include mathematical derivation in documentation
2. Add unit tests with known solutions
3. Benchmark performance impact
4. Update this README with new features 