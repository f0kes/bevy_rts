use bevy::prelude::*;

use crate::in_world::InWorldPlugin;

pub struct MetaComponentsPlugin;
impl Plugin for MetaComponentsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InWorldPlugin);
    }
}
