use bevy::prelude::*;

// Generic component that will spawn another entity with component T
#[derive(Component)]
struct SpawnLinked<T: Bundle + Clone> {
    spawned_entity: Option<Entity>,
    data_to_spawn: T,
}

// Implementation for the generic component
impl<T: Bundle + Clone> SpawnLinked<T> {
    fn new(data: T) -> Self {
        Self {
            spawned_entity: None,
            data_to_spawn: data,
        }
    }
}

// System to handle spawning linked entities
fn spawn_linked_system<T: Bundle + Clone>(
    mut commands: Commands,
    mut query: Query<(Entity, &mut SpawnLinked<T>)>,
) {
    for (entity, mut spawn_linked) in query.iter_mut() {
        if spawn_linked.spawned_entity.is_none() {
            let spawned =
                commands.spawn(spawn_linked.data_to_spawn.clone()).id();
            spawn_linked.spawned_entity = Some(spawned);
        }
    }
}

// System to handle cleanup when the SpawnLinked component is removed
fn cleanup_linked_system<T: Bundle + Clone>(
    mut commands: Commands,
    mut removed_query: RemovedComponents<SpawnLinked<T>>,
    spawn_linked_query: Query<&SpawnLinked<T>>,
) {
    for entity in removed_query.read() {
        if let Ok(spawn_linked) = spawn_linked_query.get(entity) {
            if let Some(spawned_entity) = spawn_linked.spawned_entity {
                commands.entity(spawned_entity).despawn();
            }
        }
    }
}

pub trait SpawnLinkedAppExt {
    fn add_spawn_linked<T: Bundle + Clone>(&mut self) -> &mut Self;
}

impl SpawnLinkedAppExt for App {
    fn add_spawn_linked<T: Bundle + Clone>(&mut self) -> &mut Self {
        self.add_systems(
            Update,
            (spawn_linked_system::<T>, cleanup_linked_system::<T>),
        )
    }
}
