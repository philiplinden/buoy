use avian3d::debug_render::PhysicsDebugPlugin;
use bevy::{
    dev_tools::fps_overlay::FpsOverlayPlugin,
    pbr::wireframe::{WireframeConfig, WireframePlugin},
    prelude::*,
};

#[cfg(feature = "grid_space")]
use {
    big_space::{camera::CameraController, prelude::*},
    buoy_physics::grid::Precision,
};

use buoy_common::ColorPalette;


pub(crate) fn plugin(app: &mut App) {
    app.add_plugins((
        PhysicsDebugPlugin::default(),               // Draws colliders
        WireframePlugin::default(),
        FpsOverlayPlugin::default(),
        TransformPathPlugin,
    ));
    #[cfg(feature = "grid_space")]
    app.add_plugins((
        FloatingOriginDebugPlugin::<Precision>::default(), // Draws cell AABBs and grids
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
pub struct TransformPathPlugin;

impl Plugin for TransformPathPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            add_transform_path,
            update_transform_paths.before(draw_transform_paths),
        ));
    }
}

#[derive(Component, Reflect)]
pub struct TransformPath {
    pub points: Vec<Vec3>,
    pub max_length: usize,
}

impl Default for TransformPath {
    fn default() -> Self {
        Self {
            points: Vec::new(),
            max_length: 1000,
        }
    }
}

fn add_transform_path(
    mut commands: Commands,
    query: Query<Entity, (With<GlobalTransform>, Without<TransformPath>)>,
) {
    for entity in &query {
        commands.entity(entity).insert(TransformPath::default());
    }
}

fn update_transform_paths(
    mut query: Query<(&GlobalTransform, &mut TransformPath)>,
) {
    for (transform, mut path) in &mut query {
        let current_position = transform.translation();

        if path.points.last().map_or(true, |last| last.distance(current_position) > 0.01) {
            path.points.push(current_position);

            if path.points.len() > path.max_length {
                path.points.remove(0);
            }
        }
    }
}

fn draw_transform_paths(
    mut gizmos: Gizmos,
    query: Query<&TransformPath>,
    mut config_store: ResMut<GizmoConfigStore>,
) {
    let (config, _) = config_store.config_mut::<DefaultGizmoConfigGroup>();
    config.depth_bias = -1.0;

    for path in &query {
        if path.points.len() < 2 {
            continue;
        }

        for i in 1..path.points.len() {
            let t = i as f32 / path.points.len() as f32;
            let mut color = ColorPalette::BoldPurple.color();
            color.set_alpha(1.0 - t);
            gizmos.line(
                path.points[i - 1],
                path.points[i],
                color,
            );
        }
    }
}
