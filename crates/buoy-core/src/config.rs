use bevy::prelude::*;

pub(super) fn plugin(_app: &mut App) {
    // // Ensure the asset type is known to Bevy.
    // app.init_asset::<GasPropertiesConfig>();
    // // Register a RON loader for the type. Use the standard `ron` extension.
    // app.add_plugins(RonAssetPlugin::<GasPropertiesConfig>::new(&["ron"]));
}

// #[derive(Deserialize, Debug, Asset, TypePath, Clone)]
// pub struct GasSpeciesConfig {
//     pub name: String,
//     pub abbreviation: String,
//     pub molar_mass: f32, // [kg/mol]
// }

// impl GasSpeciesConfig {
//     pub fn to_species(&self) -> GasSpecies {
//         GasSpecies {
//             name: self.name.clone(),
//             abbreviation: self.abbreviation.clone(),
//             molar_mass: MolarMass::new::<kilogram_per_mole>(self.molar_mass),
//         }
//     }
// }

// #[derive(Deserialize, Debug, Asset, TypePath, Clone)]
// pub struct GasPropertiesConfig {
//     pub gases: Vec<GasSpeciesConfig>,
//     // materials: Vec<MaterialConfig>, // can be added later
// }
