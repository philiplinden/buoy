#![allow(unused_imports)]
// Support configuring Bevy lints within code.
#![cfg_attr(bevy_lint, feature(register_tool), register_tool(bevy))]
// Disable console on Windows for non-dev builds.
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

//! Core data structures and utilities for the Buoy project.
//! 
//! This crate provides fundamental data structures and utilities used across the project:
//! - Configuration management
//! - Geometry primitives
//! - Mesh operations
//! - Data formatting utilities

pub mod config;
pub mod core;
pub mod format;
pub mod geometry;
pub mod mesh_drag;

pub use config::*;
pub use core::*;
pub use format::*;
pub use geometry::*;
pub use mesh_drag::*;

pub mod constants;
pub mod ideal_gas;
pub mod forces;
pub mod material_properties;

pub mod prelude {
    pub use crate::{
        core::{BuoyPhysicsPlugin, BuoySystemsPlugin},
        forces::{drag, scale_gravity},
        ideal_gas::{GasSpecies, IdealGas},
    };
}
