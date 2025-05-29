#![allow(unused_imports)]
// Support configuring Bevy lints within code.
#![cfg_attr(bevy_lint, feature(register_tool), register_tool(bevy))]
// Disable console on Windows for non-dev builds.
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

pub mod constants;
pub mod geometry;
pub mod ideal_gas;
pub mod atmosphere;
pub mod forces;
pub mod mesh_drag;
pub mod material_properties;
pub mod balloon;
pub mod core;
pub mod format;


pub mod prelude {
    pub use crate::{
        atmosphere::Atmosphere,
        core::{BuoyPlugin, SimState},
        forces::{drag, scale_gravity},
        ideal_gas::{GasSpecies, IdealGas},
        balloon::{Balloon, BalloonPhysics},
    };
    pub use uom::si as units;
}
