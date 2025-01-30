use bevy::prelude::*;
use meta_components::in_world::InWorld;
use movement::{
    kinematic_character_controller::MoveVelocity, movement::CursorPos,
};

use crate::{
    inventory::{ChosenSlot, Inventory, Item},
    units::unit::UnitName,
};

use super::spell::ActionData;
#[derive(Component, Clone, Copy)]
pub struct SummonSpell {
    pub summon_interval: f32,
    pub last_summon_time: f32,
    pub summon_velocity: f32,
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
        //println!("casting summon spell");
        if spell.last_summon_time < spell.summon_interval {
            continue;
        }
        
        spell.last_summon_time = 0.0;
        let caster = action_data.actor;
        if let Ok((cursor, transform, mut inventory, chosen_slot)) =
            caster_query.get_mut(caster)
        {
            if let Some((item, entity)) =
                inventory.try_take_with_slot(chosen_slot.index)
            {
                info!("there's something in slot {}", chosen_slot.index);
                match item {
                    Item::Unit { name: _ } => {
                        let mut entity_commands = commands.entity(entity);
                        entity_commands.insert(InWorld);
                        entity_commands.insert(transform.clone());
                        let direction =
                            (cursor.0 - transform.translation).normalize();
                        entity_commands.insert(MoveVelocity(
                            direction * spell.summon_velocity,
                        ));
                        info!("summoned unit");
                    }
                    _ => {}
                }
            }
        }
    }
}
