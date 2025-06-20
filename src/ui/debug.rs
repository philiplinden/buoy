use avian3d::debug_render::PhysicsDebugPlugin;
use bevy::{
    dev_tools::fps_overlay::FpsOverlayPlugin,
    pbr::wireframe::{WireframeConfig, WireframePlugin},
    prelude::*,
};

use super::colors::ColorPalette;

pub(crate) fn plugin(app: &mut App) {
    app.add_plugins((
        PhysicsDebugPlugin::default(),               // Draws colliders
        WireframePlugin::default(),
        FpsOverlayPlugin::default(),
    ));
    app.insert_resource(WireframeConfig {
        // The global wireframe config enables drawing of wireframes on every mesh,
        // except those with `NoWireframe`. Meshes with `Wireframe` will always have a wireframe,
        // regardless of the global configuration.
        global: true,
        // Controls the default color of all wireframes. Used as the default color for global wireframes.
        // Can be changed per mesh using the `WireframeColor` component.
        default_color: ColorPalette::LightBase.color(),
    });
    app.init_resource::<GizmoConfigStore>();
}
