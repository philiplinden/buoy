use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, (spawn_camera, spawn_map));
}

fn spawn_camera(
    mut commands: Commands,
) {
    commands.spawn(Camera3d::default());
}

fn spawn_map(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn a directional light so we can see the map
    commands.spawn(DirectionalLight::default());

    // Spawn a ball to represent the balloon
    let ball_mesh = meshes.add(Sphere::new(1.0));
    let color = Color::srgb(0.8, 0.7, 0.6);
    let ball_material = materials.add(StandardMaterial {
        base_color: color,
        ..default()
    });
    // Spawn the ball. If we wanted to spawn more than one, we could use a loop
    // and have them all share the mesh as long as we .clone() it. Since we only
    // have one ball, we can just use it directly. This consumes the handle.
    commands.spawn((
        Transform::from_translation(Vec3::new(0.0, 0.0, -50.0)),
        Mesh3d(ball_mesh),
        MeshMaterial3d(ball_material),
    ));
}
