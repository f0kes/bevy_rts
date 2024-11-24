use avian3d::prelude::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct StepAnimation {
    pub step_frequency: f32,    // How many steps per second
    pub step_height: f32,       // Maximum height of each step
    pub phase: f32,            // Current phase of the stepping animation (0.0 to 2π)
}

impl Default for StepAnimation {
    fn default() -> Self {
        Self {
            step_frequency: 5.0,
            step_height: 0.1,
            phase: 0.0,
        }
    }
}

pub fn animate_steps(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut StepAnimation, &LinearVelocity)>,
) {
    for (mut transform, mut step_animation, velocity) in query.iter_mut() {
        let speed = velocity.0.length();
        
        // Only animate steps when moving
        if speed > 0.1 {
            // Update the phase based on movement speed
            step_animation.phase += time.delta_seconds() * step_animation.step_frequency * speed;
            step_animation.phase %= std::f32::consts::TAU; // Keep phase between 0 and 2π
            
            // Calculate vertical offset using a sine wave
            let height_offset = (step_animation.phase.sin() + 1.0) * 0.5 * step_animation.step_height;
            
            // Apply the vertical offset to the transform
            transform.translation.y = height_offset;
        } else {
            // Reset height when not moving
            transform.translation.y = 0.0;
            step_animation.phase = 0.0;
        }
    }
}