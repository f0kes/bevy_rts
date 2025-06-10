use bevy::ecs::{component::Component, entity::Entity};

#[derive(Component)]
pub struct OwnedBy {
    pub entity: Entity,
}
