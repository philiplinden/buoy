use buoy_sim::prelude::*;

fn main() {
    // Initialize the simulation
    let mut app = App::new();
    
    // Add basic systems
    app.add_plugins(DefaultPlugins)
        .add_plugins(BuoySimPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, update);

    // Run the simulation
    app.run();
}

fn setup(mut commands: Commands) {
    // Create a basic balloon
    commands.spawn((
        BuoyantBody::new(1.0), // 1.0 kg mass
        Transform::from_xyz(0.0, 0.0, 0.0),
        GlobalTransform::default(),
    ));
}

fn update(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &BuoyantBody)>,
) {
    for (mut transform, body) in query.iter_mut() {
        // Update position based on buoyancy
        transform.translation.y += body.buoyancy_force * time.delta_seconds();
    }
} 