use crate::constraint::MovementConstraint;
use crate::plugin::MovementPluginConfig;
use avian3d::parry::query;
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
#[derive(Component, Debug, Reflect)]
pub struct Grounded;

pub trait MoveInput: Component {
    fn direction(&self) -> Vec3;
}

#[derive(Component)]
pub struct Move(pub Vec3);
impl MoveInput for Move {
    fn direction(&self) -> Vec3 {
        self.0
    }
}
#[derive(Component)]
pub struct MaxSpeed(pub f32);
#[derive(Component)]
pub struct AccelerationRate(pub f32);
#[derive(Component)]
pub struct DecelerationRate(pub f32);
#[derive(Component)]
pub struct MoveVelocity(pub Vec3);
#[derive(Component)]
pub struct CursorPos(pub Vec3);

#[derive(Component, Debug, Reflect, Clone, Copy)]
pub struct CantMove;

pub fn move_unit<T: MoveInput>(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(
        Entity,
        &mut MoveVelocity,
        &T,
        Option<&CantMove>,
        &MaxSpeed,
        &AccelerationRate,
        &DecelerationRate,
    )>,
) {
    if time.delta_secs() <= 0.0 {
        return;
    }
    for (
        entity,
        mut velocity,
        input,
        cant_move_opt,
        max_speed,
        accel_rate,
        decel_rate,
    ) in query.iter_mut()
    {
        if cant_move_opt.is_some() {
            continue;
        }
        let mut direction = input.direction();

        if direction.length_squared() > 0.1 {
            if direction.length_squared() > 1.0 {
                direction = direction.normalize();
            }
            let accel = direction * accel_rate.0;
            let velocity_vec = velocity.0;

            let new_velocity = velocity_vec + accel * time.delta_secs();
            let clamped_velocity = if new_velocity.length() <= max_speed.0 {
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
                * decel_rate.0
                * time.delta_secs();
            let new_velocity = horizontal_velocity - decel;
            if decel.length() > horizontal_velocity.length()
                || new_velocity.length() < 1.0
            {
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
pub fn insert_default_movement_stats(
    movement_config: Res<MovementPluginConfig>,
    mut commands: Commands,
    query: Query<
        (
            Entity,
            Option<&MaxSpeed>,
            Option<&AccelerationRate>,
            Option<&DecelerationRate>,
        ),
        With<MoveVelocity>,
    >,
) {
    for (entity, max_speed, accel_rate, decel_rate) in query.iter() {
        if max_speed.is_none() {
            commands
                .entity(entity)
                .insert(MaxSpeed(movement_config.default_max_speed));
        }
        if accel_rate.is_none() {
            commands
                .entity(entity)
                .insert(AccelerationRate(movement_config.default_acceleration));
        }
        if decel_rate.is_none() {
            commands
                .entity(entity)
                .insert(DecelerationRate(movement_config.default_deceleration));
        }
    }
}

pub fn apply_gravity(
    time: Res<Time>,

    mut query: Query<(Entity, &mut MoveVelocity), With<ApplyGravity>>,
    grounded: Query<&Grounded>,
) {
    let dt = time.delta_secs();

    for (entity, mut velocity) in query.iter_mut() {
        if grounded.get(entity).is_ok() {
            velocity.0.y = 0.0;
        } else {
            velocity.0.y -= 9.81 * dt;
        }
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
        //transform.translation.y = height;
    }
}

pub fn add_grounded(
    mut commands: Commands,
    terrain: Res<Terrain>,
    mut query: Query<
        (Entity, &mut Transform, &mut MoveVelocity),
        With<GlueToGround>,
    >,
) {
    for (entity, mut transform, mut velocity) in query.iter_mut() {
        let height = terrain
            .get_height(transform.translation.x, transform.translation.z);
        if transform.translation.y <= height {
            velocity.0.y = 0.0;
            transform.translation.y = height;
            commands.entity(entity).insert(Grounded);
        } else {
            commands.entity(entity).remove::<Grounded>();
        }
    }
}

pub fn apply_frame_velocity(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &MoveVelocity)>,
) {
    let dt = time.delta_secs();
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation += velocity.0 * dt;
    }
}

pub fn apply_frame_velocity_constrained<T: MovementConstraint + Resource>(
    time: Res<Time>,
    constraint: Res<T>,
    mut query: Query<(&mut Transform, &mut MoveVelocity)>,
) {
    let dt = time.delta_secs();
    for (mut transform, mut velocity) in query.iter_mut() {
        let intended_pos = transform.translation + velocity.0 * dt;
        if constraint.is_move_allowed(transform.translation, intended_pos) {
            transform.translation = intended_pos;
        } else {
            // Try sliding along X axis only
            let x_move = Vec3::new(velocity.0.x * dt, 0.0, 0.0);
            let x_pos = transform.translation + x_move;
            if constraint.is_move_allowed(transform.translation, x_pos) {
                transform.translation = x_pos;
                velocity.0.z = 0.0; // Zero out z velocity
            } else {
                velocity.0.x = 0.0; // Zero out x velocity
            }

            // Try sliding along Z axis only
            let z_move = Vec3::new(0.0, 0.0, velocity.0.z * dt);
            let z_pos = transform.translation + z_move;
            if constraint.is_move_allowed(transform.translation, z_pos) {
                transform.translation = z_pos;
            } else {
                velocity.0.z = 0.0; // Zero out z velocity
            }
        }
    }
}
