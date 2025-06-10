use bevy::prelude::*;

use crate::GameState;

use super::on_hit::spawn_on_hit_particles;

pub struct ParticlesPlugin;
impl Plugin for ParticlesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            spawn_on_hit_particles.run_if(in_state(GameState::Loaded)),
        );
    }
}
