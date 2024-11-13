use bevy::{prelude::*, utils::HashMap};
use directional_animation::ron_generation::{
    AnimationGenerationParameters, AnimationTypes, DirectionalRotationMatcher,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Component)]
pub enum Character {
    Wolf,
    Knight,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Component)]
pub enum AnimationType {
    Idle,
    Running,
    Attacking,
    Dying,
    Casting,
}
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CharacterRotationDegrees(u32);
impl DirectionalRotationMatcher for CharacterRotationDegrees {
    fn get_similarity(&self, movement_vector: bevy::math::Vec3) -> f32 {
        let angle = self.0 as f32;
        let movement_angle = movement_vector.x.atan2(-movement_vector.y).to_degrees();

        // Normalize angles to 0-360 range
        let movement_angle = ((movement_angle + 360.0) % 360.0) as u32 as f32;

        // Calculate shortest angle difference
        let mut difference = (angle - movement_angle).abs();
        if difference > 180.0 {
            difference = 360.0 - difference;
        }

        // Return normalized similarity (1.0 is perfect match, 0.0 is worst match)
        1.0 - (difference / 180.0)
    }
}

#[derive(Deserialize, Serialize, Reflect, Default, PartialEq, Eq, Hash)]
pub struct HiveMindAnimationTypes;
impl AnimationTypes for HiveMindAnimationTypes {
    type CharacterName = Character;
    type AnimationName = AnimationType;
    type Rotation = CharacterRotationDegrees;
}
fn populate_rotation_aliases() -> HashMap<String, CharacterRotationDegrees> {
    let mut rotation_aliases = HashMap::new();

    for i in (0..360).step_by(45) {
        rotation_aliases.insert(i.to_string(), CharacterRotationDegrees(i));
    }

    rotation_aliases
}
fn get_generation_params(
    test_folder: &str,
) -> AnimationGenerationParameters<HiveMindAnimationTypes> {
    let mut character_aliases = HashMap::new();
    character_aliases.insert("wolf".to_string(), Character::Wolf);

    let mut animation_aliases = HashMap::new();
    animation_aliases.insert("WOLK".to_string(), AnimationType::Running);
    animation_aliases.insert("PUNch".to_string(), AnimationType::Attacking);
    animation_aliases.insert("abiliti".to_string(), AnimationType::Casting);

    let rotation_aliases = populate_rotation_aliases();

    let assets_folder = test_folder;
    AnimationGenerationParameters {
        character_aliases,
        animation_aliases,
        rotation_aliases,
        root_folder: test_folder.to_string(),
        assets_folder: assets_folder.to_string(),
        fps: 30.,
    }
}
