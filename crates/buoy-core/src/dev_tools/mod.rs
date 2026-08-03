mod commands;

use bevy::{app::PluginGroupBuilder, prelude::*};
use bevy_repl::prelude::*;

pub struct BuoyDevTools;

impl PluginGroup for BuoyDevTools {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add_group(ReplPlugins)
            .add(commands::plugin)
    }
}
