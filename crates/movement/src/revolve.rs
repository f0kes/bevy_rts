use crate::tween_data::{RepeatCount, RepeatStrategy, TweenData};
use bevy::prelude::*;
use std::time::Duration;

#[derive(Component)]
pub struct Revolve {
    pub angle_amplitude: f32, // Maximum rotation angle in radians
    pub axis: Dir3,           // Axis of rotation (default: Y axis)
    pub angle_offset: f32,    // Base rotation offset
    pub tween_data: TweenData,
    pub last_angle: f32,
}

impl Default for Revolve {
    fn default() -> Self {
        Self {
            angle_amplitude: std::f32::consts::PI * 2.0, // full 
            axis: Dir3::Y, // Revolve around Y axis by default
            angle_offset: 0.0,
            last_angle: 0.0,
            tween_data: TweenData {
                ease_function: EaseFunction::Linear,
                repeat: RepeatCount::Infinite,
                repeat_strategy: RepeatStrategy::Restart,
                duration: Duration::from_secs_f32(6.0),
                ..Default::default()
            }
            .with_random_time(),
        }
    }
}

pub fn revolve_system(
    time: Res<Time>,
    mut query: Query<(&mut Revolve, &mut Transform)>,
) {
    for (mut revolve, mut transform) in query.iter_mut() {
        revolve.tween_data.time += time.delta_secs();
        let eased_t = revolve.tween_data.sample();
        let angle = eased_t * revolve.angle_amplitude;

        // Remove previous rotation
        transform.rotate_axis(revolve.axis, -revolve.last_angle);

        // Apply new rotation
        let new_angle = angle + revolve.angle_offset;
        transform.rotate_axis(revolve.axis, new_angle);

        revolve.last_angle = new_angle;
    }
}
