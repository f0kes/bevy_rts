use bevy::prelude::*;
use std::marker::PhantomData;

use super::{
    animation_library::{
        are_all_animation_sprites_loaded, load_sprites, AnimationWithPathsToHandles,
    },
    AnimationTypes, AnimationsCollection,
};

// 1. Loading states
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AnimationLoadingState {
    #[default]
    LoadingAnimFiles,
    LoadingSprites,
    BuildingLibrary,
    Complete,
}

// 2. Resource for animation paths
#[derive(Resource)]
pub struct AnimationPaths {
    paths: Vec<String>,
}

impl Default for AnimationPaths {
    fn default() -> Self {
        Self {
            paths: vec![
                "wolf.anim.ron".to_string(),
                // Add more paths here
            ],
        }
    }
}

// 3. Resource to track loaded animations
#[derive(Resource, Default)]
pub struct AnimationCollections<T: AnimationTypes> {
    collections: Vec<Handle<AnimationsCollection<T>>>,
}

// 4. Systems
pub fn load_animation_files<T: AnimationTypes>(
    asset_server: Res<AssetServer>,
    animation_paths: Res<AnimationPaths>,
    mut collection_resource: ResMut<AnimationCollections<T>>,
) {
    if collection_resource.collections.is_empty() {
        collection_resource.collections = animation_paths
            .paths
            .iter()
            .map(|path| asset_server.load(path))
            .collect();
    }
}

pub fn check_animations_loaded<T: AnimationTypes>(
    mut next_state: ResMut<NextState<AnimationLoadingState>>,
    asset_server: Res<AssetServer>,
    animation_collections: Res<AnimationCollections<T>>,
) {
    let all_loaded = animation_collections
        .collections
        .iter()
        .all(|handle| asset_server.is_loaded_with_dependencies(handle));

    if all_loaded {
        next_state.set(AnimationLoadingState::LoadingSprites);
    }
}

pub fn check_sprites_loaded<T: AnimationTypes>(
    mut next_state: ResMut<NextState<AnimationLoadingState>>,
    asset_server: Res<AssetServer>,
    animations_with_handles: Res<AnimationWithPathsToHandles<T>>,
) {
    let all_loaded =
        are_all_animation_sprites_loaded(&animations_with_handles.paths_to_handles, &asset_server);
    if all_loaded {
        next_state.set(AnimationLoadingState::BuildingLibrary);
    }
}
pub fn build_animation_library<T: AnimationTypes>(
    mut next_state: ResMut<NextState<AnimationLoadingState>>,
    animations_with_handles: Res<AnimationWithPathsToHandles<T>>,
    textures: ResMut<Assets<Image>>,
) {
    animations_with_handles.build_animation_library(textures);
    next_state.set(AnimationLoadingState::Complete);
}
pub struct AnimationLoaderPlugin<T: AnimationTypes> {
    phantom: PhantomData<T>,
    paths: Vec<String>,
}

impl<T: AnimationTypes> AnimationLoaderPlugin<T> {
    pub fn new(paths: Vec<String>) -> Self {
        Self {
            phantom: PhantomData,
            paths,
        }
    }
}
impl<T: AnimationTypes> Default for AnimationLoaderPlugin<T> {
    fn default() -> Self {
        Self {
            phantom: Default::default(),
            paths: Default::default(),
        }
    }
}

impl<T: AnimationTypes> Plugin for AnimationLoaderPlugin<T> {
    fn build(&self, app: &mut App) {
        app.init_state::<AnimationLoadingState>()
            .insert_resource(AnimationPaths {
                paths: self.paths.clone(),
            })
            .init_resource::<AnimationCollections<T>>()
            .add_systems(
                OnEnter(AnimationLoadingState::LoadingAnimFiles),
                load_animation_files::<T>,
            )
            .add_systems(
                Update,
                check_animations_loaded::<T>
                    .run_if(in_state(AnimationLoadingState::LoadingAnimFiles)),
            )
            .add_systems(
                OnEnter(AnimationLoadingState::LoadingSprites),
                load_sprites::<T>,
            )
            .add_systems(
                OnEnter(AnimationLoadingState::BuildingLibrary),
                build_animation_library::<T>,
            );
    }
}
