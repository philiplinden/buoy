#![allow(dead_code)]

use crate::constants::PI;
use avian3d::{
    parry::shape::{ShapeType, SharedShape},
    prelude::*,
};
use bevy::math::{Quat, Vec2, Vec3};

pub fn projected_area_of_convex_hull(points: &[Vec3], direction: Vec3) -> f32 {
    // Project vertices onto plane perpendicular to direction
    let projected_points = project_vertices_onto_plane(points, direction);

    // Calculate area of the projected polygon
    polygon_area(&projected_points)
}

/// Feature-based projected area calculation using Parry-style silhouette method
/// For each face: projected_area += face_area × |face_normal ⋅ dir|
fn feature_based_projected_area(_shape: &SharedShape, _direction: Vec3) -> Option<f32> {
    // TODO: Implement feature-based calculation using shape topology
    // This would use the shape's faces, edges, and vertices directly
    // For now, return None to fall back to AABB projection
    None
}

/// AABB projected area fallback
/// Use the projected area of the collider's AABB in the direction
fn aabb_projected_area(collider: &Collider, direction: Vec3) -> f32 {
    // Get the AABB of the collider
    let aabb = collider.aabb(Vec3::ZERO, Quat::from_rotation_arc(Vec3::Z, direction));
    let half_size = aabb_half_extents(&aabb);
    let corners = generate_cuboid_corners(half_size);
    projected_area_of_convex_hull(&corners, direction)
}

/// Calculate the projected area of a collider in the direction of motion.
/// Used for aerodynamic drag force calculations: F_drag = 0.5 * rho * v^2 * C_d * A_projected
pub fn projected_area(collider: &Collider, direction: Vec3) -> f32 {
    let normalized_direction = direction.normalize();
    let shape = collider.shape();
    let shape_type = shape.shape_type();

    match shape_type {
        // 1. Sphere – Use analytical projection: π * r²
        ShapeType::Ball => {
            if let Some(radius) = shape.as_ball() {
                projected_area_of_sphere(radius.radius, normalized_direction)
            } else {
                aabb_projected_area(collider, normalized_direction)
            }
        }

        // 2. Cuboid – Project all 8 corners, form 2D convex hull, return area
        ShapeType::Cuboid => {
            if let Some(cuboid) = shape.as_cuboid() {
                let corners = generate_cuboid_corners(cuboid.half_extents.into());
                projected_area_of_convex_hull(&corners, normalized_direction)
            } else {
                aabb_projected_area(collider, normalized_direction)
            }
        }

        // 3. Cylinder – Use analytical formula
        ShapeType::Cylinder => {
            if let Some(cylinder) = shape.as_cylinder() {
                projected_area_of_cylinder(
                    cylinder.radius,
                    cylinder.half_height * 2.0,
                    normalized_direction,
                )
            } else {
                aabb_projected_area(collider, normalized_direction)
            }
        }

        // 4. Capsule – Use analytical formula
        ShapeType::Capsule => {
            if let Some(capsule) = shape.as_capsule() {
                projected_area_of_capsule(capsule.radius, capsule.half_height(), normalized_direction)
            } else {
                aabb_projected_area(collider, normalized_direction)
            }
        }

        // 5. ConvexHull – Use Parry-style silhouette method
        ShapeType::ConvexPolyhedron => {
            if let Some(area) = feature_based_projected_area(shape, normalized_direction) {
                area
            } else {
                // Fall back to AABB projection
                aabb_projected_area(collider, normalized_direction)
            }
        }

        // 6. Other convex shapes – Try feature-based first
        _ if shape.is_convex() => {
            if let Some(area) = feature_based_projected_area(shape, normalized_direction) {
                area
            } else {
                // Fall back to AABB projection
                aabb_projected_area(collider, normalized_direction)
            }
        }

        // 7. Non-convex shapes – Use AABB fallback
        _ => aabb_projected_area(collider, normalized_direction),
    }
}

/// Calculate the volume of a collider
pub fn collider_volume(collider: &Collider) -> f32 {
    let shape = collider.shape();
    let shape_type = shape.shape_type();

    match shape_type {
        ShapeType::Ball => {
            if let Some(ball) = shape.as_ball() {
                sphere_volume(ball.radius)
            } else {
                0.0
            }
        }

        ShapeType::Cuboid => {
            if let Some(cuboid) = shape.as_cuboid() {
                cuboid_volume(cuboid.half_extents.into())
            } else {
                0.0
            }
        }

        ShapeType::Cylinder => {
            if let Some(cylinder) = shape.as_cylinder() {
                cylinder_volume(cylinder.radius, cylinder.half_height * 2.0)
            } else {
                0.0
            }
        }

        ShapeType::Capsule => {
            if let Some(capsule) = shape.as_capsule() {
                capsule_volume(capsule.radius, capsule.half_height())
            } else {
                0.0
            }
        }

        _ => {
            // For other shapes, use AABB volume as fallback
            let aabb = collider.aabb(Vec3::ZERO, Quat::IDENTITY);
            aabb_volume(&aabb) * 0.5 // Conservative estimate
        }
    }
}

/// Get the half-extents of an AABB (half of the size)
pub fn aabb_half_extents(aabb: &ColliderAabb) -> Vec3 {
    aabb.size() * 0.5
}

/// Calculate the volume of an AABB
pub fn aabb_volume(aabb: &ColliderAabb) -> f32 {
    let size = aabb.size();
    size.x * size.y * size.z
}
