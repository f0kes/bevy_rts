use bevy::prelude::*;
use meta_components::in_world::InWorld;

use super::{AddResult, Inventory, Item};
#[derive(Component)]
pub struct AddToInventory {
    pub target_inventory: Entity,
    pub item: Item,
    pub count: u32,
}

#[derive(Component)]
pub struct InInventory {
    pub target_inventory: Entity,
    pub item: Item,
    pub count: u32,
}
#[derive(Component)]
pub struct RemoveFromInventory;

pub fn handle_inventory_additions(
    mut commands: Commands,
    mut inventories: Query<&mut Inventory>,
    to_add: Query<(Entity, &AddToInventory), Without<InInventory>>,
) {
    for (entity, add_to_inventory) in to_add.iter() {
        if let Ok(mut inventory) =
            inventories.get_mut(add_to_inventory.target_inventory)
        {
            println!("inv found");
            match inventory
                .try_put(add_to_inventory.item, add_to_inventory.count)
            {
                AddResult::Success => {
                    // Remove InWorld and AddToInventory components
                    if let Some(mut entity_commands) =
                        commands.get_entity(entity)
                    {
                        println!("Adding item to inventory: Success");
                        entity_commands
                            .remove::<InWorld>()
                            .remove::<AddToInventory>()
                            .insert(InInventory {
                                target_inventory: add_to_inventory
                                    .target_inventory,
                                item: add_to_inventory.item,
                                count: add_to_inventory.count,
                            });
                    }
                }
                AddResult::Partial { leftovers } => {
                    // If we managed to add at least some items
                    if leftovers < add_to_inventory.count {
                        if let Some(mut entity_commands) =
                            commands.get_entity(entity)
                        {
                            println!("Adding item to inventory: Partial");
                            entity_commands
                                .remove::<InWorld>()
                                .remove::<AddToInventory>()
                                .insert(InInventory {
                                    target_inventory: add_to_inventory
                                        .target_inventory,
                                    item: add_to_inventory.item,
                                    count: add_to_inventory.count,
                                });
                        }
                    }
                }
                AddResult::Failed { reason } => {
                    // Just remove the AddToInventory component on failure
                    if let Some(mut entity_commands) =
                        commands.get_entity(entity)
                    {
                        println!(
                            "Adding item to inventory: Failed, reason {:?}",
                            reason
                        );
                        entity_commands.remove::<AddToInventory>();
                    }
                }
            }
        }
    }
}

pub fn handle_inventory_removals(
    mut commands: Commands,
    mut inventories: Query<&mut Inventory>,
    to_remove: Query<(Entity, &RemoveFromInventory), With<InInventory>>,
) {
    for (entity, _) in to_remove.iter() {
        if let Some(mut entity_commands) = commands.get_entity(entity) {
            println!("Removing item from inventory");
            entity_commands
                .remove::<InInventory>()
                .insert(InWorld)
                .insert(RemoveFromInventory);
        }
    }
}
