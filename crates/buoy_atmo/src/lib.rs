#![cfg(feature = "us_std_1976")]
pub mod us_std_1976;

use bevy::prelude::*;

pub struct AtmospherePlugin;

impl Plugin for AtmospherePlugin {
    fn build(&self, app: &mut App) {
        #![cfg(feature = "us_std_1976")]
        app.insert_resource(us_std_1976::StandardAtmosphere1976);
    }
}