//! Atmosphere model based on the US Standard Atmosphere, 1976.
//!
//! Reference:
//! - https://apps.dtic.mil/dtic/tr/fulltext/u2/a035728.pdf
//! - https://www.translatorscafe.com/unit-converter/en-US/calculator/altitude
//! - https://www.grc.nasa.gov/WWW/K-12/airplane/atmosmet.html

use bevy::prelude::Resource;

use tracing::{debug, error, instrument, warn};
use uom::si::{
    f32::*, length::meter, mass_density::kilogram_per_cubic_meter, pressure::kilopascal, thermodynamic_temperature::degree_celsius,
};

use buoy_physics::{
    constants::{STANDARD_PRESSURE, STANDARD_TEMPERATURE},
    ideal_gas::{GasSpecies, ideal_gas_density},
};

#[derive(Debug)]
pub enum StandardAtmosphereError {
    AltitudeOutOfBounds {
        altitude: f32,
        min: f32,
        max: f32,
    },
    TemperatureError {
        altitude: f32,
        reason: String,
    },
    PressureError {
        altitude: f32,
        reason: String,
    },
}

impl std::fmt::Display for StandardAtmosphereError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AltitudeOutOfBounds { altitude, min, max } => write!(
                f,
                "Altitude {altitude}m is out of bounds (min: {min}m, max: {max}m)"
            ),
            Self::TemperatureError { altitude, reason } => write!(
                f,
                "Invalid temperature calculation at altitude {altitude}m: {reason}"
            ),
            Self::PressureError { altitude, reason } => write!(
                f,
                "Invalid pressure calculation at altitude {altitude}m: {reason}"
            ),
        }
    }
}

impl std::error::Error for StandardAtmosphereError {}

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
    #[instrument(skip(self), fields(altitude = %altitude.get::<meter>()))]
    pub fn temperature(
        &self,
        altitude: Length,
    ) -> Result<ThermodynamicTemperature, StandardAtmosphereError> {
        let meters_above_sealevel = altitude.get::<meter>();
        debug!("Calculating temperature at altitude {}m", meters_above_sealevel);

        match self.temperature_f32(meters_above_sealevel) {
            Ok(temperature) => {
                debug!("Temperature calculated successfully: {}°C", temperature);
                Ok(ThermodynamicTemperature::new::<degree_celsius>(temperature))
            },
            Err(e) => {
                error!(error = ?e, "Failed to calculate temperature");
                Err(e)
            }
        }
    }

    /// Direct implementation of temperature from US Standard Atmosphere 1976,
    /// where altitude is expressed as a number in meters.
    ///
    /// See Equation x, Page y.
    #[instrument(skip(self), fields(altitude = meters_above_sealevel))]
    pub fn temperature_f32(
        &self,
        meters_above_sealevel: f32,
    ) -> Result<f32, StandardAtmosphereError> {
        if !self.is_valid_altitude(meters_above_sealevel) {
            warn!(
                altitude = meters_above_sealevel,
                min = Self::MIN_ALTITUDE,
                max = Self::MAX_ALTITUDE,
                "Altitude out of bounds"
            );
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
            error!(
                altitude = meters_above_sealevel,
                temperature = temperature,
                "Invalid temperature calculation"
            );
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
    #[instrument(skip(self), fields(altitude = %altitude.get::<meter>()))]
    pub fn pressure(&self, altitude: Length) -> Result<Pressure, StandardAtmosphereError> {
        let meters_above_sealevel = altitude.get::<meter>();
        debug!("Calculating pressure at altitude {}m", meters_above_sealevel);

        match self.pressure_f32(meters_above_sealevel) {
            Ok(pressure) => {
                debug!("Pressure calculated successfully: {}kPa", pressure);
                Ok(Pressure::new::<kilopascal>(pressure))
            },
            Err(e) => {
                error!(error = ?e, "Failed to calculate pressure");
                Err(e)
            }
        }
    }

    /// Direct implementation of pressure from US Standard Atmosphere 1976,
    /// where altitude is expressed as a number in meters.
    ///
    /// See Equation x, Page y.
    #[instrument(skip(self), fields(altitude = meters_above_sealevel))]
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
            error!(
                altitude = meters_above_sealevel,
                pressure = pressure,
                "Invalid pressure calculation"
            );
            return Err(StandardAtmosphereError::PressureError {
                altitude: meters_above_sealevel,
                reason: "Calculation resulted in invalid pressure".to_string(),
            });
        }

        Ok(pressure)
    }

    /// Density (kg/m³) of the atmosphere at an altitude above sea-level.
    #[instrument(skip(self), fields(altitude = %altitude.get::<meter>()))]
    pub fn density(&self, altitude: Length) -> Result<MassDensity, StandardAtmosphereError> {
        debug!("Calculating density at altitude {}m", altitude.get::<meter>());

        if let (Ok(temperature), Ok(pressure)) =
            (self.temperature(altitude), self.pressure(altitude))
        {
            let density = ideal_gas_density(temperature, pressure, &GasSpecies::air());
            debug!("Density calculated successfully: {}kg/m³", density.get::<kilogram_per_cubic_meter>());
            Ok(density)
        } else {
            warn!(
                altitude = altitude.get::<meter>(),
                min = Self::MIN_ALTITUDE,
                max = Self::MAX_ALTITUDE,
                "Altitude out of bounds for density calculation"
            );
            Err(StandardAtmosphereError::AltitudeOutOfBounds {
                altitude: altitude.get::<meter>(),
                min: Self::MIN_ALTITUDE,
                max: Self::MAX_ALTITUDE,
            })
        }
    }

    /// Atmospheric density at Standard Temperature and Pressure (STP)
    #[instrument(
        skip_all,
        fields(
            module = "density",
            condition = "STP",
            unit = "kg/m³"
        )
    )]
    pub fn density_at_stp() -> MassDensity {
        let density = ideal_gas_density(
            STANDARD_TEMPERATURE.clone(),
            STANDARD_PRESSURE.clone(),
            &GasSpecies::air(),
        );
        debug!(
            "STP density calculation successful: density = {} {}",
            density.get::<kilogram_per_cubic_meter>(),
            "kg/m³"
        );
        density
    }
}
