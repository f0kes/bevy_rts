use bevy::prelude::*;

use super::{spell::add_spell_component, vacuum::cast_vacuum};

pub struct SpellsPlugin;
impl Plugin for SpellsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, add_spell_component);
        app.add_systems(Update, cast_vacuum);
    }
}
