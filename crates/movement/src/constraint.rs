
use bevy::prelude::*;
pub trait MovementConstraint {
    fn is_move_allowed(&self, from: Vec3, to: Vec3) -> bool;
}