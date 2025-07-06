pub mod controls;
pub mod format;
pub mod scene;
pub mod sequencing;
pub mod objects;

#[cfg(feature = "render")]
pub mod render;

use bevy::{app::PluginGroupBuilder, prelude::*};

/// A custom flavor of Bevy's DefaultPlugins that includes common plugins used by Buoy.
pub struct BuoyDefaultPlugins;

impl PluginGroup for BuoyDefaultPlugins {
    fn build(self) -> PluginGroupBuilder {
        let mut group = PluginGroupBuilder::start::<Self>().add_group(DefaultPlugins);

        group = group
            .add(RuntimePlugin)
            .add(sequencing::plugin)
            .add(controls::plugin);

        // Creature comforts
        group = group.add(format::PrettyPrintPlugin);

        // configure headless rendering if gui feature is disabled
        #[cfg(not(feature = "gui"))]
        {
            group = group.add(bevy::app::ScheduleRunnerPlugin::run_loop(
                std::time::Duration::from_secs_f64(1.0 / 60.0),
            ));
        }
        // configure the window in default plugins if gui feature is enabled
        #[cfg(feature = "gui")]
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

struct RuntimePlugin;

impl Plugin for RuntimePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<RuntimeState>();
    }
}

#[derive(States, Debug, Default, Clone, Copy, Hash, PartialEq, Eq)]
pub enum RuntimeState {
    Stopped,
    #[default]
    Running,
    Faulted,
}
