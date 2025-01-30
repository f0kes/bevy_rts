pub mod plugin;
pub mod remove_from_world;
pub mod systems;

use crate::units::unit::{get_unit_data, UnitName};
use bevy::prelude::*;

#[derive(Component)]
pub struct Inventory {
    slots: Vec<ItemContainer>,
}

#[derive(Component)]
pub struct ChosenSlot {
    pub index: u32,
}

impl Default for Inventory {
    fn default() -> Self {
        Self {
            slots: vec![ItemContainer::Empty; 10],
        }
    }
}
#[derive(Debug)]
pub enum AddResult {
    Success,
    Failed { reason: AddError },
}

#[derive(Debug)]
pub enum AddError {
    InventoryFull,
    InvalidItem,
}
impl Inventory {
    pub fn new(size: u32) -> Self {
        Self {
            slots: vec![ItemContainer::Empty; size as usize],
        }
    }
    pub fn try_put(&mut self, item: Item, entity: Entity) -> AddResult {
        let stack_size = item.get_stack_size();
        for (_index, slot) in self.slots.iter_mut().enumerate() {
            match slot {
                ItemContainer::Empty => {
                    *slot = ItemContainer::Occupied {
                        item_type: item,
                        count: 1,
                        max_count: stack_size,
                        held_entities: vec![entity],
                    };
                    return AddResult::Success;
                }
                ItemContainer::Occupied {
                    item_type,
                    count,
                    max_count,
                    held_entities,
                } => {
                    if item == *item_type {
                        if *count < *max_count {
                            *count += 1;
                            held_entities.push(entity);
                            return AddResult::Success;
                        }
                    }
                }
            }
        }
        AddResult::Failed {
            reason: AddError::InventoryFull,
        }
    }
    pub fn try_take_with_item(&mut self, item: Item) -> Option<Entity> {
        for (_index, slot) in self.slots.iter_mut().enumerate() {
            match slot {
                ItemContainer::Empty => {}
                ItemContainer::Occupied {
                    item_type,
                    count,
                    max_count: _,
                    held_entities,
                } => {
                    if item == *item_type {
                        if *count > 0 {
                            *count -= 1;
                            return held_entities.pop();
                        }
                    }
                }
            }
        }
        None
    }
    pub fn try_take_with_slot(
        &mut self,
        slot_index: u32,
    ) -> Option<(Item, Entity)> {
        if let Some(slot) = self.slots.get_mut(slot_index as usize) {
            match slot {
                ItemContainer::Empty => None,
                ItemContainer::Occupied {
                    item_type,
                    count,
                    max_count: _,
                    held_entities,
                } => {
                    if *count > 0 {
                        *count -= 1;
                        if let Some(next_entity) = held_entities.pop() {
                            return Some((*item_type, next_entity));
                        }
                    }
                    None
                }
            }
        } else {
            None
        }
    }
    pub fn is_full(&self) -> bool {
        self.slots.iter().all(|slot| match slot {
            ItemContainer::Empty => false,
            ItemContainer::Occupied {
                count, max_count, ..
            } => *count >= *max_count,
        })
    }
    pub fn is_empty(&self) -> bool {
        self.slots.iter().all(|slot| match slot {
            ItemContainer::Empty => true,
            ItemContainer::Occupied { count, .. } => *count == 0,
        })
    }
}
#[derive(Debug, Clone)]
pub enum ItemContainer {
    Empty,
    Occupied {
        item_type: Item,
        count: u32,
        max_count: u32,
        held_entities: Vec<Entity>,
    },
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Item {
    Unit { name: UnitName },
}
impl Item {
    pub fn get_stack_size(&self) -> u32 {
        match self {
            Item::Unit { name } => get_unit_data(*name).stack_size,
        }
    }
}
