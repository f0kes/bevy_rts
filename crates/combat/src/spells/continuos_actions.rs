use std::mem::discriminant;

use bevy::{
    prelude::*,
    utils::{HashMap, HashSet},
};
use input_actions::action::InputAction;

use super::spell::{
    Action, ActionBundle, ActionData, ActionType,
};
#[derive(Component,Default)]
pub struct ActiveActions(pub HashSet<InputAction>);

#[derive(Component)]
pub struct ActionMapping(pub HashMap<InputAction, Action>);
pub fn process_continuous_actions(
    mut commands: Commands,
    caster_query: Query<(Entity, &ActiveActions, &ActionMapping)>,
    game_action_query: Query<(Entity, &ActionData, &Action)>,
) {
    for (caster, active_actions, action_mapping) in caster_query.iter() {
        let mut existing_game_actions = Vec::new();
        let mut desired_game_actions = Vec::new();

        // Collect existing continuous game actions for this caster
        for (entity, action_data, game_action) in game_action_query.iter() {
            if action_data.actor == caster
                && matches!(action_data.action_type, ActionType::Continuous)
            {
                existing_game_actions.push((*game_action, entity));
            }
        }

        // Collect desired game actions from active actions
        for action in active_actions.0.iter() {
            if let Some(game_action) = action_mapping.0.get(action) {
                desired_game_actions.push(*game_action);
            }
        }

        // Spawn new actions that don't exist yet
        for action in desired_game_actions.iter() {
            if !existing_game_actions.iter().any(|(existing_action, _)| {
                discriminant(existing_action) == discriminant(action)
            }) {
                commands.spawn(ActionBundle::from_action(*action, caster));
            }
        }

        // Despawn actions that are no longer desired
        for (existing_action, entity) in existing_game_actions.iter() {
            if !desired_game_actions.contains(existing_action) {
                commands.entity(*entity).despawn();
            }
        }
    }
}

/* pub fn process_continuos<T: Component>(
    mut commands: Commands,
    input: Res<ButtonInput<InputAction>>,
    actor_query: Query<(Entity,)>,
    action_query: Query<(Entity, &ActionData), With<T>>,
) {
    let (actor_entity,) = match actor_query.get_single() {
        Ok(actor_entity) => actor_entity,
        Err(_) => return,
    };
    let mut action_exists = false;
    for (_, action_data) in action_query.iter() {
        if action_data.actor == actor_entity {
            action_exists = true;
        }
    }
    
    if input.pressed(InputAction::Collect) && !action_exists {
        commands.spawn(ActionBundle::vacuum_spell(
            VacuumSpell {
                range: 20.,
                width: 2.,
                pull_force: 2.,
                eat_range: 1.,
            },
            player,
        ));
    } else if !input.pressed(InputAction::Collect) {
        for (entity, _) in action_query.iter() {
            commands.entity(entity).despawn();
        }
    }
} */
