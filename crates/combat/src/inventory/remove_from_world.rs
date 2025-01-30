use bevy::prelude::*;

use super::Inventory;

#[derive(Component)]
pub struct InInventory {
    pub holder: Entity,
}

pub fn remove_entities_in_inventories_from_world(
    mut commands: Commands,
    invenories: Query<(Entity, &Inventory)>,
    in_inventory: Query<&InInventory>,
) {
    for (holder, inventory) in invenories.iter() {
        for slot in &inventory.slots {}
    }
}
