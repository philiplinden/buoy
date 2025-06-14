use proptest::prelude::*;
use buoy_core::forces::{drag, buoyancy};
use buoy_core::ideal_gas::IdealGas;
use bevy::prelude::*;
use uom::si::f32::*;

proptest! {
    #[test]
    fn drag_force_always_opposes_velocity(
        velocity in prop::collection::vec(-100.0f32..100.0, 3),
        density in 0.0f32..2.0,
        area in 0.0f32..100.0,
        drag_coef in 0.0f32..2.0,
    ) {
        let velocity = Vec3::from_slice(&velocity);
        let force = drag(velocity, density, area, drag_coef);
        
        // Force should always oppose velocity
        assert!(force.dot(velocity) <= 0.0);
        
        // Force magnitude should be proportional to velocity magnitude
        let force_mag = force.length();
        let vel_mag = velocity.length();
        if vel_mag > 0.001 {
            assert!(force_mag > 0.0);
        }
    }

    #[test]
    fn buoyancy_force_always_upward(
        volume in 0.0f32..1000.0,
        density in 0.0f32..2.0,
    ) {
        let force = buoyancy(volume, density);
        assert!(force.y > 0.0); // Always upward
        assert!(force.x == 0.0 && force.z == 0.0); // Only vertical
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;
    use buoy_core::forces::scale_gravity;
    use buoy_core::constants::EARTH_RADIUS_M;

    #[test]
    fn test_gravity_scaling() {
        // At sea level, gravity should be close to standard gravity
        let sea_level_scale = scale_gravity(0.0);
        assert!((sea_level_scale - 1.0).abs() < 0.01);

        // At high altitude, gravity should be reduced
        let high_altitude_scale = scale_gravity(10000.0);
        assert!(high_altitude_scale < sea_level_scale);

        // At Earth's radius, gravity should be about 1/4 of surface gravity
        let radius_scale = scale_gravity(EARTH_RADIUS_M.get::<uom::si::length::meter>());
        assert!((radius_scale - 0.25).abs() < 0.01);
    }

    #[test]
    fn test_drag_force_properties() {
        let velocity = Vec3::new(1.0, 0.0, 0.0);
        let force = drag(velocity, 1.225, 1.0, 0.47);
        
        // Force should oppose motion
        assert!(force.x < 0.0);
        assert!(force.y == 0.0);
        assert!(force.z == 0.0);
        
        // Force magnitude should be proportional to velocity squared
        let force_mag = force.length();
        assert!((force_mag - 0.5 * 1.225 * 1.0 * 0.47).abs() < 0.001);
    }
} 