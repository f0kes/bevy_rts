use bevy::prelude::*;

#[derive(Component, Debug, Default)]
pub struct SimpleSteeringComponent {
    pub interest: Vec2,
    pub danger: Vec2,
}

impl SimpleSteeringComponent {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_interest(&mut self, direction: usize, value: f32) {
        // For compatibility with context map API - convert direction to vector
        let angle = (direction as f32) * (2.0 * std::f32::consts::PI / 8.0);
        let direction_vector = Vec2::new(angle.cos(), angle.sin()) * value;
        self.interest += direction_vector;
    }

    pub fn add_danger(&mut self, direction: usize, value: f32) {
        // For compatibility with context map API - convert direction to vector
        let angle = (direction as f32) * (2.0 * std::f32::consts::PI / 8.0);
        let direction_vector = Vec2::new(angle.cos(), angle.sin()) * value;
        self.danger += direction_vector;
    }

    pub fn add_vector_interest(&mut self, vector: Vec2) {
        self.interest += vector;
    }

    pub fn add_vector_danger(&mut self, vector: Vec2) {
        self.danger += vector;
    }

    pub fn get_final_movement(&self) -> Option<Vec2> {
        let result = self.interest - self.danger;
        if result.length_squared() > 0.1 {
            Some(result)
        } else {
            None
        }
    }

    pub fn reset(&mut self) {
        self.interest = Vec2::ZERO;
        self.danger = Vec2::ZERO;
    }
}
