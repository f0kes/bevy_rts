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
    query: Query<(Entity, Option<&T>, &Disable<T>)>,
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
                    .insert(Disabled::from(component.clone()))
                    .remove::<T>();
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

// Example usage
#[derive(Component, Clone, Debug)]
struct Player {
    speed: f32,
}

fn setup(mut commands: Commands) {
    commands.spawn((Player { speed: 5.0 }, Transform::default()));
}

fn input_system(
    mut commands: Commands,
    query: Query<Entity, Or<(With<Player>, With<Disabled<Player>>)>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    for entity in query.iter() {
        if keyboard.just_pressed(KeyCode::Space) {
            commands.entity(entity).toggle::<Player>();
        }
    }
}

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ComponentTogglePlugin::<Player>::default())
        .add_plugins(ComponentTogglePlugin::<Transform>::default())
        .add_systems(Startup, setup)
        .add_systems(Update, input_system)
        .run();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_disabled_from() {
        let disabled = Disabled { data: 0 };
        let disabled_2 = Disabled::from(0);
        assert_eq!(disabled.data, 0);
        assert_eq!(disabled, disabled_2);
    }
}
