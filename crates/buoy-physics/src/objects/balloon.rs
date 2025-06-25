use crate::mesh_utils::{RestState, Strain, create_icosphere_mesh};
use bevy::prelude::*;

#[derive(Component)]
#[require(Transform)]
pub struct Balloon;
