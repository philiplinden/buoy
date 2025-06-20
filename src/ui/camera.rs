use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(PostStartup, setup_camera);
}

fn setup_camera(
    mut commands: Commands,
) {
    let object_pos = Vec3::new(0.0, 10.0, 20.0);
    commands.spawn((Camera3d::default(), Transform::from_translation(object_pos).looking_at(Vec3::ZERO, Vec3::Y)));
}
