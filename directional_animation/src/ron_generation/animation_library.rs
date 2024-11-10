use bevy::{asset::LoadState, prelude::*, render::texture::ImageSampler, utils::HashMap};

use super::{AnimationLoadData, AnimationTypes, AnimationsCollection, DirectionalRotationMatcher};

type ImageHandles = Vec<Handle<Image>>;

pub type AnimationsWithPaths<T> = AnimationsCollection<T>;
pub type AnimationsWithHandles<T> = Vec<AnimationWithHandles<T>>;

pub struct AnimationWithHandles<T: AnimationTypes> {
    pub character: T::CharacterName,
    pub animation: T::AnimationName,
    pub rotation: T::Rotation,
    pub frames: ImageHandles,
    pub fps: f32,
}
#[derive(PartialEq, Eq, Hash)]
pub struct AnimationKey<T: AnimationTypes> {
    pub character: T::CharacterName,
    pub animation: T::AnimationName,
    pub rotation: T::Rotation,
}

#[derive(Component, Clone)]
pub struct MyAnimationClip {
    pub len: usize,
    pub fps: f32,
    pub texture_atlas_layout_handle: Handle<TextureAtlasLayout>,
    pub texture_atlas: Handle<Image>,
}

impl<T: AnimationTypes> From<&AnimationLoadData<T>> for AnimationWithHandles<T> {
    fn from(data: &AnimationLoadData<T>) -> Self {
        AnimationWithHandles {
            character: data.character.clone(),
            animation: data.animation.clone(),
            rotation: data.rotation.clone(),
            frames: Vec::new(),
            fps: data.fps,
        }
    }
}
#[derive(Resource, Default)]
pub struct AnimationLibrary<T: AnimationTypes> {
    pub animations: HashMap<AnimationKey<T>, MyAnimationClip>,
}
impl<T: AnimationTypes> AnimationLibrary<T> {
    pub fn new() -> Self {
        Self {
            animations: HashMap::new(),
        }
    }
    pub fn get_animation(&self, key: &AnimationKey<T>) -> Option<&MyAnimationClip> {
        self.animations.get(key)
    }
    pub fn add_animation(&mut self, key: AnimationKey<T>, clip: MyAnimationClip) {
        self.animations.insert(key, clip);
    }
    pub fn remove_animation(&mut self, key: &AnimationKey<T>) {
        self.animations.remove(key);
    }
    pub fn find_animation(
        &self,
        character: &T::CharacterName,
        animation: &T::AnimationName,
        movement_vector: Vec3,
    ) -> Option<&MyAnimationClip> {
        let mut best_similarity = 0.0;
        let mut best_key = None;
        for key in self.animations.keys() {
            if key.character == *character && key.animation == *animation {
                let similarity = key.rotation.get_similarity(movement_vector);
                if similarity > best_similarity {
                    best_similarity = similarity;
                    best_key = Some(key);
                }
            }
        }
        best_key.and_then(|key| self.get_animation(key))
    }
}

#[derive(Resource, Default)]
pub struct AnimationWithPathsToHandles<T: AnimationTypes> {
    pub paths_to_handles: HashMap<Handle<AnimationsWithPaths<T>>, Option<AnimationsWithHandles<T>>>,
}
impl<T: AnimationTypes> AnimationWithPathsToHandles<T> {
    pub fn add_collection(&mut self, collection: Handle<AnimationsCollection<T>>) {
        self.paths_to_handles.insert(collection, None);
    }
    pub fn build_animation_library(
        &self,
        mut textures: ResMut<Assets<Image>>,
        mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    ) -> AnimationLibrary<T> {
        // Create a temporary vector to store our results
        let mut results = Vec::new();

        // Process animations and collect results
        for animations_opt in self.paths_to_handles.values() {
            if let Some(animations) = animations_opt {
                for animation in animations {
                    let key = AnimationKey {
                        character: animation.character.clone(),
                        animation: animation.animation.clone(),
                        rotation: animation.rotation.clone(),
                    };

                    // Create texture atlas outside the closure
                    let (texture_atlas_layout, texture_atlas) =
                        create_texture_atlas(animation.frames.clone(), None, None, &mut textures);
                    let texture_atlas_layout_handle =
                        texture_atlas_layouts.add(texture_atlas_layout);
                    let clip = MyAnimationClip {
                        len: animation.frames.len(),
                        fps: animation.fps,
                        texture_atlas_layout_handle,
                        texture_atlas,
                    };

                    results.push((key, clip));
                }
            }
        }

        AnimationLibrary {
            animations: results.into_iter().collect(),
        }
    }
}

pub fn are_all_animation_sprites_loaded<T: AnimationTypes>(
    paths_to_handles: &HashMap<Handle<AnimationsWithPaths<T>>, Option<AnimationsWithHandles<T>>>,
    asset_server: &AssetServer,
) -> bool {
    paths_to_handles
        .iter()
        .all(|(_handle, animations_with_handles_option)| {
            if animations_with_handles_option.is_none() {
                return false;
            }

            let animations_with_handles = animations_with_handles_option.as_ref().unwrap();
            animations_with_handles.iter().all(|animation| {
                animation.frames.iter().all(|frame_handle| {
                    let state_option = asset_server.get_load_state(frame_handle);
                    let state = state_option.unwrap_or(LoadState::NotLoaded);
                    state == LoadState::Loaded
                })
            })
        })
}

pub fn create_texture_atlas(
    handles: Vec<Handle<Image>>,
    padding: Option<UVec2>,
    sampling: Option<ImageSampler>,
    textures: &mut ResMut<Assets<Image>>,
) -> (TextureAtlasLayout, Handle<Image>) {
    // Build a texture atlas using the individual sprites
    let mut texture_atlas_builder = TextureAtlasBuilder::default();
    texture_atlas_builder.padding(padding.unwrap_or_default());
    for handle in handles.iter() {
        let id = handle.id();
        let Some(texture) = textures.get(id) else {
            warn!(
                "{:?} did not resolve to an `Image` asset.",
                handle.path().unwrap()
            );
            continue;
        };

        texture_atlas_builder.add_texture(Some(id), texture);
    }
    texture_atlas_builder.max_size(UVec2::new(16384, 16384));
    let (texture_atlas_layout, texture) = texture_atlas_builder.build().unwrap();
    let texture = textures.add(texture);

    // Update the sampling settings of the texture atlas
    let image = textures.get_mut(&texture).unwrap();
    image.sampler = sampling.unwrap_or_default();

    (texture_atlas_layout, texture)
}

pub fn load_sprites<T: AnimationTypes>(
    asset_server: Res<AssetServer>,
    mut animation_collections: ResMut<AnimationWithPathsToHandles<T>>,
    animation_collections_assets: Res<Assets<AnimationsCollection<T>>>,
) {
    for (animations_with_paths_handle, animations_with_handles_option) in
        animation_collections.paths_to_handles.iter_mut()
    {
        if animations_with_handles_option.is_some() {
            continue;
        }

        let animations_with_paths =
            match animation_collections_assets.get(animations_with_paths_handle) {
                Some(collection) => collection,
                None => continue,
            };

        let animations_with_handles: AnimationsWithHandles<T> = animations_with_paths
            .animations
            .iter()
            .map(|animation| {
                let mut animation_with_handles = AnimationWithHandles::<T>::from(animation);
                animation_with_handles.frames = animation
                    .frames
                    .iter()
                    .map(|path| asset_server.load(path))
                    .collect();
                animation_with_handles
            })
            .collect();

        *animations_with_handles_option = Some(animations_with_handles);
    }
}
