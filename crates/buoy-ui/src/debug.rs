use bevy::{
    math::Isometry3d,
    pbr::wireframe::{WireframeConfig, WireframePlugin},
    prelude::*,
};
use buoy_physics::geometry::Shape;

use crate::colors::ColorPalette;


pub(crate) fn plugin(app: &mut App) {
    app.add_plugins(WireframePlugin::default());
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
    app.add_systems(Update, draw_colliders);
}

/// Draws a wireframe outline of each entity's shape. Replaces Avian's
/// `PhysicsDebugPlugin`, which we don't otherwise depend on.
fn draw_colliders(mut gizmos: Gizmos, shapes: Query<(&GlobalTransform, &Shape)>) {
    let color = ColorPalette::LightBase.color();
    for (transform, shape) in &shapes {
        let isometry = Isometry3d::new(transform.translation(), transform.rotation());
        match *shape {
            Shape::Sphere { radius } => {
                gizmos.sphere(isometry, radius, color);
            }
            Shape::Cuboid { half_extents } => {
                gizmos.cuboid(
                    Transform::from_translation(transform.translation())
                        .with_rotation(transform.rotation())
                        .with_scale(half_extents * 2.0),
                    color,
                );
            }
        }
    }
}
