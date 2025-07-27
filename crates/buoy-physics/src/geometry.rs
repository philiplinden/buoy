#![allow(dead_code)]

use crate::constants::PI;
use avian3d::{
    parry::shape::{ShapeType, SharedShape},
    prelude::*,
};
use bevy::math::{Quat, Vec2, Vec3};

pub fn sphere_volume(radius: f32) -> f32 {
    (4.0 / 3.0) * PI * f32::powf(radius, 3.0)
}

pub fn sphere_radius_from_volume(volume: f32) -> f32 {
    f32::powf(volume * 3.0 / (4.0 * PI), 1.0 / 3.0)
}

pub fn shell_volume(internal_radius: f32, thickness: f32) -> f32 {
    let external_radius = internal_radius + thickness;
    let internal_volume = sphere_volume(internal_radius);
    let external_volume = sphere_volume(external_radius);
    external_volume - internal_volume
}

pub fn sphere_surface_area(radius: f32) -> f32 {
    4.0 * PI * f32::powf(radius, 2.0)
}

pub fn projected_area_of_sphere(radius: f32, _direction: Vec3) -> f32 {
    PI * radius * radius // πr²
}

pub fn projected_area_of_cuboid(half_size: Vec3, direction: Vec3) -> f32 {
    let a = half_size.x;
    let b = half_size.y;
    let c = half_size.z;

    let nx = direction.x.abs();
    let ny = direction.y.abs();
    let nz = direction.z.abs();

    2.0 * (a * b * nz + b * c * nx + a * c * ny)
}

pub fn projected_area_of_cylinder(radius: f32, height: f32, direction: Vec3) -> f32 {
    let nx = direction.x.abs();
    let ny = direction.y.abs();
    let nz = direction.z.abs();

    // Cylindrical surface contribution: 2rh|nz|
    let cylindrical_area = 2.0 * radius * height * nz;

    // End caps contribution: πr²(|nx| + |ny|)
    let end_caps_area = PI * radius * radius * (nx + ny);

    cylindrical_area + end_caps_area
}

pub fn projected_area_of_convex_hull(points: &[Vec3], direction: Vec3) -> f32 {
    // Project vertices onto plane perpendicular to direction
    let projected_points = project_vertices_onto_plane(points, direction);

    // Calculate area of the projected polygon
    polygon_area(&projected_points)
}

/// Generate the 8 corners of a cuboid from its half-size
fn generate_cuboid_corners(half_size: Vec3) -> Vec<Vec3> {
    let x = half_size.x;
    let y = half_size.y;
    let z = half_size.z;

    vec![
        Vec3::new(-x, -y, -z), // 0: bottom-back-left
        Vec3::new(x, -y, -z),  // 1: bottom-back-right
        Vec3::new(x, y, -z),   // 2: top-back-right
        Vec3::new(-x, y, -z),  // 3: top-back-left
        Vec3::new(-x, -y, z),  // 4: bottom-front-left
        Vec3::new(x, -y, z),   // 5: bottom-front-right
        Vec3::new(x, y, z),    // 6: top-front-right
        Vec3::new(-x, y, z),   // 7: top-front-left
    ]
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

/// Projected area for capsule shape
pub fn projected_area_of_capsule(radius: f32, half_height: f32, direction: Vec3) -> f32 {
    let nx = direction.x.abs();
    let ny = direction.y.abs();
    let nz = direction.z.abs();

    // Cylindrical part contribution: 2rh|nz|
    let cylindrical_area = 2.0 * radius * half_height * nz;

    // Hemispherical caps contribution: πr²(|nx| + |ny|)
    let caps_area = PI * radius * radius * (nx + ny);

    cylindrical_area + caps_area
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

pub fn project_vertices_onto_plane(points: &[Vec3], normal: Vec3) -> Vec<Vec2> {
    let normalized_normal = normal.normalize();

    points
        .iter()
        .map(|&p| {
            // Project point onto plane perpendicular to normal
            let projected = p - p.dot(normalized_normal) * normalized_normal;

            // Create orthonormal basis for the plane
            let up = if normalized_normal.dot(Vec3::Y).abs() < 0.9 {
                Vec3::Y
            } else {
                Vec3::X
            };
            let right = normalized_normal.cross(up).normalize();
            let plane_up = normalized_normal.cross(right).normalize();

            // Convert to 2D coordinates in the plane
            Vec2::new(projected.dot(right), projected.dot(plane_up))
        })
        .collect()
}

pub fn polygon_area(points: &[Vec2]) -> f32 {
    let n = points.len();
    let mut area = 0.0;
    for i in 0..n {
        let p1 = points[i];
        let p2 = points[(i + 1) % n];
        area += p1.x * p2.y - p2.x * p1.y;
    }
    0.5 * area.abs()
}
// Volume calculation functions for different shapes

pub fn cuboid_volume(half_size: Vec3) -> f32 {
    8.0 * half_size.x * half_size.y * half_size.z
}

pub fn cylinder_volume(radius: f32, height: f32) -> f32 {
    PI * radius * radius * height
}

pub fn capsule_volume(radius: f32, half_height: f32) -> f32 {
    let cylinder_vol = cylinder_volume(radius, 2.0 * half_height);
    let sphere_vol = sphere_volume(radius);
    cylinder_vol + sphere_vol
}

/// Calculate the volume of a convex polyhedron using the divergence theorem
/// V = (1/3) * sum(face_area * face_normal ⋅ face_centroid)
pub fn convex_polyhedron_volume(points: &[Vec3], faces: &[Vec<usize>]) -> f32 {
    let mut volume = 0.0;

    for face_indices in faces {
        if face_indices.len() < 3 {
            continue;
        }

        // Calculate face centroid
        let mut centroid = Vec3::ZERO;
        for &idx in face_indices {
            centroid += points[idx];
        }
        centroid /= face_indices.len() as f32;

        // Calculate face normal using first three vertices
        let v0 = points[face_indices[0]];
        let v1 = points[face_indices[1]];
        let v2 = points[face_indices[2]];
        let normal = (v1 - v0).cross(v2 - v0).normalize();

        // Calculate face area
        let mut face_area = 0.0;
        for i in 0..face_indices.len() {
            let p1 = points[face_indices[i]];
            let p2 = points[face_indices[(i + 1) % face_indices.len()]];
            face_area += p1.cross(p2).length();
        }
        face_area *= 0.5;

        // Add contribution to volume
        volume += face_area * normal.dot(centroid);
    }

    volume / 3.0
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
