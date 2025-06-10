use bevy::prelude::*;

use crate::movement::{Acceleration, MoveVelocity};

#[derive(Reflect, PartialEq)]
pub enum RotationMode {
    Direct,
    Average { time_window: f32 },
}

#[derive(Component, Reflect)]
pub struct RotateInDirectionOfMovement {
    pub previous_rot: Quat,
    pub min_speed_squared: f32,
    pub rotation_mode: RotationMode,
}
impl Default for RotateInDirectionOfMovement {
    fn default() -> Self {
        Self {
            previous_rot: Quat::IDENTITY,
            min_speed_squared: 10.,
            rotation_mode: RotationMode::Direct,
        }
    }
}

#[derive(Component, Reflect)]
pub struct TiltInDirectionOfMovement {
    current_tilt: Quat,
    max_tilt_radians: f32,
    min_speed: f32,
    max_speed: f32,
    tilt_smoothing_speed: f32,
}
impl Default for TiltInDirectionOfMovement {
    fn default() -> Self {
        Self {
            current_tilt: Quat::IDENTITY,
            max_tilt_radians: std::f32::consts::FRAC_PI_8,
            min_speed: 10.,
            max_speed: 20.,
            tilt_smoothing_speed: 10.,
        }
    }
}

#[derive(Component, Reflect)]
pub struct AverageVelOverTime {
    pub time_window: f32,
    pub average_speed: Vec3,
}
impl Default for AverageVelOverTime {
    fn default() -> Self {
        Self {
            time_window: 1.0,
            average_speed: Vec3::ZERO,
        }
    }
}

pub fn rotate_in_direction_of_movement(
    time: Res<Time>,
    mut query: Query<(
        &mut RotateInDirectionOfMovement,
        &mut Transform,
        Option<&MoveVelocity>,
        Option<&AverageVelOverTime>,
    )>,
) {
    if time.delta_secs() <= 0.0 {
        return;
    }

    for (mut rotate, mut transform, velocity, avg_velocity) in query.iter_mut() {
        let vel_per_sec = match rotate.rotation_mode {
            RotationMode::Direct => {
                if let Some(velocity) = velocity {
                    velocity.0 / time.delta_secs()
                } else {
                    continue;
                }
            },
            RotationMode::Average { .. } => {
                if let Some(avg_velocity) = avg_velocity {
                    avg_velocity.average_speed / time.delta_secs()
                } else {
                    continue;
                }
            }
        };

        if vel_per_sec.length_squared() < rotate.min_speed_squared {
            continue;
        }

        let current_angle = f32::atan2(vel_per_sec.x, vel_per_sec.z)
            - std::f32::consts::FRAC_PI_2;

        let rotation_delta = Quat::from_rotation_y(current_angle);
        transform.rotation =
            rotation_delta * rotate.previous_rot.inverse() * transform.rotation;
        rotate.previous_rot = rotation_delta;
    }
}

pub fn update_average_velocity(
    time: Res<Time>,
    mut query: Query<(&mut AverageVelOverTime, &MoveVelocity)>,
) {
    let dt = time.delta_secs();
    if dt <= 0.0 {
        return;
    }

    for (mut avg_vel, velocity) in query.iter_mut() {
        let decay_factor = dt / avg_vel.time_window;
        let clamped_decay = decay_factor.clamp(0.0, 1.0);

        avg_vel.average_speed =
            avg_vel.average_speed.lerp(velocity.0, clamped_decay);
    }
}

pub fn add_average_velocity_component(
    mut commands: Commands,
    query: Query<
        (Entity, &RotateInDirectionOfMovement),
        (Without<AverageVelOverTime>, Changed<RotateInDirectionOfMovement>),
    >,
) {
    for (entity, rotate) in query.iter() {
        if let RotationMode::Average { time_window } = rotate.rotation_mode {
            commands
                .entity(entity)
                .insert(AverageVelOverTime {
                    time_window,
                    average_speed: Vec3::ZERO,
                });
        }
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
    for (mut tilt, mut transform, accel) in query.iter_mut() {
        transform.rotation = transform.rotation * tilt.current_tilt.inverse();

        let accel_len = accel.0.length();
        let target_rotation = if accel_len < tilt.min_speed {
            Quat::IDENTITY
        } else {
            let tilt_factor = (accel_len.min(tilt.max_speed) - tilt.min_speed)
                / (tilt.max_speed - tilt.min_speed);

            let local_accel = transform.rotation.inverse()
                * Vec3::new(accel.0.x, 0.0, accel.0.z);
            let accel_direction = local_accel.normalize_or(Vec3::Y);
            let tilt_axis = Vec3::Y.cross(accel_direction.into());
            if tilt_axis.length_squared() > 0.0 {
                Quat::from_axis_angle(
                    tilt_axis.normalize(),
                    tilt_factor * tilt.max_tilt_radians,
                )
            } else {
                Quat::IDENTITY
            }
        };

        // Calculate and store new tilt
        let new_tilt = tilt.current_tilt.slerp(
            target_rotation,
            tilt.tilt_smoothing_speed * time.delta_secs(),
        );
        tilt.current_tilt = new_tilt;

        // Apply the new tilt
        transform.rotation = transform.rotation * new_tilt;
    }
}
