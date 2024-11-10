use bevy::prelude::*;

use super::{
    animation_library::{AnimationLibrary, MyAnimationClip},
    AnimationTypes,
};

#[derive(Component)]
pub struct MovementDirection {
    pub direction: Vec3,
}
#[derive(Component)]
pub struct AnimationTimer {
    pub timer: Timer,
    pub current_frame: usize,
}

pub fn change_animation<T: AnimationTypes>(
    mut commands: Commands,
    query: Query<
        (
            Entity,
            &T::CharacterName,
            &T::AnimationName,
            &MovementDirection,
            Option<&AnimationTimer>,
        ),
        Or<(
            Changed<T::CharacterName>,
            Changed<T::AnimationName>,
            Changed<MovementDirection>,
        )>,
    >,
    animation_library: Res<AnimationLibrary<T>>,
) {
    for (entity, character, animation, movement_direction, timer) in query.iter() {
        if let Some(animation_clip) =
            animation_library.find_animation(character, animation, movement_direction.direction)
        {
            let mut index = 0;
            if let Some(timer) = timer {
                index = timer.current_frame.clone();
            }
            commands.entity(entity).insert(animation_clip.clone());
            commands
                .entity(entity)
                .insert(animation_clip.texture_atlas.clone());
            commands.entity(entity).insert(TextureAtlas {
                layout: animation_clip.texture_atlas_layout_handle.clone(),
                index,
            });

            if timer.is_none() {
                commands.entity(entity).insert(AnimationTimer {
                    timer: Timer::from_seconds(1.0 / animation_clip.fps, TimerMode::Repeating),
                    current_frame: index,
                });
            }
        }
    }
}
pub fn animate(
    time: Res<Time>,
    mut query: Query<(&MyAnimationClip, &mut AnimationTimer, &mut TextureAtlas)>,
) {
    for (clip, mut timer, mut atlas) in &mut query {
        timer.timer.tick(time.delta());
        if timer.timer.just_finished() {
            timer.current_frame = if timer.current_frame == clip.len - 1 {
                0
            } else {
                timer.current_frame + 1
            };
            atlas.index = timer.current_frame;
        }
    }
}
