pub mod constants;
pub mod geometry;
pub mod runtime;
pub mod units;
// pub mod config;

use bevy::{app::PluginGroupBuilder, prelude::*};

#[derive(Default)]
pub struct BuoyCommonPlugins;

impl PluginGroup for BuoyCommonPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(units::FormattedUnitsPlugin)
            .add(runtime::PausableSystemsPlugin)
    }
}
