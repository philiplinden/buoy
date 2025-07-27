//! Ideal gas equations.
#![allow(dead_code)]

use std::ops::{Div, Mul};

use avian3d::math::Scalar;
use bevy::{asset::Asset, prelude::*, reflect::TypePath};
use serde::Deserialize;
use uom::si::{
    f32::{
        ThermodynamicTemperature, Pressure, Mass, Volume,
        MassDensity, MolarMass
    },
    thermodynamic_temperature::kelvin,
    pressure::pascal,
    mass::kilogram,
    volume::cubic_meter,
    mass_density::kilogram_per_cubic_meter,
    molar_mass::kilogram_per_mole,
};

use crate::{
    constants::{GAS_CONSTANT, STANDARD_GRAVITY},
    geometry::sphere_volume,
};

pub(crate) fn plugin(_app: &mut App) {
    // nothing yet
}

/// Volume (m³) of an ideal gas from its temperature (K), pressure (Pa),
/// mass (kg) and molar mass (kg/mol).
pub fn ideal_gas_volume(
    temperature: ThermodynamicTemperature,
    pressure: Pressure,
    mass: Mass,
    species: &GasSpecies,
) -> Volume {
    (mass / species.molar_mass) * *GAS_CONSTANT * temperature / pressure
}

/// Density (kg/m³) of an ideal gas from its temperature (K), pressure (Pa),
/// and molar mass (kg/mol)
pub fn ideal_gas_density(
    temperature: ThermodynamicTemperature,
    pressure: Pressure,
    species: &GasSpecies,
) -> MassDensity {
    species.molar_mass * pressure / (*GAS_CONSTANT * temperature)
}

/// Molecular species of a gas.
#[derive(Component, Debug, Clone, PartialEq)]
pub struct GasSpecies {
    pub name: String,
    pub abbreviation: String,
    pub molar_mass: MolarMass, // [kg/mol] molar mass a.k.a. molecular weight
}

impl GasSpecies {
    /// Dry air.
    pub fn air() -> Self {
        GasSpecies {
            name: "Air".to_string(),
            abbreviation: "AIR".to_string(),
            molar_mass: MolarMass::new::<kilogram_per_mole>(0.0289647),
        }
    }

    pub fn helium() -> Self {
        GasSpecies {
            name: "Helium".to_string(),
            abbreviation: "He".to_string(),
            molar_mass: MolarMass::new::<kilogram_per_mole>(0.0040026),
        }
    }

    pub fn new(name: String, abbreviation: String, molar_mass: MolarMass) -> Self {
        GasSpecies {
            name,
            abbreviation,
            molar_mass,
        }
    }

    pub fn from_species_name(name: String) -> Self {
        match name.as_str() {
            "air" => GasSpecies::air(),
            "helium" => GasSpecies::helium(),
            _ => panic!("Unknown gas species: {}", name),
        }
    }
}

impl Default for GasSpecies {
    fn default() -> Self {
        GasSpecies::helium()
    }
}

#[derive(Deserialize, Debug, Asset, TypePath, Clone)]
pub struct GasSpeciesConfig {
    pub name: String,
    pub abbreviation: String,
    pub molar_mass: f32, // [kg/mol]
}

impl GasSpeciesConfig {
    pub fn to_species(&self) -> GasSpecies {
        GasSpecies {
            name: self.name.clone(),
            abbreviation: self.abbreviation.clone(),
            molar_mass: MolarMass::new::<kilogram_per_mole>(self.molar_mass),
        }
    }
}

#[derive(Deserialize, Debug, Asset, TypePath, Clone)]
pub struct GasPropertiesConfig {
    pub gases: Vec<GasSpeciesConfig>,
    // materials: Vec<MaterialConfig>, // can be added later
}

/// Properties of an ideal gas per unit mass.
#[derive(Component, Default, Debug, Clone, PartialEq)]
pub struct IdealGas {
    pub species: GasSpecies,
    pub mass: Mass,
    pub temperature: ThermodynamicTemperature,
    pub pressure: Pressure,
}

impl IdealGas {
    pub fn new(
        species: GasSpecies,
        temperature: ThermodynamicTemperature,
        pressure: Pressure,
        mass: Mass,
    ) -> Self {
        IdealGas {
            species,
            temperature,
            pressure,
            mass,
        }
    }

    pub fn volume(&self) -> Volume {
        ideal_gas_volume(self.temperature, self.pressure, self.mass, &self.species)
    }

    pub fn density(&self) -> MassDensity {
        ideal_gas_density(self.temperature, self.pressure, &self.species)
    }

    pub fn with_mass(self, mass: f32) -> Self {
        Self {
            mass: Mass::new::<kilogram>(mass),
            ..self
        }
    }
}
