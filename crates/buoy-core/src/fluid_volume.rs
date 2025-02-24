use bevy::prelude::*;
use big_space::prelude::*;
use uom::si::{
    f32::*, length::meter, mass_density::kilogram_per_cubic_meter, pressure::pascal,
    thermodynamic_temperature::kelvin,
};

use crate::atmosphere::Atmosphere;
use crate::constants::TRANSLATION_SCALE;
use crate::grid::{
    Precision, RootGrid, GRID_CELL_EDGE_LENGTH_METERS, GRID_SWITCHING_THRESHOLD_METERS,
};

pub(crate) fn plugin(app: &mut App) {
    app.init_resource::<DefaultFluidVolumeSettings>();
}

/// Marks a grid that contains a fluid volume.
///
/// FIXME: There must be a better way to do this.
#[derive(Component)]
pub struct FluidVolumeGrid;

/// Represents a volume of fluid with physical properties.
/// Each fluid volume is its own grid, allowing for independent spatial organization
/// and property management.
#[derive(Component)]
pub struct FluidVolume {
    /// Temperature in Kelvin (K)
    pub temperature: ThermodynamicTemperature,
    /// Pressure in Pascals (Pa)
    pub pressure: Pressure,
    /// Density in kg/m³
    pub density: MassDensity,
    /// Grid that defines the spatial structure of this volume
    pub grid: Grid<Precision>,
}

/// Builder for creating fluid volumes with custom properties
pub struct FluidVolumeBuilder {
    temperature: ThermodynamicTemperature,
    pressure: Pressure,
    density: MassDensity,
    size: IVec3, // Number of cells in each dimension
    position: Vec3,
}

impl FluidVolumeBuilder {
    /// Create a new builder with default atmospheric conditions
    pub fn new() -> Self {
        Self {
            temperature: ThermodynamicTemperature::new::<kelvin>(300.0),
            pressure: Pressure::new::<pascal>(101325.0),
            density: MassDensity::new::<kilogram_per_cubic_meter>(1.225),
            size: IVec3::new(10, 10, 10), // Default 10x10x10 grid
            position: Vec3::ZERO,
        }
    }

    /// Set the temperature of the fluid volume
    pub fn with_temperature(mut self, temperature: ThermodynamicTemperature) -> Self {
        self.temperature = temperature;
        self
    }

    /// Set the pressure of the fluid volume
    pub fn with_pressure(mut self, pressure: Pressure) -> Self {
        self.pressure = pressure;
        self
    }

    /// Set the density of the fluid volume
    pub fn with_density(mut self, density: MassDensity) -> Self {
        self.density = density;
        self
    }

    /// Set the size of the fluid volume in grid cells
    pub fn with_size(mut self, size: IVec3) -> Self {
        self.size = size;
        self
    }

    /// Set the position of the fluid volume in world space
    pub fn with_position(mut self, position: Vec3) -> Self {
        self.position = position;
        self
    }

    /// Build and spawn the fluid volume
    pub fn spawn(self, commands: &mut Commands, root_grid: Entity) {
        commands.entity(root_grid).with_children(|parent| {
            let mut volume_grid = parent.spawn((
                Grid::<Precision>::new(
                    GRID_CELL_EDGE_LENGTH_METERS,
                    GRID_SWITCHING_THRESHOLD_METERS,
                ),
                FluidVolumeGrid,
                Transform::from_translation(self.position),
                GridCell::<Precision>::default(),
            ));

            let half_size = self.size / 2;
            
            for x in -half_size.x..half_size.x {
                for y in -half_size.y..half_size.y {
                    for z in -half_size.z..half_size.z {
                        let position = Vec3::new(
                            x as f32 * GRID_CELL_EDGE_LENGTH_METERS,
                            y as f32 * GRID_CELL_EDGE_LENGTH_METERS,
                            z as f32 * GRID_CELL_EDGE_LENGTH_METERS,
                        );

                        volume_grid.with_children(|cells| {
                            cells.spawn((
                                FluidVolumeCell {
                                    temperature: self.temperature,
                                    pressure: self.pressure,
                                    density: self.density,
                                },
                                Transform::from_translation(position),
                                GridCell::<Precision>::default(),
                            ));
                        });
                    }
                }
            }
        });
    }
}

#[derive(Component)]
pub struct FluidVolumeCell {
    pub temperature: ThermodynamicTemperature,
    pub pressure: Pressure,
    pub density: MassDensity,
}

#[derive(Resource)]
pub struct DefaultFluidVolumeSettings {
    pub temperature: ThermodynamicTemperature,
    pub pressure: Pressure,
    pub density: MassDensity,
}

impl Default for DefaultFluidVolumeSettings {
    fn default() -> Self {
        Self {
            temperature: ThermodynamicTemperature::new::<kelvin>(300.0),
            pressure: Pressure::new::<pascal>(101325.0),
            density: MassDensity::new::<kilogram_per_cubic_meter>(1.225),
        }
    }
}
