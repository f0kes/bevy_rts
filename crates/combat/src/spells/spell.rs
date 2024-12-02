use bevy::prelude::*;

use crate::spells::vacuum::VacuumSpell;

#[derive(Component)]
pub struct ActionData {
    pub actor: Entity,
}
#[derive(Component)]
pub enum Action {
    VacuumSpell(VacuumSpell),
}

#[derive(Bundle)]
pub struct ActionBundle {
    pub action: Action,
    pub data: ActionData,
}
impl ActionBundle {
    pub fn new(action: Action, actor: Entity) -> Self {
        Self {
            action,
            data: ActionData { actor },
        }
    }

    pub fn vacuum_spell(spell: VacuumSpell, actor: Entity) -> Self {
        Self::new(Action::VacuumSpell(spell), actor)
    }
}
