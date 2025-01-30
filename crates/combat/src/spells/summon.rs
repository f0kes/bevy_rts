use bevy::prelude::*;
use movement::movement::CursorPos;

use crate::inventory::{ChosenSlot, Inventory};

use super::spell::ActionData;
#[derive(Component, Clone, Copy)]
pub struct SummonSpell {
    pub summon_interval: f32,
    pub last_summon_time: f32,
}
pub fn cast_summon(
    mut commands: Commands,
    mut spell: Query<(&mut SummonSpell, &ActionData)>,
    mut caster_query: Query<(
        &CursorPos,
        &Transform,
        &mut Inventory,
        &ChosenSlot,
    )>,
    time: Res<Time>,
) {
    for (mut spell, action_data) in spell.iter_mut() {
        spell.last_summon_time += time.delta_seconds();
        if spell.last_summon_time < spell.summon_interval {
            continue;
        }
        spell.last_summon_time = 0.0;
        let caster = action_data.actor;
        if let Ok((cursor, transform, mut inventory, chosen_slot)) =
            caster_query.get_mut(caster)
        {
            
        }
    }
}
