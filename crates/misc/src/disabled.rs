use bevy::{ecs::system::EntityCommands, prelude::*};
use std::marker::PhantomData;

#[derive(Debug, PartialEq, Eq, Component)]
pub struct Disabled<T> {
    pub data: T,
}

#[derive(Debug, Component)]
pub struct Enable<T: Component>(pub PhantomData<T>);

#[derive(Debug, Component)]
pub struct Disable<T: Component>(pub PhantomData<T>);

#[derive(Debug, Component)]
pub struct Toggle<T: Component>(pub PhantomData<T>);

impl<T> From<T> for Disabled<T> {
    fn from(data: T) -> Self {
        Disabled { data }
    }
}

// System to enable components
pub fn enable_components<T: Component + Clone>(
    mut commands: Commands,
    query: Query<(Entity, Option<&Disabled<T>>, &Enable<T>)>,
) {
    for (entity, disabled, _) in query.iter() {
        let mut entity_commands = commands.entity(entity);
        // Always remove Enable marker
        entity_commands.remove::<Enable<T>>();
      

        // Only process if we have a disabled component
        if let Some(disabled) = disabled {
            entity_commands
                .insert(disabled.data.clone())
                .remove::<Disabled<T>>();
          
        }
    }
}

// System to disable components
pub fn disable_components<T: Component + Clone>(
    mut commands: Commands,
    query: Query<(Entity, Option<&T>, &Disable<T>), With<T>>,
) {
    for (entity, component, _) in query.iter() {
        let mut entity_commands = commands.entity(entity);
        // Always remove Disable marker
        entity_commands.remove::<Disable<T>>();
        // Only process if we have the component
        if let Some(component) = component {
            entity_commands
                .insert(Disabled::from(component.clone()))
                .remove::<T>();
        }
    }
}
pub fn remove_disabled_on_added<T: Component>(
    mut commands: Commands,
    query: Query<Entity, (Added<T>, With<Disabled<T>>)>,
) {
    for entity in query.iter() {
        commands.entity(entity).remove::<Disabled<T>>();
    }
}

// System to toggle components
pub fn toggle_components<T: Component + Clone>(
    mut commands: Commands,
    query: Query<(Entity, Option<&T>, Option<&Disabled<T>>, &Toggle<T>)>,
) {
    for (entity, component, disabled_component, _) in query.iter() {
        match (component, disabled_component) {
            (Some(component), None) => {
                commands
                    .entity(entity)
                    .remove::<T>()
                    .insert(Disabled::from(component.clone()));
            }
            (None, Some(disabled)) => {
                commands
                    .entity(entity)
                    .insert(disabled.data.clone())
                    .remove::<Disabled<T>>();
            }
            _ => {} // Should not happen
        }
        commands.entity(entity).remove::<Toggle<T>>();
    }
}
pub fn overwrite_disabled<T: Component>(
    mut commands: Commands,
    query: Query<(Entity, &T), With<Disabled<T>>>,
) {
    for (entity, _) in query.iter() {
        commands.entity(entity).remove::<Disabled<T>>();
    }
}

// Generic plugin for component toggling
pub struct ComponentTogglePlugin<T: Component + Clone>(PhantomData<T>);

impl<T: Component + Clone> Default for ComponentTogglePlugin<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<T: Component + Clone> Plugin for ComponentTogglePlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                enable_components::<T>,
                disable_components::<T>,
                toggle_components::<T>,
                overwrite_disabled::<T>,
            ),
        );
    }
}
pub trait ToggleCommands {
    fn toggle<T: Component>(&mut self) -> &mut Self;
    fn enable<T: Component>(&mut self) -> &mut Self;
    fn disable<T: Component>(&mut self) -> &mut Self;
}

impl<'w> ToggleCommands for EntityCommands<'w> {
    fn toggle<T: Component>(&mut self) -> &mut Self {
        self.insert(Toggle::<T>(PhantomData))
    }

    fn enable<T: Component>(&mut self) -> &mut Self {
        self.insert(Enable::<T>(PhantomData))
    }

    fn disable<T: Component>(&mut self) -> &mut Self {
        self.insert(Disable::<T>(PhantomData))
    }
}
