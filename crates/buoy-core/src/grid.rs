//! Grid system for managing spatial organization and fluid volumes in the simulation.
//!
//! This module implements a hierarchical grid system using BigSpace for large-scale
//! simulations. The root grid contains the entire world, while child grids can be
//! used for specific volumes like atmospheres or oceans.
//!
//! Use child grids to specify large environment volumes, like atmosphere or ocean.
//! New spatial entities should be spawned with their own local grids.
//! Objects can move across environment boundaries so long as they are related
//! along the same branch of the hierarchy.
//!
//! The switching threshold (set to 0.0 here) determines when grid cells switch.
//! A non-zero threshold creates a "buffer zone" around cell boundaries to prevent
//! rapid switching when objects oscillate near the edge. For example:
//!
//! ```text
//! Cell 1          |          Cell 2
//!                 |
//!         <--🚀-->|   (threshold = 0.0)
//!                 |
//!     [===buffer zone===]   (threshold > 0.0)
//! ```
//!
//! With threshold = 0.0, the object triggers an immediate cell switch when crossing
//! the boundary. A positive threshold allows some movement past the boundary before
//! switching, preventing jitter for objects that frequently cross cell edges.

use bevy::prelude::*;
use big_space::prelude::*;

/// The size of grid cells in meters. This determines the spatial resolution of the grid.
/// Larger values improve performance but reduce precision.
pub const GRID_CELL_EDGE_LENGTH_METERS: f32 = 10.0;

/// The buffer zone around grid cell boundaries in meters. This prevents rapid switching
/// when objects oscillate near cell edges. A value of 0.5 means objects can move 0.5m
/// past the boundary before switching cells.
pub const GRID_SWITCHING_THRESHOLD_METERS: f32 = 0.5;

/// The precision type for grid coordinates. This determines the maximum size of the
/// simulation space. Higher precision types allow larger worlds but use more memory.
#[cfg(all(feature = "i32", not(any(feature = "i64", feature = "i128"))))]
pub type Precision = i32;
#[cfg(all(feature = "i64", not(any(feature = "i32", feature = "i128"))))]
pub type Precision = i64;
#[cfg(all(feature = "i128", not(any(feature = "i32", feature = "i64"))))]
pub type Precision = i128;

// Plugin setup
pub(crate) fn plugin(app: &mut App) {
    // Add BigSpace plugin with selected precision
    app.add_plugins(BigSpacePlugin::<Precision>::default());

    // Add startup systems - order matters!
    // 1. setup_worldspace creates the root grid
    // 2. spawn_fluid_volume creates fluid volumes as children of the root grid
    app.add_systems(Startup, setup_worldspace);
}

/// Creates the root grid that contains the entire simulation world.
/// This is the top-level grid in the hierarchy. All other grids and entities
/// should be children of this grid to maintain proper spatial relationships.
fn setup_worldspace(mut commands: Commands) {
    let world_grid = Grid::<Precision>::new(
        GRID_CELL_EDGE_LENGTH_METERS,
        GRID_SWITCHING_THRESHOLD_METERS,
    );

    // Spawn the root grid with necessary components
    commands.spawn_big_space(world_grid, |root_grid| {
        // Mark this as the root grid
        root_grid.insert(RootGrid);

        // Create a starting spot entity
        root_grid.spawn_spatial((
            Name::new("Starting Spot"),
            StartingSpot,
            FloatingOrigin,
            Transform::default(),
        ));
    });
}

/// Marks the starting location of the simulation world
#[derive(Component)]
pub struct StartingSpot;

/// Marks the root grid entity
#[derive(Component)]
pub struct RootGrid;
