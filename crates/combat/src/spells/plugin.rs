use bevy::prelude::*;
use meta_components::temporal::TemporaryAppExt;
use movement::movement::CantMove;

use super::{
    continuos_actions::process_continuous_actions, spell::add_spell_component, summon::cast_summon, vacuum::cast_vacuum
};

pub struct SpellsPlugin;
impl Plugin for SpellsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, add_spell_component);
        app.add_systems(Update, cast_vacuum);
        app.add_systems(Update, cast_summon);
        app.add_systems(Update, process_continuous_actions);
        app.add_temporary::<CantMove>();
    }
}
