use bevy::{app::PluginGroupBuilder, prelude::*};

/// A custom flavor of Bevy's DefaultPlugins that includes common plugins used by Buoy.
pub struct BuoyDefaultPlugins;

impl PluginGroup for BuoyDefaultPlugins {
    fn build(self) -> PluginGroupBuilder {
        let mut group = PluginGroupBuilder::start::<Self>().add_group(DefaultPlugins);

        group = group.add(CommonStatesPlugin).add(CommonTypesPlugin);

        // disable TransformPlugin if grid_space feature is enabled
        #[cfg(feature = "grid_space")]
        {
            // big_space requires TransformPlugin to be disabled
            group = group.disable::<TransformPlugin>();
        }

        // configure headless rendering if window feature is disabled
        #[cfg(not(feature = "window"))]
        {
            group = group
                .disable::<bevy_render::RenderPlugin>()
                .add(bevy_render::mesh::MeshPlugin)
                .add(bevy::app::ScheduleRunnerPlugin::run_loop(
                    std::time::Duration::from_secs_f64(1.0 / 60.0),
                ));
        }
        // configure the window if window feature is enabled
        #[cfg(feature = "window")]
        {
            group = group.set(bevy::window::WindowPlugin {
                primary_window: bevy::window::Window {
                    title: "buoy 🛟".to_string(),
                    canvas: Some("#bevy".to_string()),
                    fit_canvas_to_parent: true,
                    prevent_default_event_handling: true,
                    ..default()
                }
                .into(),
                ..default()
            });
        }
        group
    }
}

/// A plugin that registers common states used by Buoy.
pub struct CommonStatesPlugin;

impl Plugin for CommonStatesPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<super::states::SimState>();
    }
}

/// A plugin that registers common types used by Buoy.
pub struct CommonTypesPlugin;

impl Plugin for CommonTypesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<super::types::UomQuantity>();
    }
}
