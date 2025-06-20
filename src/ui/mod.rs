mod controls;
mod colors;
mod camera;
mod lighting;

#[cfg(feature = "debug")]
mod debug;

use bevy::prelude::*;

pub(crate) fn plugin(app: &mut App) {
    app.add_plugins((
        controls::plugin,
        camera::plugin,
        lighting::plugin,
    ));

    #[cfg(feature = "debug")]
    app.add_plugins(debug::plugin);
}
