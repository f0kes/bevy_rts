use std::f32::consts;

use avian3d::prelude::*;
use bevy::prelude::*;

use crate::movement::Acceleration;

#[derive(Component, Reflect)]
pub struct RotateInDirectionOfMovement {
    previous_rot: Quat,
    min_speed_squared: f32,
}
impl Default for RotateInDirectionOfMovement {
    fn default() -> Self {
        Self {
            previous_rot: Quat::IDENTITY,
            min_speed_squared: 10.,
        }
    }
}

#[derive(Component, Reflect)]
pub struct TiltInDirectionOfMovement {
    previous_rot: Quat,
    max_tilt_radians: f32,
    min_speed: f32,
    max_speed: f32,
    tilt_smoothing: f32,
}
impl Default for TiltInDirectionOfMovement {
    fn default() -> Self {
        Self {
            previous_rot: Quat::IDENTITY,
            max_tilt_radians: std::f32::consts::FRAC_PI_4,
            min_speed: 10.,
            max_speed: 20.,
            tilt_smoothing: 0.1,
        }
    }
}

pub fn rotate_in_direction_of_movement(
    mut query: Query<(
        &mut RotateInDirectionOfMovement,
        &mut Transform,
        &LinearVelocity,
    )>,
) {
    for (mut rotate, mut transform, velocity) in query.iter_mut() {
        if velocity.0.length_squared() < rotate.min_speed_squared {
            continue;
        }

        let current_angle = f32::atan2(velocity.0.x, velocity.0.z)
            - std::f32::consts::FRAC_PI_2;
        let velocity_xz = Vec3::new(velocity.0.x, velocity.0.y, velocity.0.z);
        let local_forward_to_velocity =
            Quat::from_rotation_arc(transform.forward().into(), velocity_xz);
        let rotation_delta = Quat::from_rotation_y(current_angle);
        transform.rotation =
            rotation_delta * rotate.previous_rot.inverse() * transform.rotation;
        rotate.previous_rot = rotation_delta;
    }
}

pub fn tilt_in_direction_of_acceleration(
    time: Res<Time>,
    mut query: Query<(
        &mut TiltInDirectionOfMovement,
        &mut Transform,
        &Acceleration,
    )>,
) {
    let dt = time.delta_seconds() * 60.0;

    for (mut tilt, mut transform, accel) in query.iter_mut() {
        let accel_len = accel.0.length();
        let target_rotation = if accel_len < tilt.min_speed {
            Quat::IDENTITY
        } else {
            let tilt_factor = (accel_len.min(tilt.max_speed) - tilt.min_speed)
                / (tilt.max_speed - tilt.min_speed);

            let accel_direction =
                Vec3::new(accel.0.x, 0.0, accel.0.z).normalize_or(Vec3::Y);
            let tilt_axis = Vec3::Y.cross(accel_direction);

            if tilt_axis.length_squared() > 0.0 {
                Quat::from_axis_angle(
                    tilt_axis.normalize(),
                    tilt_factor * tilt.max_tilt_radians,
                )
            } else {
                Quat::IDENTITY
            }
        };

        transform.rotation =
            target_rotation * tilt.previous_rot.inverse() * transform.rotation;
        tilt.previous_rot = target_rotation;
    }
}
