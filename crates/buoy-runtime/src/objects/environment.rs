use avian3d::prelude::*;
use bevy::prelude::*;

/// Component marking a ground plane in the simulation.
#[derive(Component, Debug, Clone, Copy)]
#[require(Transform, RigidBody, Collider)]
pub struct GroundPlane {
    pub extents: (f32, f32),
    pub thickness: f32,
}

impl Default for GroundPlane {
    fn default() -> Self {
        Self {
            extents: (1.0, 1.0),
            thickness: 0.01,
        }
    }
}

impl GroundPlane {
    pub fn generate_collider(&self) -> Collider {
        let half_extents = Vec3::new(self.extents.0 / 2.0, self.thickness / 2.0, self.extents.1 / 2.0);
        Collider::cuboid(half_extents.x, half_extents.y, half_extents.z)
    }
}

/// All the components needed to create a ground plane.
#[derive(Bundle)]
pub struct GroundPlaneBundle {
    name: Name,
    ground_plane: GroundPlane,
    transform: Transform,
    collider: Collider,
    rigid_body: RigidBody,
}

impl GroundPlane {
    pub fn new(extents: (f32, f32)) -> impl Bundle {
        let ground_plane = GroundPlane { extents, thickness: 0.01 };
        GroundPlaneBundle {
            name: Name::new("Ground Plane"),
            ground_plane,
            // make the top surface flush with the origin
            transform: Transform::from_xyz(0.0, -ground_plane.thickness / 2.0, 0.0),
            collider: ground_plane.generate_collider(),
            rigid_body: RigidBody::Static,
        }
    }
}

/// System to update the collider of a ground plane when its extents change.
pub fn update_ground_plane_collider(
    mut commands: Commands,
    mut query: Query<(Entity, &GroundPlane), Changed<GroundPlane>>, // This filter makes the system run only when GroundPlane changes
) {
    for (entity, ground_plane) in query.iter_mut() {
        if ground_plane.extents.0 != ground_plane.extents.0
            || ground_plane.extents.1 != ground_plane.extents.1
            || ground_plane.thickness != ground_plane.thickness
        {
            commands
                .entity(entity)
                .remove::<Collider>()
                .insert(ground_plane.generate_collider());
        }
    }
}
