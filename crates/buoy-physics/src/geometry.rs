use glam::{Vec2, Vec3};
use crate::constants::PI;

/// Generate the 8 corners of a cuboid from its half-size
#[allow(dead_code)]
pub fn generate_cuboid_corners(half_size: Vec3) -> Vec<Vec3> {
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
