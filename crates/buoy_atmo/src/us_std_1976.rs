//! Atmosphere model based on the US Standard Atmosphere, 1976.
//!
//! Reference:
//! - https://apps.dtic.mil/dtic/tr/fulltext/u2/a035728.pdf
//! - https://www.translatorscafe.com/unit-converter/en-US/calculator/altitude
//! - https://www.grc.nasa.gov/WWW/K-12/airplane/atmosmet.html

use bevy::prelude::Resource;
use thiserror::Error;
use uom::si::{
    f32::*, length::meter, pressure::kilopascal, thermodynamic_temperature::degree_celsius,
};

use buoy_core::{
    constants::{STANDARD_PRESSURE, STANDARD_TEMPERATURE},
    ideal_gas::{GasSpecies, ideal_gas_density},
};

#[derive(Error, Debug)]
pub enum StandardAtmosphereError {
    #[error("Altitude {altitude}m is out of bounds (min: {min}m, max: {max}m)")]
    AltitudeOutOfBounds {
        altitude: f32,
        min: f32,
        max: f32,
    },
    #[error("Invalid temperature calculation at altitude {altitude}m: {reason}")]
    TemperatureError {
        altitude: f32,
        reason: String,
    },
    #[error("Invalid pressure calculation at altitude {altitude}m: {reason}")]
    PressureError {
        altitude: f32,
        reason: String,
    },
}

/// US Standard Atmosphere, 1976
///
/// The ideal, steady-state, density vs. height profile of air.
///
/// "A hypothetical vertical distribution of atmospheric temperature, pressure
/// and density which, by international agreement, is roughly representative of
/// year-round, midlatitude conditions. [...] The air is assumed to obey the
/// perfect gas law and hydrostatic equation which, taken together, relate
/// temperature, pressure and density with geopotential. [...]
///
/// [The model atmosphere is] considered to rotate with the earth and be an
/// average over the diurnal cycle, semi-annual variation, and the range of
/// conditions from active to quiet geomagnetic and active to quiet sunspot
/// conditions. Above the turbopause (about 110 km) generalized forms of the
/// hydrostatic equations apply."
///
/// The model expresses the temperature-height profile for altitudes below 86 km
/// as linear segments that join smoothly at the mesopause to a continuous curve
/// through the thermosphere.
#[derive(Resource)]
pub struct StandardAtmosphere1976;

impl StandardAtmosphere1976 {
    pub const MAX_ALTITUDE: f32 = 85000.0; // meters above sea-level
    pub const MIN_ALTITUDE: f32 = -56.0; // meters above sea-level

    #[inline]
    fn is_valid_altitude(&self, altitude: f32) -> bool {
        (Self::MIN_ALTITUDE..=Self::MAX_ALTITUDE).contains(&altitude)
    }

    /// Temperature of the atmosphere at an altitude above sea-level based on
    /// US Standard Atmosphere 1976, where altitude is expressed as a [`Quantity`].
    ///
    /// This function wraps a direct implementation of the model
    /// ([`StandardAtmosphere::temperature_f32`]) with flexible unit quantities
    /// for the inputs and outputs.
    pub fn temperature(
        &self,
        altitude: Length,
    ) -> Result<ThermodynamicTemperature, StandardAtmosphereError> {
        let meters_above_sealevel = altitude.get::<meter>();
        match self.temperature_f32(meters_above_sealevel) {
            Ok(temperature) => Ok(ThermodynamicTemperature::new::<degree_celsius>(temperature)),
            Err(e) => Err(e),
        }
    }

    /// Direct implementation of temperature from US Standard Atmosphere 1976,
    /// where altitude is expressed as a number in meters.
    ///
    /// See Equation x, Page y.
    pub fn temperature_f32(
        &self,
        meters_above_sealevel: f32,
    ) -> Result<f32, StandardAtmosphereError> {
        if !self.is_valid_altitude(meters_above_sealevel) {
            return Err(StandardAtmosphereError::AltitudeOutOfBounds {
                altitude: meters_above_sealevel,
                min: Self::MIN_ALTITUDE,
                max: Self::MAX_ALTITUDE,
            });
        }

        let temperature = match meters_above_sealevel {
            alt if (-57.0..11000.0).contains(&alt) => {
                15.04 - 0.00649 * alt
            }
            alt if (11000.0..25000.0).contains(&alt) => {
                -56.46
            }
            alt if (25000.0..85000.0).contains(&alt) => {
                -131.21 + 0.00299 * alt
            }
            _ => unreachable!(), // We've already checked bounds
        };

        if temperature.is_nan() || temperature.is_infinite() {
            return Err(StandardAtmosphereError::TemperatureError {
                altitude: meters_above_sealevel,
                reason: "Calculation resulted in invalid temperature".to_string(),
            });
        }

        Ok(temperature)
    }

    /// Pressure of the atmosphere at an altitude above sea-level based on
    /// US Standard Atmosphere 1976, where altitude is expressed as a [`Quantity`].
    ///
    /// This function wraps a direct implementation of the model
    /// ([`StandardAtmosphere::pressure_f32`]) with flexible unit quantities
    /// for the inputs and outputs.
    pub fn pressure(&self, altitude: Length) -> Result<Pressure, StandardAtmosphereError> {
        let meters_above_sealevel = altitude.get::<meter>();
        match self.pressure_f32(meters_above_sealevel) {
            Ok(pressure) => Ok(Pressure::new::<kilopascal>(pressure)),
            Err(e) => Err(e),
        }
    }

    /// Direct implementation of pressure from US Standard Atmosphere 1976,
    /// where altitude is expressed as a number in meters.
    ///
    /// See Equation x, Page y.
    pub fn pressure_f32(&self, meters_above_sealevel: f32) -> Result<f32, StandardAtmosphereError> {
        let temperature = self.temperature_f32(meters_above_sealevel)?;
        
        let pressure = match meters_above_sealevel {
            alt if (-57.0..11000.0).contains(&alt) => {
                101.29 * f32::powf(temperature / 288.08, 5.256)
            }
            alt if (11000.0..25000.0).contains(&alt) => {
                22.65 * f32::exp(1.73 - 0.000157 * alt)
            }
            alt if (25000.0..85000.0).contains(&alt) => {
                2.488 * f32::powf(temperature / 216.6, -11.388)
            }
            _ => unreachable!(), // We've already checked bounds
        };

        if pressure.is_nan() || pressure.is_infinite() || pressure <= 0.0 {
            return Err(StandardAtmosphereError::PressureError {
                altitude: meters_above_sealevel,
                reason: "Calculation resulted in invalid pressure".to_string(),
            });
        }

        Ok(pressure)
    }

    /// Density (kg/m³) of the atmosphere at an altitude above sea-level.
    pub fn density(&self, altitude: Length) -> Result<MassDensity, StandardAtmosphereError> {
        if let (Ok(temperature), Ok(pressure)) =
            (self.temperature(altitude), self.pressure(altitude))
        {
            Ok(ideal_gas_density(temperature, pressure, &GasSpecies::air()))
        } else {
            Err(StandardAtmosphereError::AltitudeOutOfBounds {
                altitude: altitude.get::<meter>(),
                min: Self::MIN_ALTITUDE,
                max: Self::MAX_ALTITUDE,
            })
        }
    }

    /// Atmospheric density at Standard Temperature and Pressure (STP)
    pub fn density_at_stp() -> MassDensity {
        ideal_gas_density(
            STANDARD_TEMPERATURE.clone(),
            STANDARD_PRESSURE.clone(),
            &GasSpecies::air(),
        )
    }
}
