use std::time::Duration;

use bevy::prelude::*;
use bevy_tweening::{lens::TransformPositionLens, EaseMethod, Tween};

use crate::tween_data::{RepeatCount, RepeatStrategy, TweenData};

#[derive(Component)]
pub struct Bob {
    pub amplitude: f32,
    pub y_offset: f32,
    pub tween_data: TweenData,
    pub last_y: f32,
}
impl Default for Bob {
    fn default() -> Self {
        Self {
            amplitude: 0.5,
            y_offset: 0.0,
            last_y: 0.0,
            tween_data: TweenData {
                ease_function: EaseFunction::SineInOut,
                repeat: RepeatCount::Infinite,
                repeat_strategy: RepeatStrategy::Reverse,
                duration: Duration::from_secs_f32(1.6),
                ..Default::default()
            }
            .with_random_time(),
        }
    }
}
pub fn bob_system(
    time: Res<Time>,
    mut query: Query<(&mut Bob, &mut Transform)>,
) {
    for (mut bob, mut transform) in query.iter_mut() {
        bob.tween_data.time += time.delta_secs();
        let eased_t = bob.tween_data.sample();
        let delta = eased_t * bob.amplitude;
        transform.translation.y -= bob.last_y;
        transform.translation.y += delta + bob.y_offset;
        bob.last_y = delta + bob.y_offset;
    }
}
