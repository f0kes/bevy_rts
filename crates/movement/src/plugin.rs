use bevy::prelude::*;

use crate::bob::bob_system;
use crate::constraint::MovementConstraint;
use crate::follow::follow_target;
use crate::follow_rotation::follow_rotation;
use crate::movement::add_grounded;
use crate::movement::apply_frame_velocity;
use crate::movement::apply_frame_velocity_constrained;
use crate::movement::apply_gravity;
use crate::movement::glue_to_ground;
use crate::movement::insert_default_movement_stats;
use crate::movement::move_unit;

use crate::movement::MoveInput;
use crate::revolve::revolve_system;
use crate::rotate::add_average_velocity_component;
use crate::rotate::rotate_in_direction_of_movement;
use crate::rotate::tilt_in_direction_of_acceleration;
use crate::rotate::update_average_velocity;
use crate::rotate::RotateInDirectionOfMovement;
use crate::rotate::TiltInDirectionOfMovement;
use crate::step_animation::animate_steps;
const ACCELERATION: f32 = 15.0;
const MAX_SPEED: f32 = 10.0;
const DECELERATION: f32 = 15.0;

pub struct MovementBasePlugin<T: MoveInput> {
    pub config: MovementPluginConfig,
    pub _marker: std::marker::PhantomData<T>,
}

// Unconstrained movement plugin
pub struct MovementPlugin<T: MoveInput>(MovementBasePlugin<T>);

// Constrai\ned movement plugin
pub struct ConstrainedMovementPlugin<
    T: MoveInput,
    C: MovementConstraint + Resource,
> {
    base: MovementBasePlugin<T>,
    _marker: std::marker::PhantomData<C>,
}

impl<T: MoveInput> MovementBasePlugin<T> {
    pub fn new(config: MovementPluginConfig) -> Self {
        Self {
            config,
            _marker: std::marker::PhantomData,
        }
    }

    fn build(&self, app: &mut App) {
        app.register_type::<RotateInDirectionOfMovement>();
        app.register_type::<TiltInDirectionOfMovement>();

        app.add_systems(
            Update,
            (
                move_unit::<T>,
                insert_default_movement_stats,
                apply_gravity,
                glue_to_ground,
                add_grounded,
                rotate_in_direction_of_movement,
                add_average_velocity_component,
                tilt_in_direction_of_acceleration
                    .after(rotate_in_direction_of_movement),
                update_average_velocity.before(rotate_in_direction_of_movement),
                animate_steps,
                follow_target,
                follow_rotation,
                bob_system,
                revolve_system,
            ),
        );

        app.insert_resource(self.config.clone());
    }
}

impl<T: MoveInput> MovementPlugin<T> {
    pub fn new(config: MovementPluginConfig) -> Self {
        Self(MovementBasePlugin::new(config))
    }
}

impl<T: MoveInput, C: MovementConstraint + Resource>
    ConstrainedMovementPlugin<T, C>
{
    pub fn new(config: MovementPluginConfig) -> Self {
        Self {
            base: MovementBasePlugin::new(config),
            _marker: std::marker::PhantomData,
        }
    }
}

impl<T: MoveInput> Plugin for MovementPlugin<T> {
    fn build(&self, app: &mut App) {
        self.0.build(app);
        app.add_systems(PostUpdate, apply_frame_velocity);
    }
}

impl<T: MoveInput, C: MovementConstraint + Resource> Plugin
    for ConstrainedMovementPlugin<T, C>
{
    fn build(&self, app: &mut App) {
        self.base.build(app);
        app.add_systems(PostUpdate, apply_frame_velocity_constrained::<C>);
    }
}

#[derive(Resource, Clone)]
pub struct MovementPluginConfig {
    pub default_acceleration: f32,
    pub default_max_speed: f32,
    pub default_deceleration: f32,
}

impl Default for MovementPluginConfig {
    fn default() -> Self {
        Self {
            default_acceleration: ACCELERATION,
            default_max_speed: MAX_SPEED,
            default_deceleration: DECELERATION,
        }
    }
}
