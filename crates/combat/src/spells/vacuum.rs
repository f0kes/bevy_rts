use bevy::prelude::*;

use crate::units::unit::Unit;

#[derive(Component)]
pub struct VacuumSpell {
    pub range: f32,
    pub width: f32,
    pub pull_force: f32,
}

pub fn cast_vacuum(
    mut commands: Commands,
    spell: Query<(Entity, &VacuumSpell)>,
    targets: Query<(Entity, &Transform, &Unit)>,
) {
    for (spell_entity, spell) in spell.iter() {}
}
