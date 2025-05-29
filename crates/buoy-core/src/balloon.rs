use bevy::prelude::*;
use avian3d::prelude::{Position, Gravity};
use uom::si::{f32::*, force::newton, length::meter, mass::kilogram, volume::cubic_meter, acceleration::meter_per_second_squared};

use crate::ideal_gas::GasSpecies;
use crate::material_properties::Skin;

/// Physical properties of a balloon envelope
#[derive(Component, Debug, Clone)]
pub struct Balloon {
    // Maximum volume (m³) before the balloon bursts
    pub envelope_volume: Volume,
    // Mass (kg) of the envelope material
    pub envelope_mass: Mass,
    // Mass (kg) of the payload
    pub payload_mass: Mass,
    // Gas filling the balloon
    pub lift_gas: GasSpecies,
    // Altitude (m) at which the balloon is expected to burst
    pub burst_altitude: Length,
    // Path to the balloon 3D mesh file
    pub mesh_path: String,
    // Material properties of the balloon skin
    pub skin: Skin,
}

impl Default for Balloon {
    fn default() -> Self {
        Self {
            envelope_volume: Volume::new::<cubic_meter>(1000.0),
            envelope_mass: Mass::new::<kilogram>(2.0),
            payload_mass: Mass::new::<kilogram>(1.5),
            lift_gas: GasSpecies::helium(),
            burst_altitude: Length::new::<meter>(30000.0),
            mesh_path: "assets/models/balloon.glb".to_string(),
            skin: Skin::default(),
        }
    }
}

use crate::{
    atmosphere::Atmosphere,
    forces::buoyancy,
    geometry::sphere_radius_from_volume,
};

pub(crate) fn plugin(app: &mut App) {
    // app.register_type::<Balloon>()
    app.add_systems(FixedUpdate, update_balloon_physics);
}

/// Real-time physics calculations for balloons
#[derive(Component, Debug, Clone)]
pub struct BalloonPhysics {
    // Current volume (m³) of the gas
    pub current_volume: Volume,
    // Current buoyant force (N)
    pub buoyant_force: Force,
    // Current ambient atmospheric conditions
    pub ambient_temperature: ThermodynamicTemperature,
    pub ambient_pressure: Pressure,
    pub ambient_density: MassDensity,
    // Current gas conditions
    pub gas_temperature: ThermodynamicTemperature,
    pub gas_pressure: Pressure,
    pub gas_density: MassDensity,
    // Current balloon radius (m)
    pub radius: f32,
    // Has the balloon burst?
    pub burst: bool,
}

impl Default for BalloonPhysics {
    fn default() -> Self {
        Self {
            current_volume: Volume::new::<cubic_meter>(0.0),
            buoyant_force: Force::new::<newton>(0.0),
            ambient_temperature: Atmosphere::standard_temperature(),
            ambient_pressure: Atmosphere::standard_pressure(),
            ambient_density: Atmosphere::standard_density(),
            gas_temperature: Atmosphere::standard_temperature(),
            gas_pressure: Atmosphere::standard_pressure(),
            gas_density: MassDensity::default(),
            radius: 0.0,
            burst: false,
        }
    }
}

fn update_balloon_physics(
    mut query: Query<(&mut BalloonPhysics, &Balloon, &Position)>,
    atmosphere: Res<Atmosphere>,
    gravity: Res<Gravity>,
) {
    for (mut physics, balloon, position) in query.iter_mut() {
        // Update ambient conditions based on position
        let position_vec = Vec3::new(position.x, position.y, position.z);
        physics.ambient_temperature = atmosphere.temperature(position_vec);
        physics.ambient_pressure = atmosphere.pressure(position_vec);
        physics.ambient_density = atmosphere.density(position_vec);

        // TODO: Implement gas expansion model based on altitude
        // For now, use a simple model where volume increases with altitude

        // Calculate buoyant force
        physics.buoyant_force = Force::new::<newton>(
            buoyancy(
                Acceleration::new::<meter_per_second_squared>(0.0),
                physics.current_volume,
                physics.ambient_density
            ).y
        );

        // Update balloon radius
        physics.radius = sphere_radius_from_volume(physics.current_volume.get::<cubic_meter>());

        // Check for burst conditions
        if position.y > balloon.burst_altitude.get::<meter>()
            || physics.current_volume > balloon.envelope_volume {
            physics.burst = true;
        }
    }
}
