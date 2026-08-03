use bevy::prelude::*;

const CAMERA_TARGET: Vec3 = Vec3::ZERO;

pub fn plugin(app: &mut App) {
    {
        app.init_resource::<OriginalCameraTransform>()
            .add_systems(Startup, setup_normal_camera);
    }
}

#[derive(Resource, Deref, DerefMut, Default)]
pub struct OriginalCameraTransform(Transform);

fn setup_normal_camera(mut commands: Commands) {
    let camera_pos = Vec3::new(-2.0, 2.5, 5.0);
    let camera_transform =
        Transform::from_translation(camera_pos).looking_at(CAMERA_TARGET, Vec3::Y);
    commands.insert_resource(OriginalCameraTransform(camera_transform));

    commands.spawn((Camera3d::default(), camera_transform));
}
