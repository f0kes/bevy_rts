use crate::actions::Actions;
use crate::animation_defintions::{AnimationType, Character};
use crate::loading::TextureAssets;
use crate::GameState;
use bevy::prelude::*;
use directional_animation::ron_generation::animator::MovementDirection;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_player)
            .add_systems(Update, move_player.run_if(in_state(GameState::Playing)));
    }
}

fn spawn_player(mut commands: Commands) {
    commands
        .spawn(Player)
        .insert(SpriteBundle {
            transform: Transform::from_scale(Vec3::ONE * 1.0),
            ..Default::default()
        })
        .insert(Character::Wolf)
        .insert(AnimationType::Running)
        .insert(MovementDirection {
            direction: Vec3::new(0., 0., 0.),
        });
}

fn move_player(
    time: Res<Time>,
    actions: Res<Actions>,
    mut player_query: Query<(&mut Transform, Option<&mut MovementDirection>), With<Player>>,
) {
    if actions.player_movement.is_none() {
        return;
    }
    let speed = 150.;
    let movement = Vec3::new(
        actions.player_movement.unwrap().x * speed * time.delta_seconds(),
        actions.player_movement.unwrap().y * speed * time.delta_seconds(),
        0.,
    );
    for (mut player_transform, option_direction) in &mut player_query {
        player_transform.translation += movement;
        if movement.length_squared() > 0.01 {
            if let Some(mut direction) = option_direction {
                direction.direction = movement;
            }
        }
    }
}
