use bevy::prelude::*;

use crate::in_world::{on_enter_world, on_exit_world};

pub struct MetaComponentsPlugin;
impl Plugin for MetaComponentsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (on_enter_world, on_exit_world));
    }
}
