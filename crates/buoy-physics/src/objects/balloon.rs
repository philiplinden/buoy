use bevy::asset::Asset;
use bevy::prelude::*;
use bevy::reflect::TypePath;
use serde::Deserialize;

use crate::{
    forces::{DragCoefficient, Mass as BodyMass, Velocity},
    geometry::{sphere_radius_from_volume, Shape},
    ideal_gas::{GasSpecies, IdealGas},
};
use uom::si::{
    f32::{Mass, Pressure, ThermodynamicTemperature},
    thermodynamic_temperature::kelvin,
    pressure::pascal,
    mass::kilogram,
    volume::cubic_meter,
};

#[derive(Component, Default)]
#[require(Transform)]
pub struct Balloon;

#[derive(Deserialize, Debug, Asset, TypePath)]
pub struct BalloonConfig {
    pub lift_gas_species: String,
    pub lift_gas_mass: f32, // kg
    pub balloon_mass: f32,  // kg
    pub payload_mass: f32,  // kg
    pub drag_coefficient: f32,
}

impl Balloon {
    pub fn new() -> BalloonBundle {
        let balloon = Balloon;
        let lift_gas = IdealGas::new(
            GasSpecies::from_species_name("helium".to_string()),
            ThermodynamicTemperature::new::<kelvin>(293.0),
            Pressure::new::<pascal>(101325.0),
            Mass::new::<kilogram>(1.0),
        );
        let radius = sphere_radius_from_volume(lift_gas.volume().get::<cubic_meter>());
        let mass_kg = lift_gas.mass.get::<kilogram>();
        BalloonBundle {
            name: Name::new("Balloon"),
            balloon,
            lift_gas,
            transform: Transform::from_xyz(0.0, radius * 2.0, 0.0),
            shape: Shape::Sphere { radius },
            mass: BodyMass(mass_kg),
            velocity: Velocity::default(),
            drag_coefficient: DragCoefficient(0.47),
        }
    }
    pub fn new_from_config(config: &BalloonConfig) -> BalloonBundle {
        let balloon = Balloon;
        let lift_gas = IdealGas::new(
            GasSpecies::from_species_name(config.lift_gas_species.clone()),
            ThermodynamicTemperature::new::<kelvin>(293.0),
            Pressure::new::<pascal>(101325.0),
            Mass::new::<kilogram>(config.lift_gas_mass),
        );
        let radius = sphere_radius_from_volume(lift_gas.volume().get::<cubic_meter>());
        let mass_kg = lift_gas.mass.get::<kilogram>();
        BalloonBundle {
            name: Name::new("Balloon"),
            balloon,
            lift_gas,
            transform: Transform::from_xyz(0.0, radius * 2.0, 0.0),
            shape: Shape::Sphere { radius },
            mass: BodyMass(mass_kg),
            velocity: Velocity::default(),
            drag_coefficient: DragCoefficient(config.drag_coefficient),
        }
    }
}

#[derive(Bundle)]
pub struct BalloonBundle {
    name: Name,
    balloon: Balloon,
    lift_gas: IdealGas,
    transform: Transform,
    shape: Shape,
    mass: BodyMass,
    velocity: Velocity,
    drag_coefficient: DragCoefficient,
}
