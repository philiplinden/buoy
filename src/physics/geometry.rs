#![allow(dead_code)]

use super::constants::PI;

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
