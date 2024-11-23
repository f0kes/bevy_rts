use bevy::prelude::*;

use crate::movement::move_unit;

use crate::movement::MoveInput;
use crate::rotate::rotate_in_direction_of_movement;
use crate::rotate::tilt_in_direction_of_acceleration;
use crate::rotate::RotateInDirectionOfMovement;
use crate::rotate::TiltInDirectionOfMovement;
const ACCELERATION: f32 = 15.0;
const MAX_SPEED: f32 = 10.0;
const DECELERATION: f32 = 15.0;

#[derive(Default)]
pub struct MovementPlugin<T: MoveInput> {
    pub config: MovementPluginConfig,
    pub _marker: std::marker::PhantomData<T>,
}

impl<T: MoveInput> Plugin for MovementPlugin<T> {
    fn build(&self, app: &mut App) {
        app.register_type::<RotateInDirectionOfMovement>();
        app.register_type::<TiltInDirectionOfMovement>();
        app.add_systems(Update, move_unit::<T>);
        //app.add_systems(Update, apply_gravity);
        app.insert_resource(self.config.clone());
        app.add_systems(Update, rotate_in_direction_of_movement);
        app.add_systems(
            Update,
            tilt_in_direction_of_acceleration
                .after(rotate_in_direction_of_movement),
        );
    }
}

impl<T: MoveInput> MovementPlugin<T> {
    pub fn new(config: MovementPluginConfig) -> Self {
        Self {
            config,
            _marker: std::marker::PhantomData,
        }
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
