use bevy::prelude::*;
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum InputAction {
    UseItem,
    Collect,
    MoveForward,
    MoveBack,
    MoveLeft,
    MoveRight,
}
