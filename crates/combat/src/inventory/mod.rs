pub mod remove_from_world;
pub mod systems;
pub mod plugin;

use crate::units::unit::{get_unit_data, UnitName};
use bevy::prelude::*;

#[derive(Component)]
pub struct Inventory {
    slots: Vec<ItemContainer>,
    size: u32,
}
impl Default for Inventory {
    fn default() -> Self {
        Self {
            slots: vec![ItemContainer::Empty; 10],
            size: 10,
        }
    }
}
#[derive(Debug)]
pub enum AddResult {
    Success,
    Partial { leftovers: u32 },
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
            size,
        }
    }
    pub fn try_put(&mut self, item: Item, count: u32) -> AddResult {
        let mut remaining = count;

        // First pass: fill existing stacks
        for slot in &mut self.slots {
            if let ItemContainer::Occupied {
                item_type: existing,
                count: current,
                max_count,
                
            } = slot
            {
                if *existing == item {
                    let space = *max_count - *current;
                    let add_amount = remaining.min(space);
                    *current += add_amount;
                    remaining -= add_amount;

                    if remaining == 0 {
                        return AddResult::Success;
                    }
                }
            }
        }

        // Second pass: fill empty slots
        for slot in &mut self.slots {
            if let ItemContainer::Empty = slot {
                let max_count = item.get_stack_size();
                let add_amount = remaining.min(max_count);
                *slot = ItemContainer::Occupied {
                    item_type: item,
                    count: add_amount,
                    max_count,
                };
                remaining -= add_amount;

                if remaining == 0 {
                    return AddResult::Success;
                }
            }
        }

        // If we still have remaining items, return appropriate result
        if remaining < count {
            AddResult::Partial {
                leftovers: remaining,
            }
        } else {
            AddResult::Failed {
                reason: AddError::InventoryFull,
            }
        }
    }
    pub fn try_take(&mut self, item: Item, count: u32) -> u32 {
        let mut remaining = count;

        for slot in &mut self.slots {
            if let ItemContainer::Occupied {
                item_type: existing,
                count: current,
                max_count: _,
                
            } = slot
            {
                if *existing == item {
                    let take_amount = remaining.min(*current);
                    *current -= take_amount;
                    remaining -= take_amount;

                    if *current == 0 {
                        *slot = ItemContainer::Empty;
                    }

                    if remaining == 0 {
                        return count;
                    }
                }
            }
        }

        count - remaining
    }
}
#[derive(Debug, Clone)]
pub enum ItemContainer {
    Empty,
    Occupied {
        item_type: Item,
        count: u32,
        max_count: u32,
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
