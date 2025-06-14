use std::fmt::Display;
use bevy::prelude::*;
use uom::si::{f32::*, Quantity};

pub(crate) fn plugin(app: &mut App) {
    app.register_type::<UomQuantity>();
}

#[derive(Component, Debug, Reflect)]
pub struct UomQuantity {
    value: f32,
    unit: String,
}

impl UomQuantity {
    pub fn new<D, U, V>(quantity: &Quantity<D, U, V>) -> Self
    where
        D: uom::si::Dimension + ?Sized,
        U: uom::si::Units<V> + ?Sized + uom::si::Unit,
        V: uom::num::Num + uom::Conversion<V> + Into<f32> + Clone,
    {
        Self {
            value: quantity.value.clone().into(),
            unit: U::abbreviation().to_string(),
        }
    }

    /// Get the raw value of the quantity
    pub fn value(&self) -> f32 {
        self.value
    }

    /// Get the unit abbreviation
    pub fn unit(&self) -> &str {
        &self.unit
    }
}

impl Display for UomQuantity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.value, self.unit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uom::si::{
        length::{kilometer, meter},
        mass::{kilogram, gram},
        pressure::{kilopascal, pascal},
        thermodynamic_temperature::{degree_celsius, kelvin},
        time::{hour, second},
        velocity::{kilometer_per_hour, meter_per_second},
    };

    #[test]
    fn test_length_formatting() {
        let length = Length::new::<meter>(5.0);
        let display = UomQuantity::new(&length);
        assert_eq!(display.to_string(), "5.0 m");
        assert_eq!(display.value(), 5.0);
        assert_eq!(display.unit(), "m");

        let length = Length::new::<kilometer>(2.5);
        let display = UomQuantity::new(&length);
        assert_eq!(display.to_string(), "2.5 km");
    }

    #[test]
    fn test_mass_formatting() {
        let mass = Mass::new::<kilogram>(75.0);
        let display = UomQuantity::new(&mass);
        assert_eq!(display.to_string(), "75.0 kg");
        assert_eq!(display.value(), 75.0);
        assert_eq!(display.unit(), "kg");

        let mass = Mass::new::<gram>(500.0);
        let display = UomQuantity::new(&mass);
        assert_eq!(display.to_string(), "500.0 g");
    }

    #[test]
    fn test_pressure_formatting() {
        let pressure = Pressure::new::<kilopascal>(101.325);
        let display = UomQuantity::new(&pressure);
        assert_eq!(display.to_string(), "101.325 kPa");
        assert_eq!(display.value(), 101.325);
        assert_eq!(display.unit(), "kPa");

        let pressure = Pressure::new::<pascal>(101325.0);
        let display = UomQuantity::new(&pressure);
        assert_eq!(display.to_string(), "101325.0 Pa");
    }

    #[test]
    fn test_temperature_formatting() {
        let temp = ThermodynamicTemperature::new::<degree_celsius>(20.0);
        let display = UomQuantity::new(&temp);
        assert_eq!(display.to_string(), "20.0 °C");
        assert_eq!(display.value(), 20.0);
        assert_eq!(display.unit(), "°C");

        let temp = ThermodynamicTemperature::new::<kelvin>(293.15);
        let display = UomQuantity::new(&temp);
        assert_eq!(display.to_string(), "293.15 K");
    }

    #[test]
    fn test_velocity_formatting() {
        let velocity = Velocity::new::<meter_per_second>(10.0);
        let display = UomQuantity::new(&velocity);
        assert_eq!(display.to_string(), "10.0 m/s");
        assert_eq!(display.value(), 10.0);
        assert_eq!(display.unit(), "m/s");

        let velocity = Velocity::new::<kilometer_per_hour>(36.0);
        let display = UomQuantity::new(&velocity);
        assert_eq!(display.to_string(), "36.0 km/h");
    }

    #[test]
    fn test_time_formatting() {
        let time = Time::new::<second>(60.0);
        let display = UomQuantity::new(&time);
        assert_eq!(display.to_string(), "60.0 s");
        assert_eq!(display.value(), 60.0);
        assert_eq!(display.unit(), "s");

        let time = Time::new::<hour>(1.5);
        let display = UomQuantity::new(&time);
        assert_eq!(display.to_string(), "1.5 h");
    }

    #[test]
    fn test_negative_values() {
        let temp = ThermodynamicTemperature::new::<degree_celsius>(-20.0);
        let display = UomQuantity::new(&temp);
        assert_eq!(display.to_string(), "-20.0 °C");
        assert_eq!(display.value(), -20.0);
    }

    #[test]
    fn test_small_values() {
        let pressure = Pressure::new::<pascal>(0.001);
        let display = UomQuantity::new(&pressure);
        assert_eq!(display.to_string(), "0.001 Pa");
        assert_eq!(display.value(), 0.001);
    }

    #[test]
    fn test_large_values() {
        let length = Length::new::<kilometer>(1000.0);
        let display = UomQuantity::new(&length);
        assert_eq!(display.to_string(), "1000.0 km");
        assert_eq!(display.value(), 1000.0);
    }
}
