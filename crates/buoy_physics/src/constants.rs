#![allow(dead_code)]
//! Pre-computed constants in SI units.
//!
//! The purpose of this module is for convenience and neatness. Rather than
//! requiring other modules to find and instantiate the correct unuts for a
//! given constant, we can just use these.
//!
//! All constants are computed using the `uom` crate and support conversion.
//!
//! TODO: Add an option to support f64 features that are available in the crates
//! that `buoy-core` depends on.

use std::sync::LazyLock;
use uom::si::{
    acceleration::standard_gravity, f32::*, heat_capacity::boltzmann_constant, length::meter,
    molar_heat_capacity::molar_gas_constant, pressure::pascal, thermodynamic_temperature::kelvin,
};

/// The mathematical constant pi.
pub static PI: f32 = std::f32::consts::PI;

/// The Boltzmann constant in J/K.
pub static BOLTZMANN_CONSTANT: LazyLock<HeatCapacity> =
    LazyLock::new(|| HeatCapacity::new::<boltzmann_constant>(1.0));

/// The molar gas constant in J/(mol·K).
pub static GAS_CONSTANT: LazyLock<MolarHeatCapacity> =
    LazyLock::new(|| MolarHeatCapacity::new::<molar_gas_constant>(1.0));

/// The standard gravity on Earth in m/s².
pub static STANDARD_GRAVITY: LazyLock<Acceleration> =
    LazyLock::new(|| Acceleration::new::<standard_gravity>(1.0));

/// The temperature part of Standard Temperature and Pressure (STP).
pub static STANDARD_TEMPERATURE: LazyLock<ThermodynamicTemperature> =
    LazyLock::new(|| ThermodynamicTemperature::new::<kelvin>(273.15));

/// The pressure part of Standard Temperature and Pressure (STP).
pub static STANDARD_PRESSURE: LazyLock<Pressure> =
    LazyLock::new(|| Pressure::new::<pascal>(101325.0));

/// The radius of the Earth in meters.
pub static EARTH_RADIUS_M: LazyLock<Length> = LazyLock::new(|| Length::new::<meter>(6371007.2));

/// The scale factor for translations. This is used to convert between
/// simulation units and world units. One unit in the simulation is this
/// dimension in the real world.
pub const TRANSLATION_SCALE: LazyLock<Length> = LazyLock::new(|| Length::new::<meter>(1.0));
