#[cfg(feature = "bevy")]
use bevy::asset::Asset;
#[cfg(feature = "bevy")]
use bevy::prelude::{Bundle, Component, Name, Transform};
#[cfg(feature = "bevy")]
use bevy::reflect::TypePath;
use serde::Deserialize;

use crate::{
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

#[cfg_attr(feature = "bevy", derive(Component))]
#[cfg_attr(feature = "bevy", require(Transform))]
#[derive(Default)]
pub struct Balloon;

#[cfg_attr(feature = "bevy", derive(Asset, TypePath))]
#[derive(Deserialize, Debug)]
pub struct BalloonConfig {
    pub lift_gas_species: String,
    pub lift_gas_mass: f32, // kg
    pub balloon_mass: f32,  // kg
    pub payload_mass: f32,  // kg
    pub drag_coefficient: f32,
}

/// Bevy-free description of a balloon, computed from either defaults or a
/// [`BalloonConfig`]. The ECS adapter (`BalloonBundle`) is built from this.
pub struct BalloonSpec {
    pub lift_gas: IdealGas,
    pub shape: Shape,
    pub mass_kg: f32,
    pub drag_coefficient: f32,
    /// Height (m) above the origin to spawn the balloon at, so it starts
    /// resting on the ground rather than half-buried in it.
    pub spawn_height: f32,
}

impl Balloon {
    pub fn spec() -> BalloonSpec {
        let lift_gas = IdealGas::new(
            GasSpecies::from_species_name("helium".to_string()),
            ThermodynamicTemperature::new::<kelvin>(293.0),
            Pressure::new::<pascal>(101325.0),
            Mass::new::<kilogram>(1.0),
        );
        let radius = sphere_radius_from_volume(lift_gas.volume().get::<cubic_meter>());
        let mass_kg = lift_gas.mass.get::<kilogram>();
        BalloonSpec {
            lift_gas,
            shape: Shape::Sphere { radius },
            mass_kg,
            drag_coefficient: 0.47,
            spawn_height: radius * 2.0,
        }
    }

    pub fn spec_from_config(config: &BalloonConfig) -> BalloonSpec {
        let lift_gas = IdealGas::new(
            GasSpecies::from_species_name(config.lift_gas_species.clone()),
            ThermodynamicTemperature::new::<kelvin>(293.0),
            Pressure::new::<pascal>(101325.0),
            Mass::new::<kilogram>(config.lift_gas_mass),
        );
        let radius = sphere_radius_from_volume(lift_gas.volume().get::<cubic_meter>());
        let mass_kg = lift_gas.mass.get::<kilogram>();
        BalloonSpec {
            lift_gas,
            shape: Shape::Sphere { radius },
            mass_kg,
            drag_coefficient: config.drag_coefficient,
            spawn_height: radius * 2.0,
        }
    }

    #[cfg(feature = "bevy")]
    pub fn new() -> BalloonBundle {
        Self::spec().into()
    }

    #[cfg(feature = "bevy")]
    pub fn new_from_config(config: &BalloonConfig) -> BalloonBundle {
        Self::spec_from_config(config).into()
    }
}

#[cfg(feature = "bevy")]
#[derive(Bundle)]
pub struct BalloonBundle {
    name: Name,
    balloon: Balloon,
    lift_gas: IdealGas,
    transform: Transform,
    shape: Shape,
    mass: crate::forces::Mass,
    velocity: crate::forces::Velocity,
    drag_coefficient: crate::forces::DragCoefficient,
}

#[cfg(feature = "bevy")]
impl From<BalloonSpec> for BalloonBundle {
    fn from(spec: BalloonSpec) -> Self {
        BalloonBundle {
            name: Name::new("Balloon"),
            balloon: Balloon,
            lift_gas: spec.lift_gas,
            transform: Transform::from_xyz(0.0, spec.spawn_height, 0.0),
            shape: spec.shape,
            mass: crate::forces::Mass(spec.mass_kg),
            velocity: crate::forces::Velocity::default(),
            drag_coefficient: crate::forces::DragCoefficient(spec.drag_coefficient),
        }
    }
}
