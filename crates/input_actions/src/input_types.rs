use std::hash::Hash;

use bevy::prelude::*;

#[derive(Component, PartialEq, Eq, Hash, Clone, Copy)]
pub enum InputType {
    Key(KeyCode),
    MouseButton(MouseButton),
}
impl From<KeyCode> for InputType {
    fn from(key: KeyCode) -> Self {
        InputType::Key(key)
    }
}
impl From<MouseButton> for InputType {
    fn from(button: MouseButton) -> Self {
        InputType::MouseButton(button)
    }
}
pub fn test() {}
