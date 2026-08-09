use avian3d::prelude::Collider;
use bevy::{
    math::Isometry3d,
    pbr::wireframe::{WireframeConfig, WireframePlugin},
    prelude::*,
};

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

/// Draws a wireframe outline of each entity's collider shape. Replaces
/// Avian's `PhysicsDebugPlugin`, which we don't otherwise depend on.
fn draw_colliders(mut gizmos: Gizmos, colliders: Query<(&GlobalTransform, &Collider)>) {
    let color = ColorPalette::LightBase.color();
    for (transform, collider) in &colliders {
        let isometry = Isometry3d::new(transform.translation(), transform.rotation());
        let shape = collider.shape();
        if let Some(ball) = shape.as_ball() {
            gizmos.sphere(isometry, ball.radius, color);
        } else if let Some(cuboid) = shape.as_cuboid() {
            let half_extents: Vec3 = cuboid.half_extents.into();
            gizmos.cuboid(
                Transform::from_translation(transform.translation())
                    .with_rotation(transform.rotation())
                    .with_scale(half_extents * 2.0),
                color,
            );
        }
    }
}
