use avian3d::prelude::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct RotateInDirectionOfMovement;

pub fn rotate_in_direction_of_movement(
    mut query: Query<(&RotateInDirectionOfMovement, &mut Transform, &LinearVelocity)>,
) {
    for (_, mut transform, velocity) in query.iter_mut() {
        if velocity.0.length() > 0.0 {
            transform.rotation = Quat::from_rotation_y(
                velocity.0.angle_between(Vec3::new(1.0, 0.0, 0.0)),
            );
        }
    }
}
