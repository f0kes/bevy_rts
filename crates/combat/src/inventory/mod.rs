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
    pub fn add(&mut self, item: Item, count: u32) -> AddResult {
        let mut remaining = count;

        // First pass: fill existing stacks
        for slot in &mut self.slots {
            if let ItemContainer::Item {
                item: existing,
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
                *slot = ItemContainer::Item {
                    item,
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
}
#[derive(Debug, Clone, Copy)]
pub enum ItemContainer {
    Empty,
    Item {
        item: Item,
        count: u32,
        max_count: u32,
    },
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Item {
    Unit { name: UnitName, entity: Entity },
}
impl Item {
    pub fn get_stack_size(&self) -> u32 {
        match self {
            Item::Unit { name, entity: _ } => get_unit_data(*name).stack_size,
        }
    }
}
