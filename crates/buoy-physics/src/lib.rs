#![allow(unused_imports)]
pub mod atmosphere;
pub mod constants;
pub mod core;
pub mod forces;
pub mod geometry;
pub mod ideal_gas;
pub mod time;

#[cfg(feature = "grid_space")]
pub mod fluid_volume;
#[cfg(feature = "grid_space")]
pub mod grid;

pub mod prelude {
    pub use crate::{
        atmosphere::Atmosphere,
        forces::{drag, scale_gravity},
        ideal_gas::{GasSpecies, IdealGas},
    };
    pub use uom::si::{
        f32::{Mass, MassDensity, MolarMass, Pressure, ThermodynamicTemperature, Volume},
        mass::kilogram,
        mass_density::kilogram_per_cubic_meter,
        molar_mass::kilogram_per_mole,
        pressure::pascal,
        thermodynamic_temperature::kelvin,
        volume::cubic_meter,
    };
}

#[cfg(feature = "grid_space")]
pub use fluid_volume::{FluidVolumeBuilder, FluidVolumeCell, DefaultFluidVolumeSettings};

pub use core::BuoyPhysicsPlugin;
