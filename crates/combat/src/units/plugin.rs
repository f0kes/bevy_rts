use bevy::prelude::*;

use super::unit::{spawn_units, UnitModels};

pub struct UnitsPlugin;
impl Plugin for UnitsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UnitModels>();
        app.add_systems(Update, spawn_units);
    }
}
