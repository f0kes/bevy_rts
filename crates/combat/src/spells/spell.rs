use bevy::prelude::*;

use crate::spells::vacuum::VacuumSpell;

#[derive(Component)]
pub struct ActionData {
    pub actor: Entity,
}
#[derive(Component)]
pub enum Action {
    None,
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
