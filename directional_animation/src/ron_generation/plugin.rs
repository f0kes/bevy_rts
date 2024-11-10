use bevy::prelude::*;
use std::marker::PhantomData;

use super::{
    animation_library::{
        self, are_all_animation_sprites_loaded, load_sprites, AnimationLibrary,
        AnimationWithPathsToHandles, AnimationsWithPaths,
    },
    animator::{animate, change_animation},
    AnimationLoader, AnimationTypes, AnimationsCollection,
};

// 1. Loading states
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States)]
pub enum AnimationLoadingState {
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

// 4. Systems
pub fn load_animation_files<T: AnimationTypes>(
    asset_server: Res<AssetServer>,
    animation_paths: Res<AnimationPaths>,
    mut collection_resource: ResMut<AnimationWithPathsToHandles<T>>,
) {
    let animation_definitions: Vec<Handle<AnimationsWithPaths<T>>> = animation_paths
        .paths
        .iter()
        .map(|path| asset_server.load(path))
        .collect();
    collection_resource.paths_to_handles.extend(
        animation_definitions
            .iter()
            .map(|handle| (handle.clone(), None)),
    );
}

pub fn check_animations_loaded<T: AnimationTypes>(
    mut next_state: ResMut<NextState<AnimationLoadingState>>,
    asset_server: Res<AssetServer>,
    animation_collections: Res<AnimationWithPathsToHandles<T>>,
) {
    let all_loaded = animation_collections
        .paths_to_handles
        .iter()
        .all(|(handle, _)| asset_server.is_loaded_with_dependencies(handle));

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
    texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut animation_library: ResMut<AnimationLibrary<T>>,
) {
    animation_library.animations = animations_with_handles
        .build_animation_library(textures, texture_atlas_layouts)
        .animations;
    next_state.set(AnimationLoadingState::Complete);
}

#[derive(Default)]
pub struct LoadAnimationPlugin<T: AnimationTypes> {
    phantom: PhantomData<T>,
    paths: Option<Vec<String>>,
}

impl<T: AnimationTypes> LoadAnimationPlugin<T> {
    pub fn new(paths: Vec<String>) -> Self {
        Self {
            phantom: PhantomData,
            paths: Some(paths),
        }
    }
}

impl<T: AnimationTypes> Plugin for LoadAnimationPlugin<T> {
    fn build(&self, app: &mut App) {
        app.init_asset::<AnimationsCollection<T>>();
        app.init_asset_loader::<AnimationLoader<T>>();
        app.insert_state(AnimationLoadingState::LoadingAnimFiles);
        app.insert_resource(if let Some(ref paths) = self.paths {
            AnimationPaths {
                paths: paths.clone(),
            }
        } else {
            AnimationPaths::default()
        });

        app.init_resource::<AnimationWithPathsToHandles<T>>();
        app.init_resource::<AnimationLibrary<T>>();
        app.add_systems(
            OnEnter(AnimationLoadingState::LoadingAnimFiles),
            load_animation_files::<T>,
        );
        app.add_systems(
            Update,
            check_animations_loaded::<T>.run_if(in_state(AnimationLoadingState::LoadingAnimFiles)),
        );
        app.add_systems(
            OnEnter(AnimationLoadingState::LoadingSprites),
            load_sprites::<T>,
        );
        app.add_systems(
            Update,
            check_sprites_loaded::<T>.run_if(in_state(AnimationLoadingState::LoadingSprites)),
        );
        app.add_systems(
            OnEnter(AnimationLoadingState::BuildingLibrary),
            build_animation_library::<T>,
        );
    }
}

pub struct AnimatePlugin<T: AnimationTypes> {
    phantom: PhantomData<T>,
}

impl<T: AnimationTypes> Default for AnimatePlugin<T> {
    fn default() -> Self {
        Self {
            phantom: Default::default(),
        }
    }
}

impl<T: AnimationTypes> Plugin for AnimatePlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            change_animation::<T>.run_if(in_state(AnimationLoadingState::Complete)),
        );
        app.add_systems(
            Update,
            animate.run_if(in_state(AnimationLoadingState::Complete)),
        );
    }
}
