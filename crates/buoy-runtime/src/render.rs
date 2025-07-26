use bevy::prelude::*;
use crate::objects::GroundPlane;

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (setup_scene_visuals, add_mesh_to_ground_plane));
    }
}

fn setup_scene_visuals(
    mut commands: Commands,
) {
    // Spawn a light so we can see
    commands.spawn((
        PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..Default::default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
}

/// Spawns a mesh for each ground plane in the scene so we can see it.
fn add_mesh_to_ground_plane(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<(Entity, &GroundPlane), Without<Mesh3d>>,
) {
    for (entity, ground_plane) in query.iter_mut() {
        commands.entity(entity).with_children(|children| {
            children.spawn((
                Mesh3d(meshes.add(Plane3d::default().mesh().size(ground_plane.extents.0, ground_plane.extents.1))),
                MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
            ));
        });
    }
}
