use bevy::prelude::*;
use meta_components::temporal::Temporary;
use movement::{
    kinematic_character_controller::MoveVelocity,
    movement::{CantMove, CursorPos},
};
use steering::{
    spatial_filters::rotated_box::{BoxData, RotatedBoxFilter},
    spatial_hashing::spatial_hashmap::SpatialHashmap,
    steering_agent::get_nearby_unit_entities_and_positions,
};

use crate::{
    inventory::{systems::AddToInventory, Inventory, Item},
    teams::Team,
    unit_filters::team_filters::TeamFilterExt,
    units::unit::Unit,
};

use super::spell::{Action, ActionData};

#[derive(Component, Clone, Copy)]
pub struct VacuumSpell {
    pub range: f32,
    pub width: f32,
    pub pull_force: f32,
    pub eat_range: f32,
}

pub fn cast_vacuum(
    mut commands: Commands,
    spell: Query<(&VacuumSpell, &ActionData)>,
    transforms: Query<(Entity, &mut Transform)>,
    cursors: Query<(Entity, &CursorPos)>,
    teams: Query<(Entity, &Team)>,
    spatial_hashmap: Res<SpatialHashmap>,
    mut inventory: Query<&mut Inventory>,
    mut move_velocities: Query<&mut MoveVelocity>,
    units: Query<&Unit>,
) {
    let entities_and_positions: Vec<(Entity, Vec3)> = transforms
        .iter()
        .map(|(entity, transform)| (entity, transform.translation))
        .collect();

    for (spell, action_data) in spell.iter() {
        let caster = action_data.actor;
        let (
            Ok((_, caster_cursor)),
            Ok((_, caster_transform)),
            Ok((_, caster_team)),
            Ok(mut caster_inventory),
        ) = (
            cursors.get(caster),
            transforms.get(caster),
            teams.get(caster),
            inventory.get_mut(caster),
        )
        else {
            continue;
        };

        let distances = get_nearby_unit_entities_and_positions(
            spatial_hashmap.as_ref(),
            caster,
            caster_transform.translation,
            spell.range,
            &entities_and_positions,
            100000,
        )
        .in_rotated_box(BoxData {
            start: caster_transform.translation,
            end: caster_cursor.0,
            dimensions: Vec3::new(spell.width, spell.width, spell.range),
        })
        .same_team(&teams, caster_team)
        .map(|(entity, position)| {
            let distance = (position - caster_transform.translation).length();
            (entity, distance)
        });

        for (entity, distance) in distances {
            let (
                Ok((_, target_transform)),
                Ok(ref mut move_velocity),
                Ok(unit),
            ) = (
                transforms.get(entity),
                move_velocities.get_mut(entity),
                units.get(entity),
            )
            else {
                continue;
            };

            let direction = (caster_transform.translation
                - target_transform.translation)
                .normalize();
            let force = direction * spell.pull_force / distance;
            move_velocity.0 += force;
            commands.entity(entity).insert(Temporary {
                value: CantMove,
                time_left: 0.5,
            });
            //1. if the unit is close enough to the caster,
            //   add it to the caster's inventory
            if distance < spell.eat_range {
                println!("adding_to_inv");
                commands.entity(entity).insert(AddToInventory {
                    target_inventory: caster,
                    item: Item::Unit {
                        name: unit.unit_name,
                    },
                    count: 1,
                });
            };
        }
    }
}
