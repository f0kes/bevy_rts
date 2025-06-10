use bevy::prelude::*;
use movement::movement::{Move, MoveVelocity};

use crate::{
    spatial_hashing::spatial_hashmap::SpatialHashmap,
    steering_agent::get_nearby_unit_entities_and_positions,
};

// ======== Components ========

/// Marker component for entities that should behave as boids
#[derive(Component, Reflect)]
pub struct Boid;

/// Controls alignment behavior (tendency to face same direction as neighbors)
#[derive(Component, Reflect)]
pub struct AlignmentBehavior {
    pub weight: f32,
}

/// Controls separation behavior (tendency to avoid crowding)
#[derive(Component, Reflect)]
pub struct SeparationBehavior {
    pub weight: f32,
}

/// Controls target seeking behavior (tendency to move toward targets)
#[derive(Component, Reflect)]
pub struct TargetSeekingBehavior {
    pub weight: f32,
}

/// Maximum distance to consider neighbors when flocking
#[derive(Component, Reflect)]
pub struct NeighborDistance(pub f32);

/// Maximum distance to detect enemy targets
#[derive(Component, Reflect)]
pub struct AttackRadius(pub f32);

/// Currently targeted enemy unit (if any)
#[derive(Component, Reflect)]
pub struct TargetedUnit(pub Option<Entity>);

/// Controls how quickly a boid changes direction
/// Values closer to 1.0 mean slower turning
#[derive(Component, Reflect)]
pub struct DirectionSmoothingFactor(pub f32);

/// Team component for boids
#[derive(Component, Reflect, PartialEq, Eq)]
pub struct Team(pub u32);

// ======== Systems ========

/// Resets the Move component at the start of each frame
pub fn reset_move_system(mut query: Query<&mut Move, With<Boid>>) {
    for mut move_component in query.iter_mut() {
        move_component.0 = Vec3::ZERO;
    }
}

/// Finds the closest enemy unit within attack radius
pub fn targeting_system(
    mut query: Query<
        (Entity, &Transform, &AttackRadius, &Team, &mut TargetedUnit),
        With<Boid>,
    >,
    spatial_hashmap: Res<SpatialHashmap>,
    team_query: Query<&Team>,
) {
    // Collect boid positions for spatial query
    let entities_and_positions: Vec<(Entity, Vec3)> = query
        .iter()
        .map(|(entity, transform, _, _, _)| (entity, transform.translation))
        .collect();

    for (entity, transform, attack_radius, team, mut targeted_unit) in
        query.iter_mut()
    {
        let boid_pos = transform.translation;
        let attack_radius_squared = attack_radius.0 * attack_radius.0;
        let mut min_sqr_dist = attack_radius_squared;
        targeted_unit.0 = None;

        // Get nearby entities
        let nearby_entities = get_nearby_unit_entities_and_positions(
            spatial_hashmap.as_ref(),
            entity,
            boid_pos,
            attack_radius.0,
            &entities_and_positions,
            50,
        );

        // Find closest enemy unit within attack radius
        for (other_entity, other_pos) in nearby_entities {
            if other_entity == entity {
                continue;
            }

            if let Ok(other_team) = team_query.get(other_entity) {
                if other_team != team {
                    let to_other = other_pos - boid_pos;
                    let sqr_dist = to_other.length_squared();

                    if sqr_dist < min_sqr_dist {
                        targeted_unit.0 = Some(other_entity);
                        min_sqr_dist = sqr_dist;
                    }
                }
            }
        }
    }
}

/// Implements separation behavior (avoiding getting too close to neighbors)
pub fn separation_system(
    mut query: Query<
        (
            Entity,
            &Transform,
            &NeighborDistance,
            &SeparationBehavior,
            &mut Move,
        ),
        With<Boid>,
    >,
    spatial_hashmap: Res<SpatialHashmap>,
) {
    // Collect boid positions for spatial query
    let entities_and_positions: Vec<(Entity, Vec3)> = query
        .iter()
        .map(|(entity, transform, _, _, _)| (entity, transform.translation))
        .collect();

    for (
        entity,
        transform,
        neighbor_distance,
        separation_behavior,
        mut move_component,
    ) in query.iter_mut()
    {
        
        let boid_pos = transform.translation;
        let mut separation = Vec3::ZERO;
        let mut separation_desire = 0.0;

        // Get nearby entities
        let nearby_entities = get_nearby_unit_entities_and_positions(
            spatial_hashmap.as_ref(),
            entity,
            boid_pos,
            neighbor_distance.0,
            &entities_and_positions,
            50,
        );

        // Calculate separation vector
        for (other_entity, other_pos) in nearby_entities {
            if other_entity == entity {
                continue;
            }

            let diff = boid_pos - other_pos;
            let diff_len = diff.length();

            if diff_len > 0.001 {
                let scaler =
                    (1.0 - diff_len / neighbor_distance.0).clamp(0.0, 1.0);
                separation += diff * (scaler / diff_len);
                separation_desire += scaler;
            }
        }

        // Only update if we have some separation desire
        if separation_desire > 0.0 {
            move_component.0 += separation * separation_behavior.weight;
        }
    }
}

/// Implements alignment behavior (tendency to face same direction as neighbors)
pub fn alignment_system(
    mut query: Query<
        (
            Entity,
            &Transform,
            &NeighborDistance,
            &Team,
            &AlignmentBehavior,
            &mut Move,
        ),
        With<Boid>,
    >,
    query_transform: Query<&Transform>,
    spatial_hashmap: Res<SpatialHashmap>,
    team_query: Query<&Team>,
) {
    // Collect boid positions for spatial query
    let entities_and_positions: Vec<(Entity, Vec3)> = query
        .iter()
        .map(|(entity, transform, _, _, _, _)| (entity, transform.translation))
        .collect();

    for (
        entity,
        transform,
        neighbor_distance,
        team,
        alignment_behavior,
        mut move_component,
    ) in query.iter_mut()
    {
        let boid_pos = transform.translation;
        let mut alignment = Vec3::ZERO;
        let mut nearby_count = 0;

        // Get nearby entities
        let nearby_entities = get_nearby_unit_entities_and_positions(
            spatial_hashmap.as_ref(),
            entity,
            boid_pos,
            neighbor_distance.0,
            &entities_and_positions,
            50,
        );

        // Calculate alignment vector (average direction)
        for (other_entity, _) in nearby_entities {
            if other_entity == entity {
                continue;
            }

            // Only align with same team
            if let Ok(other_team) = team_query.get(other_entity) {
                if other_team == team {
                    // Get the other entity's transform to use its forward direction
                    if let Ok(other_transform) =
                        query_transform.get(other_entity)
                    {
                        alignment += other_transform.forward().normalize();
                        nearby_count += 1;
                    }
                }
            }
        }

        // Apply alignment if we have neighbors
        if nearby_count > 0 {
            alignment = (alignment / nearby_count as f32).normalize_or_zero();
            move_component.0 += alignment * alignment_behavior.weight;
        }
    }
}

/// Implements target seeking behavior (moving toward targeted units)
pub fn target_seeking_system(
    mut query: Query<
        (
            &Transform,
            &TargetSeekingBehavior,
            &mut Move,
            Option<&TargetedUnit>,
        ),
        With<Boid>,
    >,
    target_transform_query: Query<&Transform>,
) {
    for (transform, target_behavior, mut move_component, targeted_unit) in
        query.iter_mut()
    {
        let boid_pos = transform.translation;

        // If we have a targeted unit, move toward it
        if let Some(targeted_unit) = targeted_unit {
            if let Some(target_entity) = targeted_unit.0 {
                if let Ok(target_transform) =
                    target_transform_query.get(target_entity)
                {
                    let to_target =
                        (target_transform.translation - boid_pos).normalize();
                    move_component.0 += to_target * target_behavior.weight;
                }
            }
        }
    }
}

/// Adjusts velocity based on facing angle relative to target
pub fn velocity_adjustment_system(
    mut query: Query<
        (&Transform, &TargetedUnit, &mut MoveVelocity),
        With<Boid>,
    >,
    target_transform_query: Query<&Transform>,
) {
    for (transform, targeted_unit, mut velocity) in query.iter_mut() {
        // If we have a targeted unit, adjust velocity based on angle to target
        if let Some(target_entity) = targeted_unit.0 {
            if let Ok(target_transform) =
                target_transform_query.get(target_entity)
            {
                let boid_dir = if velocity.0.length_squared() > 0.1 {
                    velocity.0.normalize()
                } else {
                    transform.forward().normalize()
                };

                let from_target = (transform.translation
                    - target_transform.translation)
                    .normalize();
                let dot_p = boid_dir.dot(from_target).max(0.1);

                // Scale velocity by dot product (slow down when not facing away from target)
                velocity.0 = velocity.0 * dot_p;
                println!("Dot product: {}", dot_p);
            }
        }
    }
}

/// Smoothly changes direction (interpolates between current and target direction)
/* pub fn direction_smoothing_system(
    mut query: Query<
        (&mut Transform, &Move, &DirectionSmoothingFactor),
        With<Boid>,
    >,
) {
    for (mut transform, move_component, smoothing) in query.iter_mut() {
        if move_component.0.length_squared() > 0.01 {
            let current_dir = transform.forward();
            let target_dir = move_component.0.normalize();

            // Smooth between current and target direction
            let new_dir = current_dir.lerp(target_dir, 1.0 - smoothing.0);

            // Update transform
            if new_dir.length_squared() > 0.01 {
                transform.look_to(new_dir, Vec3::Y);
            }
        }
    }
} */

// ======== Plugin ========

/// Plugin that adds boid flocking behavior systems
pub struct BoidPlugin;

impl Plugin for BoidPlugin {
    fn build(&self, app: &mut App) {
        app
            // Register components
            .register_type::<Boid>()
            .register_type::<AlignmentBehavior>()
            .register_type::<SeparationBehavior>()
            .register_type::<TargetSeekingBehavior>()
            .register_type::<NeighborDistance>()
            .register_type::<AttackRadius>()
            .register_type::<TargetedUnit>()
            .register_type::<DirectionSmoothingFactor>()
            .register_type::<Team>()
            // Add systems in correct order
            .add_systems(
                Update,
                (
                    reset_move_system,
                    targeting_system,
                    (
                        separation_system,
                        alignment_system,
                        target_seeking_system,
                    ),
                    velocity_adjustment_system,
                    //direction_smoothing_system,
                )
                    .chain(),
            );
    }
}

// ======== Spawn Helper Functions ========

/// Helper function to add boid behavior to an entity
#[derive(Bundle)]
pub struct BoidBundle {
    pub marker: Boid,
    pub alignment: AlignmentBehavior,
    pub separation: SeparationBehavior,
    pub target_seeking: TargetSeekingBehavior,
    pub neighbor_distance: NeighborDistance,
    pub attack_radius: AttackRadius,
    pub targeted_unit: TargetedUnit,
    pub direction_smoothing: DirectionSmoothingFactor,
}

impl BoidBundle {
    /// Creates a new BoidBundle with custom parameters
    pub fn new(
        alignment_weight: f32,
        separation_weight: f32,
        target_seeking_weight: f32,
        neighbor_distance: f32,
        attack_radius: f32,
        smoothing_factor: f32,
    ) -> Self {
        Self {
            marker: Boid,
            alignment: AlignmentBehavior {
                weight: alignment_weight,
            },
            separation: SeparationBehavior {
                weight: separation_weight,
            },
            target_seeking: TargetSeekingBehavior {
                weight: target_seeking_weight,
            },
            neighbor_distance: NeighborDistance(neighbor_distance),
            attack_radius: AttackRadius(attack_radius),
            targeted_unit: TargetedUnit(None),
            direction_smoothing: DirectionSmoothingFactor(smoothing_factor),
        }
    }
    pub fn aggressive() -> Self {
        Self::new(
            0.8, // lower alignment - less concerned with group direction
            1.2, // moderate separation - maintains some distance but willing to engage
            2.0, // high target seeking - eagerly pursues targets
            8.0, // shorter neighbor distance - tighter formation
            7.0, // longer attack radius - more aggressive detection range
            0.7, // quicker turning - more responsive to target changes
        )
    }
}

impl Default for BoidBundle {
    fn default() -> Self {
        Self::new(
            1.0,  // alignment_weight
            1.5,  // separation_weight
            1.0,  // target_seeking_weight
            10.0, // neighbor_distance
            5.0,  // attack_radius
            0.9,  // smoothing_factor
        )
    }
}
