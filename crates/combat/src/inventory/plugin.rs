use super::systems::handle_inventory_additions;
use bevy::prelude::*;

pub struct InventoryPlugin;
impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_inventory_additions);
    }
}
