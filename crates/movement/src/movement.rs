use crate::{
    kinematic_character_controller::MoveVelocity, plugin::MovementPluginConfig,
};
use avian3d::prelude::*;
use bevy::prelude::*;
use world_gen::terrain::Terrain;
use world_gen::terrain::TerrainLike;

#[derive(Component, Debug, Reflect)]
pub struct Acceleration(pub Vec3);
#[derive(Component, Debug, Reflect)]
pub struct ApplyGravity;
#[derive(Component, Debug, Reflect)]
pub struct GlueToGround {
    last_height: f32,
}
impl Default for GlueToGround {
    fn default() -> Self {
        Self { last_height: 0.0 }
    }
}

pub trait MoveInput: Component {
    fn direction(&self) -> Vec3;
}

pub fn move_unit<T: MoveInput>(
    mut commands: Commands,
    time: Res<Time>,
    movement_config: Res<MovementPluginConfig>,
    mut query: Query<(Entity, &mut MoveVelocity, &T)>,
) {
    if time.delta_seconds() <= 0.0 {
        return;
    }
    for (entity, mut velocity, input) in query.iter_mut() {
        let mut direction = input.direction();

        if direction.length_squared() > 0.1 {
            if direction.length_squared() > 1.0 {
                direction = direction.normalize();
            }
            let accel = direction * movement_config.default_acceleration;
            let mut velocity_vec = velocity.0;

            let new_velocity = velocity_vec + accel * time.delta_seconds();
            let clamped_velocity =
                if new_velocity.length() <= movement_config.default_max_speed {
                    new_velocity
                } else {
                    new_velocity.normalize() * velocity_vec.length()
                };
            velocity.0 = clamped_velocity;
            commands.entity(entity).insert(Acceleration(accel));
        } else {
            let horizontal_velocity =
                Vec3::new(velocity.0.x, 0.0, velocity.0.z);
            let decel = horizontal_velocity.normalize_or_zero()
                * movement_config.default_deceleration;
            let new_velocity =
                (horizontal_velocity - decel * time.delta_seconds());

            // Only modify X and Z components if they need to be zeroed
            if new_velocity.length() < 1.0 {
                velocity.0.x = 0.0;
                velocity.0.z = 0.0;
            } else {
                velocity.0.x = new_velocity.x;
                velocity.0.z = new_velocity.z;
            }
            commands.entity(entity).insert(Acceleration(decel));
            //commands.entity(entity).remove::<Acceleration>();
        }
    }
}

pub fn apply_gravity(
    time: Res<Time>,
    gravity: Res<Gravity>,
    mut query: Query<(Entity, &mut MoveVelocity), With<ApplyGravity>>,
) {
    let dt = time.delta_seconds();

    for (entity, mut velocity) in query.iter_mut() {
        velocity.0 += gravity.0 * dt;
    }
}
pub fn glue_to_ground(
    terrain: Res<Terrain>,
    mut query: Query<(&mut Transform, &mut GlueToGround)>,
) {
    for (mut transform, mut gtg) in query.iter_mut() {
        let height = terrain
            .get_height(transform.translation.x, transform.translation.z);
        
        transform.translation.y += height - gtg.last_height;
        gtg.last_height = height;
    }
}
