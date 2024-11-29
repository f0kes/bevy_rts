use bevy::prelude::*;
use movement::movement::{Move, MoveInput};

use crate::plugin::SteeringAgent;

const DIRECTION_COUNT: usize = 8;
#[derive(Component, Debug)]
pub struct ContextMap {
    interest: [f32; DIRECTION_COUNT],
    danger: [f32; DIRECTION_COUNT],
    direction_vectors: [Vec2; DIRECTION_COUNT], // Cache for direction vectors
}

impl Default for ContextMap {
    fn default() -> Self {
        let mut direction_vectors = [Vec2::ZERO; DIRECTION_COUNT];
        for i in 0..DIRECTION_COUNT {
            let angle = (i as f32)
                * (2.0 * std::f32::consts::PI / DIRECTION_COUNT as f32);
            direction_vectors[i] = Vec2::new(angle.cos(), angle.sin());
        }

        Self {
            interest: [0.0; DIRECTION_COUNT],
            danger: [0.0; DIRECTION_COUNT],
            direction_vectors,
        }
    }
}

impl ContextMap {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_interest(&mut self, direction: usize, value: f32) {
        if direction < DIRECTION_COUNT && self.interest[direction] < value {
            self.interest[direction] = value;
        }
    }

    pub fn add_danger(&mut self, direction: usize, value: f32) {
        if direction < DIRECTION_COUNT && self.danger[direction] < value {
            self.danger[direction] = value;
        }
    }

    pub fn add_vector_interest(&mut self, vector: Vec2) {
        let magnitude = vector.length();
        if magnitude == 0.0 {
            return;
        }
        let normalized = vector / magnitude;

        for (i, &direction_vector) in self.direction_vectors.iter().enumerate()
        {
            let dot = normalized.dot(direction_vector);
            let val = magnitude * dot;
            // Only add positive dot products to avoid negative desires
            if dot > 0.0 && self.interest[i] < val {
                self.interest[i] = val;
            }
        }
    }

    pub fn add_vector_danger(&mut self, vector: Vec2) {
        let magnitude = vector.length();
        if magnitude == 0.0 {
            return;
        }
        let normalized = vector / magnitude;

        for (i, &direction_vector) in self.direction_vectors.iter().enumerate()
        {
            let dot = normalized.dot(direction_vector);
            let val = magnitude * dot;
            // Only add positive dot products to avoid negative avoidance
            if dot > 0.0 && self.danger[i] < val {
                self.danger[i] = val;
            }
        }
    }

    pub fn get_final_movement(&self) -> Option<Vec2> {
        let mut max_direction = 0;
        let mut max_value = f32::NEG_INFINITY;
        let mut value_set = false;

        for direction in 0..DIRECTION_COUNT {
            let value = self.interest[direction] - self.danger[direction];
            if value > max_value {
                max_value = value;
                max_direction = direction;
                value_set = true;
            }
        }
        if value_set {
            Some(self.direction_vectors[max_direction] * max_value)
        } else {
            None
        }
    }

    pub fn get_direction_vector(&self, direction: usize) -> Vec2 {
        self.direction_vectors[direction]
    }

    pub fn reset(&mut self) {
        self.interest.fill(0.0);
        self.danger.fill(0.0);
    }
    pub fn get_direction_count(&self) -> usize {
        DIRECTION_COUNT
    }
}

pub fn move_based_on_context_map(
    mut commands: Commands,
    mut query: Query<(Entity, &mut ContextMap)>,
) {
    for (entity, mut context_map) in query.iter_mut() {
        let final_movement = context_map.get_final_movement();
        if let Some(final_movement) = final_movement {
            let new_move = Vec3::new(final_movement.x, 0.0, final_movement.y);
            commands.entity(entity).insert(Move(new_move));
        } else {
            commands.entity(entity).insert(Move(Vec3::ZERO));
        }

        context_map.reset();
    }
}
pub fn add_context_map_to_steering_agents(
    mut commands: Commands,
    query: Query<Entity, (With<SteeringAgent>, Without<ContextMap>)>,
) {
    for entity in query.iter() {
        commands.entity(entity).insert(ContextMap::new());
    }
}
