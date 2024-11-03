pub mod generate_animations_ron;

use bevy::app::App;
use bevy::asset::{Asset, AssetApp, AssetLoader, AsyncReadExt};
use bevy::reflect::{Reflect, TypePath};
use bevy::{math::Vec3, utils::HashMap};
use serde::{Deserialize, Serialize};
use std::fs::{self};
use std::path::Path;
use thiserror::Error;

pub trait DirectionalRotationMatcher {
    fn get_similarity(&self, movement_vector: Vec3) -> f32;
}
pub trait Converter<From, To> {
    fn convert(&self, from: From) -> To;
}
impl<K, V> Converter<&K, V> for HashMap<K, V>
where
    K: Eq + std::hash::Hash,
    V: Clone + Serialize,
{
    fn convert(&self, from: &K) -> V {
        self.get(from).unwrap().clone()
    }
}

pub trait AnimationTypes: Deserialize<'static> + Serialize + Reflect + TypePath + Default {
    type CharacterName: Clone + Serialize + for<'a> Deserialize<'a> + Send + Sync;
    type AnimationName: Clone + Serialize + for<'a> Deserialize<'a> + Send + Sync;
    type Rotation: DirectionalRotationMatcher
        + Clone
        + Serialize
        + for<'a> Deserialize<'a>
        + Send
        + Sync;
}

//TODO: utilize converters
pub struct AnimationGenerationParameters<T: AnimationTypes> {
    pub character_aliases: HashMap<String, T::CharacterName>,
    pub animation_aliases: HashMap<String, T::AnimationName>,
    pub rotation_aliases: HashMap<String, T::Rotation>,
    pub root_folder: String,
    pub assets_folder: String,
    pub fps: f32,
}
#[derive(Serialize, Deserialize, Asset, TypePath)]
pub struct AnimationData<T: AnimationTypes> {
    pub character: T::CharacterName,
    pub animation: T::AnimationName,
    pub rotation: T::Rotation,
    pub frames: Vec<String>,
    pub fps: f32,
}
#[derive(Serialize, Asset, TypePath)]
pub struct AnimationsCollection<T: AnimationTypes> {
    pub animations: Vec<AnimationData<T>>,
}
impl<'de, T: AnimationTypes> Deserialize<'de> for AnimationsCollection<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(AnimationsCollection {
            animations: Vec::deserialize(deserializer)?,
        })
    }
}

#[derive(Default)]
pub struct AnimationLoader<T: AnimationTypes> {
    phantom: std::marker::PhantomData<T>,
}
#[non_exhaustive]
#[derive(Debug, Error)]
pub enum AnimationLoaderError {
    /// An [IO](std::io) Error
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),
    /// A [RON](ron) Error
    #[error("Could not parse RON: {0}")]
    RonSpannedError(#[from] ron::error::SpannedError),
}
impl<AT: AnimationTypes> AssetLoader for AnimationLoader<AT> {
    fn extensions(&self) -> &[&str] {
        &["anim.ron"]
    }

    type Asset = AnimationsCollection<AT>;

    type Settings = ();

    type Error = AnimationLoaderError;

    async fn load<'a>(
        &'a self,
        reader: &'a mut bevy::asset::io::Reader<'_>,
        settings: &'a Self::Settings,
        load_context: &'a mut bevy::asset::LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let custom_asset = ron::de::from_bytes::<AnimationsCollection<AT>>(&bytes)?;
        Ok(custom_asset)
    }
}
pub trait AnimationAssetAppExt {
    fn init_animation_assset<T: AnimationTypes>(&mut self) -> &mut Self;
}
impl AnimationAssetAppExt for App {
    fn init_animation_assset<T: AnimationTypes>(&mut self) -> &mut Self {
        self.init_asset::<AnimationsCollection<T>>();
        self.init_asset_loader::<AnimationLoader<T>>()
    }
}
