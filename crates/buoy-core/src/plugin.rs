use super::*;
use bevy::{app::PluginGroupBuilder, prelude::*};

/// A custom flavor of Bevy's DefaultPlugins that includes common plugins used by Buoy.
pub struct BuoyDefaultPlugins;

impl PluginGroup for BuoyDefaultPlugins {
    fn build(self) -> PluginGroupBuilder {
        let mut builder = PluginGroupBuilder::start::<Self>().add(sequencing::plugin).add(config::plugin);

        // Dev tools
        #[cfg(feature = "dev")]
        {
            builder = builder.add_group(dev_tools::BuoyDevTools);
        }

        builder
    }
}
