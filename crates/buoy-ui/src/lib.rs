mod controls;
mod camera;
mod lighting;
mod scene;

#[cfg(feature = "dev")]
mod debug;

use bevy::prelude::*;

pub struct BuoyUiPlugin;

impl Plugin for BuoyUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            controls::plugin,
            camera::plugin,
            lighting::plugin,
            scene::plugin,
        ));

    #[cfg(feature = "dev")]
    app.add_plugins(debug::plugin);

    #[cfg(feature = "inspect")]
    app.add_plugins(WorldInspectorPlugin::new());

    }
}
