use bevy::prelude::*;
use misc::disabled::ToggleCommands;

#[derive(Component)]
pub struct InWorld;
pub fn on_enter_world(
    mut commands: Commands,
    query: Query<Entity, Added<InWorld>>,
) {
    for entity in query.iter() {
        commands.entity(entity).enable::<Handle<Scene>>();
        commands.entity(entity).enable::<Transform>();
        commands.entity(entity).enable::<GlobalTransform>();
    }
}
pub fn on_exit_world(
    mut commands: Commands,
    mut removed: RemovedComponents<InWorld>,
) {
    for entity in removed.read() {
        if let Some(mut entity_commands) = commands.get_entity(entity) {
            entity_commands
                .disable::<Handle<Scene>>()
                .disable::<Transform>()
                .disable::<GlobalTransform>();
        }
    }
}
