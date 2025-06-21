use bevy::prelude::*;

#[cfg(feature = "grid_space")]
mod grid_space {
    use bevy::math::DVec3;
    use big_space::prelude::*;
    use buoy_physics::grid::Precision;
}

const CAMERA_TARGET: Vec3 = Vec3::ZERO;

pub fn plugin(app: &mut App) {
    #[cfg(feature = "grid_space")]
    {
        app.add_plugins((big_space::camera::CameraControllerPlugin::<Precision>::default(),));
        app.add_systems(PostStartup, setup_floating_camera);
        app.add_systems(PostUpdate, big_space::camera::default_camera_inputs);
    }
    #[cfg(not(feature = "grid_space"))]
    {
        app.init_resource::<OriginalCameraTransform>()
            .add_systems(Startup, setup_normal_camera);
    }
}

#[derive(Resource, Deref, DerefMut, Default)]
pub struct OriginalCameraTransform(Transform);

/// Spawns the camera in the root grid and attaches the FloatingOrigin to it.
///
/// This system removes the previous FloatingOrigin and spawns a new one with
/// the camera. There should only be one FloatingOrigin. Any spatial entity can
/// be the floating origin. Attaching it to the camera ensures the camera will
/// never see floating point precision rendering artifacts.
#[cfg(feature = "grid_space")]
fn setup_floating_camera(
    mut commands: Commands,
    mut previous_origin: Query<Entity, With<FloatingOrigin>>,
    // HACK: This is a hack to access the root grid and add to it. It is not
    // ideal nor recommended. https://github.com/aevyrie/big_space/issues/36
    root_grid: Query<(Entity, &Grid<Precision>), With<RootGrid>>,
) {
    // Remove the FloatingOrigin component from the previous origin.
    let origin = previous_origin.single_mut();
    commands.entity(origin).remove::<FloatingOrigin>();

    let (root_grid_id, root_grid) = root_grid.single();

    // Spawn the camera and attach the FloatingOrigin to it.
    let object_pos = DVec3::new(0.0, 10.0, 20.0);
    let (object_cell, object_pos) = root_grid.translation_to_grid(object_pos);
    commands
        .spawn((
            Camera3d::default(),
            FloatingOrigin,
            object_cell,
            Transform::from_translation(object_pos).looking_at(Vec3::ZERO, Vec3::Y),
            big_space::camera::CameraController::default() // Built-in camera controller
                .with_speed_bounds([0.1, 10.0])
                .with_smoothness(0.98, 0.98)
                .with_speed(1.0),
        ))
        .set_parent(root_grid_id);
}

#[cfg(not(feature = "grid_space"))]
fn setup_normal_camera(mut commands: Commands) {
    let camera_pos = Vec3::new(-2.0, 2.5, 5.0);
    let camera_transform =
        Transform::from_translation(camera_pos).looking_at(CAMERA_TARGET, Vec3::Y);
    commands.insert_resource(OriginalCameraTransform(camera_transform));

    commands.spawn((Camera3d::default(), camera_transform));
}
