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
pub struct CursorPos(pub Vec3);

#[derive(Component, Debug, Reflect,Clone, Copy)]
pub struct CantMove;
pub fn move_unit<T: MoveInput>(
    mut commands: Commands,
    time: Res<Time>,
    movement_config: Res<MovementPluginConfig>,
    mut query: Query<(Entity, &mut MoveVelocity, &T, Option<&CantMove>)>,
) {
    if time.delta_seconds() <= 0.0 {
        return;
    }
    for (entity, mut velocity, input, cant_move_opt) in query.iter_mut() {
        if cant_move_opt.is_some() {
            continue;
        }
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

    mut query: Query<(Entity, &mut MoveVelocity), With<ApplyGravity>>,
    grounded: Query<&Grounded>,
) {
    let dt = time.delta_seconds();

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
        (With<GlueToGround>),
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
