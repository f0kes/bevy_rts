use bevy::prelude::*;

use crate::on_hit::spawn_on_hit_particles;
pub struct ParticlesPlugin;
impl Plugin for ParticlesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_on_hit_particles);
    }
}
