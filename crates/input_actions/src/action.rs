use bevy::prelude::*;
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Action {
    Spawn,
    Collect,
    MoveForward,
    MoveBack,
    MoveLeft,
    MoveRight,
}
