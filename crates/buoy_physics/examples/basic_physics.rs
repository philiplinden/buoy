//! Basic physics calculations example
//! 
//! This example demonstrates the core physics calculations available in the buoy_physics crate.

use buoy_physics::{forces, ideal_gas, material_properties, constants};
use uom::si::{
    f64::*,
    mass::kilogram,
    thermodynamic_temperature::kelvin,
    volume::cubic_meter,
    pressure::pascal,
    force::newton,
};

fn main() {
    // Calculate force using Newton's second law
    let mass = Mass::new::<kilogram>(1.0);
    let acceleration = Length::new::<meter>(9.81) / Time::new::<second>(1.0).powi(2);
    let force = forces::calculate_force(mass, acceleration);
    println!("Force: {} N", force.get::<newton>());

    // Use ideal gas law
    let volume = Volume::new::<cubic_meter>(1.0);
    let temperature = ThermodynamicTemperature::new::<kelvin>(293.15);
    let moles = 1.0;
    let pressure = ideal_gas::calculate_pressure(volume, temperature, moles);
    println!("Pressure: {} Pa", pressure.get::<pascal>());

    // Get material properties
    let material = material_properties::Material::Air;
    let density = material_properties::get_density(material);
    println!("Air density: {} kg/m³", density);

    // Use physical constants
    println!("Gravitational acceleration: {} m/s²", constants::GRAVITY);
    println!("Gas constant: {} J/(mol·K)", constants::GAS_CONSTANT);
} 