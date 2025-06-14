use buoy_sim::prelude::*;
use buoy_aero::prelude::*;

fn main() {
    let mut app = App::new();
    
    app.add_plugins(DefaultPlugins)
        .add_plugins(BuoySimPlugin)
        .add_plugins(AeroPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (update_position, update_aerodynamics));

    app.run();
}

fn setup(mut commands: Commands) {
    // Create a complex aerodynamic body
    commands.spawn((
        BuoyantBody::new(2.0),
        AeroBody::new(
            AeroProfile::Airfoil {
                chord_length: 1.0,
                thickness: 0.12,
                camber: 0.02,
            },
            SurfaceProperties {
                roughness: 0.0001,
                porosity: 0.0,
            },
        ),
        Transform::from_xyz(0.0, 10.0, 0.0),
        GlobalTransform::default(),
        Velocity::default(),
    ));
}

fn update_position(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Velocity, &BuoyantBody, &AeroBody)>,
) {
    for (mut transform, mut velocity, body, aero) in query.iter_mut() {
        // Update velocity based on forces
        velocity.linear += (body.buoyancy_force + aero.total_force) * time.delta_seconds();
        
        // Update position based on velocity
        transform.translation += velocity.linear * time.delta_seconds();
    }
}

fn update_aerodynamics(
    time: Res<Time>,
    mut query: Query<(&mut AeroBody, &Transform, &Velocity)>,
) {
    for (mut aero, transform, velocity) in query.iter_mut() {
        // Update aerodynamic forces based on current state
        aero.update_forces(velocity.linear, transform.rotation);
    }
} 