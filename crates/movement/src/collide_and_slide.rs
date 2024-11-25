use avian3d::{position, prelude::*};
use bevy::{prelude::*, transform};

use crate::kinematic_character_controller::FrameVelocity;

#[derive(Component)]
pub struct CollideAndSlide {
    pub skin_width: f32,
    pub max_iterations: u32, // Added to prevent infinite recursion
    pub epsilon: f32,
}
impl Default for CollideAndSlide {
    fn default() -> Self {
        Self {
            skin_width: 0.1,
            max_iterations: 4,
            epsilon: 0.1,
        }
    }
}

pub fn collide_and_slide(
    mut sliders: Query<(
        Entity,
        &mut FrameVelocity,
        &Collider,
        &Transform,
        &CollideAndSlide,
    )>,
    spatial_query: SpatialQuery,
    mut gizmos: Gizmos,
) {
    for (entity, mut velocity, collider, transform, slider) in
        sliders.iter_mut()
    {
        let final_displacement = recursive_slide(
            entity,
            velocity.0,
            collider,
            transform,
            slider,
            &spatial_query,
            0,
            &mut gizmos,
        );

        velocity.0 = final_displacement;
        gizmos.line(
            transform.translation,
            (transform.translation + velocity.0),
            Color::srgb(1.0, 0.0, 0.0),
        );
    }
}

fn recursive_slide(
    entity: Entity,
    velocity: Vec3, // This is now displacement rather than velocity
    collider: &Collider,
    transform: &Transform,
    slider: &CollideAndSlide,
    spatial_query: &SpatialQuery,
    depth: u32,
    mut gizmos: &mut Gizmos,
) -> Vec3 {
    // Base cases
    if velocity.is_nan() || !velocity.is_finite() {
        return Vec3::ZERO;
    }
    if depth >= slider.max_iterations {
        return velocity;
    }

    let query_filter =
        SpatialQueryFilter::default().with_excluded_entities(vec![entity]);

    if let Ok(cast_dir) = Dir3::new(velocity) {
        if let Some(hit_data) = spatial_query.cast_shape(
            collider,
            transform.translation,
            transform.rotation,
            cast_dir,
            velocity.length() + slider.skin_width,
            false,
            query_filter,
        ) {
            let snap_to_surface: Vec3 = velocity.normalize()
                * (hit_data.time_of_impact - slider.skin_width);

            let mut left_over = velocity - snap_to_surface;
            let normal = hit_data.normal1;
            left_over = left_over - normal * left_over.dot(normal);

            if left_over.length_squared() > slider.epsilon * slider.epsilon {
                let mut new_transform = transform.clone();
                new_transform.translation += snap_to_surface;
                let recursive_motion = recursive_slide(
                    entity,
                    left_over,
                    collider,
                    &new_transform,
                    slider,
                    spatial_query,
                    depth + 1,
                    &mut gizmos,
                );
                return snap_to_surface + recursive_motion;
            }

            return snap_to_surface;
        }
    }

    // No collision found, return original displacement
    velocity
}
