use crate::plugin::MovementPluginConfig;
use avian3d::prelude::*;
use bevy::prelude::*;

#[derive(Component, Debug, Reflect)]
pub struct Acceleration(pub Vec3);

pub trait MoveInput: Component {
    fn direction(&self) -> Vec3;
}

pub fn move_unit<T: MoveInput>(
    mut commands: Commands,
    time: Res<Time>,
    movement_config: Res<MovementPluginConfig>,
    mut query: Query<(Entity, Option<&mut LinearVelocity>, &T)>,
) {
    for (entity, velocity_opt, input) in query.iter_mut() {
        let mut direction = input.direction();

        if direction.length_squared() > 0.1 {
            if direction.length_squared() > 1.0 {
                direction = direction.normalize();
            }
            let accel = direction * movement_config.default_acceleration;
            let mut velocity_vec = Vec3::ZERO;
            if let Some(velocity) = velocity_opt {
                velocity_vec = velocity.0;
            }
            let new_velocity = velocity_vec + accel * time.delta_seconds();
            let clamped_velocity =
                if new_velocity.length() <= movement_config.default_max_speed {
                    new_velocity
                } else {
                    new_velocity.normalize() * velocity_vec.length()
                };
            commands
                .entity(entity)
                .insert(LinearVelocity(clamped_velocity))
                .insert(Acceleration(accel));
        } else {
            if let Some(velocity) = velocity_opt {
                let decel = velocity.0.normalize_or_zero()
                    * movement_config.default_deceleration;
                let new_velocity = velocity.0 - decel * time.delta_seconds();
                let clamped_velocity = if new_velocity.length() < 1.0 {
                    Vec3::new(0.0, velocity.0.y, 0.0)
                } else {
                    new_velocity
                };
                commands
                    .entity(entity)
                    .insert(LinearVelocity(clamped_velocity))
                    .insert(Acceleration(decel));
            }
        }
    }
}

pub fn apply_gravity(
    mut commands: Commands,
    time: Res<Time>,
    gravity: Res<Gravity>,
    mut query: Query<(Entity, &mut LinearVelocity)>,
) {
    for (entity, velocity) in query.iter_mut() {
        let new_velocity = velocity.0 + gravity.0 * time.delta_seconds();
        commands.entity(entity).insert(LinearVelocity(new_velocity));
    }
}
