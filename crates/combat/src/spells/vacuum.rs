use bevy::prelude::*;
use movement::{
    kinematic_character_controller::MoveVelocity, movement::CursorPos,
};
use steering::{
    spatial_filters::rotated_box::{BoxData, RotatedBoxFilter},
    spatial_hashing::spatial_hashmap::SpatialHashmap,
    steering_agent::get_nearby_unit_entities_and_positions,
};

use crate::{
    inventory::Inventory, teams::Team,
    unit_filters::team_filters::TeamFilterExt, units::unit::Unit,
};

use super::spell::{Action, ActionData};

#[derive(Component, Clone, Copy)]
pub struct VacuumSpell {
    pub range: f32,
    pub width: f32,
    pub pull_force: f32,
}

pub fn cast_vacuum(
    spell: Query<(Entity, &VacuumSpell, &Action, &ActionData)>,
    transforms: Query<(Entity, &mut Transform)>,
    cursors: Query<(Entity, &CursorPos)>,
    teams: Query<(Entity, &Team)>,
    spatial_hashmap: Res<SpatialHashmap>,
    inventory: Query<&Inventory>,
    mut move_velocities: Query<&mut MoveVelocity>,
) {
    let entities_and_positions: Vec<(Entity, Vec3)> = transforms
        .iter()
        .map(|(entity, transform)| (entity, transform.translation))
        .collect();

    for (spell_entity, spell, action, action_data) in spell.iter() {
        let caster = action_data.actor;
        let (
            Ok((_, caster_cursor)),
            Ok((_, caster_transform)),
            Ok((_, caster_team)),
            Ok(caster_inventory),
        ) = (
            cursors.get(caster),
            transforms.get(caster),
            teams.get(caster),
            inventory.get(caster),
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
            let (Ok((_, target_transform)), Ok(ref mut move_velocity)) =
                (transforms.get(entity), move_velocities.get_mut(entity))
            else {
                continue;
            };

            let direction = (caster_transform.translation
                - target_transform.translation)
                .normalize();
            let force = direction * spell.pull_force / distance;
            move_velocity.0 += force;
        }
    }
}
