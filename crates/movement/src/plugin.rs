use avian3d::prelude::PostProcessCollisions;
use avian3d::prelude::SolverSet;
use bevy::prelude::*;

use crate::collide_and_slide;
use crate::kinematic_character_controller::add_collide_and_slide_to_characters;
use crate::kinematic_character_controller::apply_frame_velocity;
use crate::movement::add_grounded;
use crate::movement::apply_gravity;
use crate::movement::glue_to_ground;
use crate::movement::move_unit;

use crate::collide_and_slide::collide_and_slide;
use crate::movement::MoveInput;
use crate::rotate::rotate_in_direction_of_movement;
use crate::rotate::tilt_in_direction_of_acceleration;
use crate::rotate::RotateInDirectionOfMovement;
use crate::rotate::TiltInDirectionOfMovement;
use crate::step_animation::animate_steps;
use crate::vidar_cas;
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

        // Create a base set for movement systems
        app.add_systems(
            Update,
            (
                move_unit::<T>,
                apply_gravity,
                glue_to_ground,
                add_grounded,
                rotate_in_direction_of_movement,
                tilt_in_direction_of_acceleration
                    .after(rotate_in_direction_of_movement),
                animate_steps,
            ),
        );

        app.insert_resource(self.config.clone());
        //app.add_systems(Update, add_collide_and_slide_to_characters);
        // Add collide_and_slide after all other systems that modify LinearVelocity
        /* app.add_systems(
            Update,
            collide_and_slide.after(move_unit::<T>).after(apply_gravity),
        ); */

        //app.add_systems(PostUpdate, vidar_cas::collide_and_slide_system);
        app.add_systems(
            PostUpdate,
            apply_frame_velocity.after(vidar_cas::collide_and_slide_system),
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
