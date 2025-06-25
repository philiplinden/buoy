#![allow(unused_imports)]
pub mod atmosphere;
pub mod constants;
pub mod forces;
pub mod geometry;
pub mod ideal_gas;
pub mod time;
pub mod mesh_utils;
pub mod objects;

#[cfg(feature = "grid_space")]
pub mod fluid_volume;
#[cfg(feature = "grid_space")]
pub mod grid;

pub mod prelude {
    pub use crate::{
        atmosphere::Atmosphere,
        forces::{drag, scale_gravity},
        ideal_gas::{GasSpecies, IdealGas},
        mesh_utils,
    };
}

#[cfg(feature = "grid_space")]
pub use fluid_volume::{FluidVolumeBuilder, FluidVolumeCell, DefaultFluidVolumeSettings};

use avian3d::prelude::{PhysicsInterpolationPlugin, PhysicsPlugins, PhysicsSet};
use bevy::{
    app::{PluginGroup, PluginGroupBuilder},
    prelude::*,
};

pub struct BuoyPhysicsPlugin;

impl Plugin for BuoyPhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            PhysicsPlugins::default().set(PhysicsInterpolationPlugin::interpolate_all()),
            atmosphere::plugin,
            ideal_gas::plugin,
            forces::plugin,
            time::plugin,
            mesh_utils::plugin,
            #[cfg(feature = "grid_space")]
            fluid_volume::plugin,
            #[cfg(feature = "grid_space")]
            grid::plugin,
        ));
    }
}
