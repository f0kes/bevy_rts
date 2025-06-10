use bevy::prelude::*;

/// Common component that all steering behaviors will modify
#[derive(Component, Default)]
pub struct SteeringBehavior {
    pub interests: Vec<(Vec2, f32)>, // Direction and strength
    pub dangers: Vec<(Vec2, f32)>,   // Direction and strength
}

impl SteeringBehavior {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_interest(&mut self, direction: usize, value: f32) {
        let angle = (direction as f32) * (2.0 * std::f32::consts::PI / 8.0);
        let vector = Vec2::new(angle.cos(), angle.sin());
        self.interests.push((vector, value));
    }

    pub fn add_danger(&mut self, direction: usize, value: f32) {
        let angle = (direction as f32) * (2.0 * std::f32::consts::PI / 8.0);
        let vector = Vec2::new(angle.cos(), angle.sin());
        self.dangers.push((vector, value));
    }

    pub fn add_vector_interest(&mut self, direction: Vec2) {
        if direction.length_squared() > 0.0 {
            self.interests.push((direction.normalize(), direction.length()));
        }
    }

    pub fn add_vector_danger(&mut self, direction: Vec2) {
        if direction.length_squared() > 0.0 {
            self.dangers.push((direction.normalize(), direction.length()));
        }
    }

    pub fn reset(&mut self) {
        self.interests.clear();
        self.dangers.clear();
    }
}