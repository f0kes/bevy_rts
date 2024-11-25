use bevy::prelude::*;

use crate::collide_and_slide::CollideAndSlide;

#[derive(Component)]
pub struct KinematicCharacterController {
    pub max_slope: f32,
    pub max_step_height: f32,
    pub max_iterations: u32,
    pub epsilon: f32,
    pub skin_width: f32,
}
impl Default for KinematicCharacterController {
    fn default() -> Self {
        Self {
            max_slope: 45.0f32.to_radians(),
            max_step_height: 0.5,
            max_iterations: 4,
            epsilon: 0.01,
            skin_width: 0.1,
        }
    }
}

#[derive(Bundle, Default)]
pub struct KinematicCharacterControllerBundle {
    pub controller: KinematicCharacterController,
    pub velocity: FrameVelocity,
}
#[derive(Component, Default)]
pub struct FrameVelocity(pub Vec3);

pub fn apply_frame_velocity(
    mut query: Query<(&mut Transform, &FrameVelocity)>,
) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation += velocity.0;
    }
}

pub fn add_collide_and_slide_to_characters(
    mut commands: Commands,
    query: Query<
        (Entity, &KinematicCharacterController),
        Without<CollideAndSlide>,
    >,
) {
    for (entity, kcc) in query.iter() {
        commands.entity(entity).insert(CollideAndSlide {
            skin_width: kcc.skin_width,
            max_iterations: kcc.max_iterations,
            epsilon: kcc.epsilon,
        });
    }
}
