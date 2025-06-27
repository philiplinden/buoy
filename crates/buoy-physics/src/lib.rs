#![allow(unused_imports)]
pub mod atmosphere;
pub mod constants;
pub mod forces;
pub mod geometry;
pub mod ideal_gas;
// pub mod mesh_utils;
pub mod objects;

#[cfg(feature = "grid_space")]
pub mod grid;

pub mod prelude {
    pub use crate::{
        atmosphere::Atmosphere,
        forces::{drag, scale_gravity},
        ideal_gas::{GasSpecies, IdealGas},
    };
}

use avian3d::prelude::{PhysicsInterpolationPlugin, PhysicsPlugins, PhysicsSet};
use bevy::{
    app::{PluginGroup, PluginGroupBuilder},
    prelude::*,
};

pub struct BuoyPhysicsPlugin;

impl Plugin for BuoyPhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            PhysicsPlugins::default(),
            atmosphere::plugin,
            ideal_gas::plugin,
            forces::plugin,
            // mesh_utils::plugin,
            #[cfg(feature = "grid_space")]
            grid::plugin,
        ));
    }
}
