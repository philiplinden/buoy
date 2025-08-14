use uom::si::{
    acceleration::meter_per_second_squared, area::square_meter, f32::*, length::meter,
    mass::kilogram, mass_density::kilogram_per_cubic_meter, ratio::ratio, volume::cubic_meter,
    velocity::meter_per_second,
};

use crate::atmosphere::Atmosphere;
use crate::constants::{EARTH_RADIUS_M, STANDARD_GRAVITY};
use crate::geometry::{projected_area, collider_volume};

fn net_force(
    mut query: Query<(
        &mut ExternalForce,
        &Position,
        &ComputedMass,
        &Collider,
        &LinearVelocity,
        &DragCoefficient,
    )>,
    atmosphere: Res<Atmosphere>,
) {
    for (mut external_force, position, computed_mass, collider, velocity, drag_coefficient) in
        query.iter_mut()
    {
        let mass = uom::si::f32::Mass::new::<kilogram>(computed_mass.value());
        let drag_area =
            uom::si::f32::Area::new::<square_meter>(projected_area(collider, velocity.0));
        let gravity = local_gravity(uom::si::f32::Length::new::<meter>(position.y));
        let weight = weight(mass, gravity);
        let buoyancy = buoyancy(gravity, uom::si::f32::Volume::new::<cubic_meter>(collider_volume(collider)), atmosphere.density(position.0));
        let drag = drag(
            velocity.0,
            atmosphere.density(position.0),
            drag_area,
            drag_coefficient.0,
        );
        let net_force = weight + buoyancy + drag;

        external_force.apply_force(net_force);
    }
}

#[derive(Component, Default)]
pub struct DragCoefficient(pub Scalar);
