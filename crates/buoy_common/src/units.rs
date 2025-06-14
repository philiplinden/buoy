use bevy::prelude::*;
use std::fmt::Display;
use uom::si::Quantity;

#[derive(Default)]
pub struct FormattedUnitsPlugin;

impl Plugin for FormattedUnitsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<UomQuantity>();
    }
}

#[derive(Component, Debug, Reflect, Default)]
pub struct UomQuantity {
    value: f32,
    unit: String,
}

impl UomQuantity {
    pub fn new<D, U, V>(quantity: &Quantity<D, U, V>) -> Self
    where
        D: uom::si::Dimension + ?Sized,
        U: uom::si::Units<V> + ?Sized + uom::si::Unit,
        V: uom::num::Num + uom::Conversion<V> + Into<f32> + Clone,
    {
        Self {
            value: quantity.value.clone().into(),
            unit: U::abbreviation().to_string(),
        }
    }

    /// Get the raw value of the quantity
    pub fn value(&self) -> f32 {
        self.value
    }

    /// Get the unit abbreviation
    pub fn unit(&self) -> &str {
        &self.unit
    }
}

impl Display for UomQuantity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.value, self.unit)
    }
}
