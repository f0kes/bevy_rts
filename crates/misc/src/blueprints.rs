use std::marker::PhantomData;

use bevy::{
    ecs::system::EntityCommands, prelude::*, reflect::GetTypeRegistration,
};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, SystemSet)]
pub struct BlueprintsSet;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, SystemSet)]
pub enum BlueprintSet {
    #[default]
    Cleanup,
    Sync,
    Flush,
}

pub trait FromBlueprint<T> {
    fn from_blueprint(blueprint: &T) -> Self;
}

#[derive(Debug, Component, Default, Reflect)]
#[reflect(Component)]
pub struct Blueprint<B: Default>(B);

impl<B: Default> Blueprint<B> {
    pub fn new(data: B) -> Self {
        Blueprint(data)
    }
}

#[derive(Debug, Component, Default, Reflect)]
#[reflect(Component)]
pub struct IsBlueprint;

pub struct AsSelf;

pub trait BlueprintTarget {
    fn remove_target_bundle<T, P: Bundle + FromBlueprint<T>>(
        entity: &mut EntityCommands,
    );

    fn attach_target_bundle<T, P: Bundle + FromBlueprint<T>>(
        entity: &mut EntityCommands,
        bundle: P,
    );
}

impl BlueprintTarget for AsSelf {
    fn remove_target_bundle<T, P: Bundle + FromBlueprint<T>>(
        entity: &mut EntityCommands,
    ) {
        entity.remove::<IsBlueprint>();
        entity.remove::<P>();
    }

    fn attach_target_bundle<T, P: Bundle + FromBlueprint<T>>(
        entity: &mut EntityCommands,
        bundle: P,
    ) {
        entity.insert(IsBlueprint);
        entity.insert(bundle);
    }
}

pub struct BlueprintPlugin<
    B,
    P: Bundle + FromBlueprint<B>,
    T: BlueprintTarget = AsSelf,
> {
    blueprint_marker: PhantomData<B>,
    prefab_marker: PhantomData<P>,
    target_marker: PhantomData<T>,
}

impl<B, P, T> Default for BlueprintPlugin<B, P, T>
where
    P: Bundle + FromBlueprint<B>,
    T: BlueprintTarget,
{
    fn default() -> Self {
        Self {
            blueprint_marker: PhantomData::<B>,
            prefab_marker: PhantomData::<P>,
            target_marker: PhantomData::<T>,
        }
    }
}

impl<B, P, T> BlueprintPlugin<B, P, T>
where
    B: Default + Send + Sync + 'static,
    P: Bundle + FromBlueprint<B>,
    T: BlueprintTarget,
{
    fn should_sync_blueprint(
        blueprint_query: Query<(), Changed<Blueprint<B>>>,
    ) -> bool {
        !blueprint_query.is_empty()
    }

    fn sync_blueprint_prefab(
        mut commands: Commands,
        blueprint_query: Query<(Entity, &Blueprint<B>), Changed<Blueprint<B>>>,
    ) {
        for (entity, blueprint) in blueprint_query.iter() {
            let mut entity_commands = commands.entity(entity);
            T::remove_target_bundle::<B, P>(&mut entity_commands);
            T::attach_target_bundle::<B, P>(
                &mut entity_commands,
                P::from_blueprint(&blueprint.0),
            );
        }
    }

    fn handle_removed_blueprints(
        mut commands: Commands,
        mut blueprint_query: RemovedComponents<Blueprint<B>>,
    ) {
        for entity in blueprint_query.read() {
            if let Some(mut entity_commands) = commands.get_entity(entity) {
                T::remove_target_bundle::<B, P>(&mut entity_commands);
            }
        }
    }
}

impl<B, P, T> Plugin for BlueprintPlugin<B, P, T>
where
    B: Default
        + GetTypeRegistration
        + FromReflect
        + TypePath
        + Send
        + Sync
        + 'static,
    P: Bundle + FromBlueprint<B>,
    T: BlueprintTarget + Send + Sync + 'static,
{
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                Self::handle_removed_blueprints.in_set(BlueprintSet::Cleanup),
                Self::sync_blueprint_prefab
                    .in_set(BlueprintSet::Sync)
                    .run_if(Self::should_sync_blueprint),
            ),
        );

        app.register_type::<Blueprint<B>>().register_type::<B>();
    }
}

pub struct BlueprintsPlugin;

impl Plugin for BlueprintsPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (
                BlueprintSet::Cleanup,
                BlueprintSet::Sync,
                BlueprintSet::Flush,
            )
                .chain()
                .in_set(BlueprintsSet),
        )
        .add_systems(Update, apply_deferred.in_set(BlueprintSet::Flush));
    }
}
