use bevy::prelude::*;
use uom::si::{f32::*, mass::kilogram, volume::cubic_meter};

use crate::{
    atmosphere::Atmosphere,
    balloon::Balloon,
    balloon_physics::BalloonPhysics,
    ideal_gas::GasSpecies,
    mesh_drag::MeshDragCalculator,
};

/// Spawn a standard test balloon with default configuration
pub fn spawn_test_balloon(commands: &mut Commands) -> Entity {
    let balloon = commands.spawn((
        Balloon {
            envelope_volume: Volume::new::<cubic_meter>(1000.0),
            envelope_mass: Mass::new::<kilogram>(2.0),
            payload_mass: Mass::new::<kilogram>(1.5),
            lift_gas: GasSpecies::helium(),
            burst_altitude: Length::new::<meter>(30000.0),
            mesh_path: "assets/models/balloon.glb".to_string(),
            skin: Skin::default(),
        },
        BalloonPhysics::default(),
        MeshDragCalculator::default(),
        // Avian3D components
        RigidBody::Dynamic,
        Position::default(),
        Rotation::default(),
        LinearVelocity::default(),
        AngularVelocity::default(),
        Mass::new::<kilogram>(3.5), // Envelope + payload
        ExternalForce::default(),
        ExternalTorque::default(),
        GravityScale(1.0),
    )).id();
    
    balloon
}