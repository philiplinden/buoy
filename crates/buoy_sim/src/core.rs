use std::fmt::Display;

use super::*;
use avian3d::prelude::*;
use bevy::{
    app::{PluginGroup, PluginGroupBuilder},
    prelude::*,
};
use uom::si::{Quantity, acceleration::meter_per_second_squared, f32::*};

pub struct BuoyPhysicsPlugin;

impl Plugin for BuoyPhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            PhysicsPlugins::default().set(PhysicsInterpolationPlugin::interpolate_all()),
            ideal_gas::plugin,
        ));
        app.insert_resource(Gravity(
            Vec3::NEG_Y * super::constants::STANDARD_GRAVITY.get::<meter_per_second_squared>(),
        ));
        app.add_systems(
            FixedUpdate,
            (update_gravity)
                .chain()
                .in_set(PhysicsStepSet::First)
                .in_set(PausableSystems),
        );
    }
}

fn update_gravity(mut query: Query<(&mut GravityScale, &Position)>) {
    for (mut gravity_scale, position) in query.iter_mut() {
        gravity_scale.0 = super::forces::scale_gravity(position.y);
    }
}

/// The plugin that handles the overall simulation state.
pub struct BuoySystemsPlugin;

impl Plugin for BuoySystemsPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<SimState>();
        app.add_plugins((format::plugin,));

        app.configure_sets(Update, PausableSystems.run_if(in_state(SimState::Running)));
    }
}

#[derive(States, Debug, Default, Clone, Copy, Hash, PartialEq, Eq)]
pub enum SimState {
    #[default]
    Initializing,
    Running,
    Stopped,
    Paused,
    HardwareWaiting,
    Faulted,
}

/// A system set for systems that shouldn't run while the game is paused.
#[derive(SystemSet, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct PausableSystems;
