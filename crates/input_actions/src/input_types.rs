use std::hash::Hash;

use bevy::prelude::*;

#[derive(Component, PartialEq, Eq, Hash, Clone, Copy)]
pub enum InputType {
    Key(KeyCode),
}
pub fn test() {}
