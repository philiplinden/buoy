use bevy::{app::PluginGroupBuilder, prelude::*};

pub struct BuoyDefaultPlugins;

impl PluginGroup for BuoyDefaultPlugins {
    fn build(self) -> PluginGroupBuilder {
        // initialize the default plugins before modifying them
        let mut group = PluginGroupBuilder::start::<DefaultPlugins>();

        // add our own plugins
        group = group
            .add(CommonStatesPlugin)
            .add(CommonTypesPlugin);

        // disable TransformPlugin if grid_space feature is enabled
        #[cfg(feature = "grid_space")]
        {
            // big_space requires TransformPlugin to be disabled
            group = group.disable::<TransformPlugin>();
        }
        group
    }
}

pub struct CommonStatesPlugin;

impl Plugin for CommonStatesPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<super::states::SimState>();
    }
}

pub struct CommonTypesPlugin;

impl Plugin for CommonTypesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<super::types::UomQuantity>();
    }
}
