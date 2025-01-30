use bevy::prelude::*;
use strum_macros::EnumDiscriminants;

use crate::spells::vacuum::VacuumSpell;

#[derive(Component)]
pub struct ActionData {
    pub actor: Entity,
    pub action_type: ActionType,
}
#[derive(Component)]
pub enum ActionType {
    Continuous,
    Instant,
    Channeled,
}
#[derive(Component, EnumDiscriminants, Clone, Copy,)]
#[strum_discriminants(derive(Hash))]
pub enum Action {
    None,
    VacuumSpell(VacuumSpell),
}
impl PartialEq for Action {
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}
impl Eq for Action {}

#[derive(Bundle)]
pub struct ActionBundle {
    pub action: Action,
    pub data: ActionData,
}
impl ActionBundle {
    pub fn new(action: Action, actor: Entity) -> Self {
        Self {
            action,
            data: ActionData {
                actor,
                action_type: ActionType::Instant,
            },
        }
    }

    pub fn vacuum_spell(spell: VacuumSpell, actor: Entity) -> Self {
        Self::new(Action::VacuumSpell(spell), actor)
            .with_type(ActionType::Continuous)
    }
    pub fn with_type(mut self, action_type: ActionType) -> Self {
        self.data.action_type = action_type;
        self
    }
    pub fn from_action(action: Action, actor: Entity) -> Self {
        match action {
            Action::None => Self::new(action, actor),
            Action::VacuumSpell(spell) => Self::vacuum_spell(spell, actor),
        }
    }
}
pub fn add_spell_component(
    mut commands: Commands,
    query: Query<(Entity, &Action), Added<Action>>,
) {
    for (entity, action) in query.iter() {
        if let Action::VacuumSpell(spell) = action {
            commands.entity(entity).insert(*spell);
        }
    }
}
