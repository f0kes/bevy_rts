use bevy::reflect::Reflect;
use bevy::utils::HashMap;
use directional_animation::ron_generation::generate_animations_ron::generate_animations_ron;
use directional_animation::ron_generation::{
    AnimationGenerationParameters, AnimationTypes, DirectionalRotationMatcher,
};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

// Test implementation of required traits
#[derive(Clone, Serialize, Deserialize)]
enum TestCharacter {
    Wolf,
    Knight,
}

#[derive(Clone, Serialize, Deserialize)]
enum TestAnimation {
    Idle,
    Running,
    Attacking,
    Dying,
    Casting,
}

#[derive(Clone, Serialize, Deserialize)]
struct TestRotation(f32);

impl DirectionalRotationMatcher for TestRotation {
    fn get_similarity(&self, _movement_vector: bevy::math::Vec3) -> f32 {
        self.0
    }
}

#[derive(Deserialize,Serialize, Reflect)]
struct TestTypes;
impl AnimationTypes for TestTypes {
    type CharacterName = TestCharacter;
    type AnimationName = TestAnimation;
    type Rotation = TestRotation;
}

fn setup_test_directories(root: &str) {
    // Create root directory
    fs::create_dir_all(root).unwrap();

    // Create test structure: root/hero/walk/front/frame1.png
    let test_path = Path::new(root).join("hero").join("walk").join("front");
    fs::create_dir_all(&test_path).unwrap();

    // Create a test PNG file
    fs::write(test_path.join("frame1.png"), &[]).unwrap();
}
fn populate_rotation_aliases() -> HashMap<String, TestRotation> {
    let angles = "0,45,90,135,180,225,270,315";
    let mut rotation_aliases = HashMap::new();

    for angle_str in angles.split(',') {
        let angle: f32 = angle_str.parse().unwrap();
        rotation_aliases.insert(angle_str.to_string(), TestRotation(angle));
    }

    rotation_aliases
}

#[test]
fn test_animation_generation() {
    // Create test data
    let mut character_aliases = HashMap::new();
    character_aliases.insert("wolf".to_string(), TestCharacter::Wolf);

    let mut animation_aliases = HashMap::new();
    animation_aliases.insert("WOLK".to_string(), TestAnimation::Running);
    animation_aliases.insert("PUNch".to_string(), TestAnimation::Attacking);
    animation_aliases.insert("abiliti".to_string(), TestAnimation::Casting);

    let rotation_aliases = populate_rotation_aliases();

    let test_folder = "/run/host/var/home/f0kes/dev/bevy/bevy_rts/assets";
    let assets_folder = test_folder;
    let params: AnimationGenerationParameters<TestTypes> = AnimationGenerationParameters {
        character_aliases,
        animation_aliases,
        rotation_aliases,
        root_folder: test_folder.to_string(),
        assets_folder: assets_folder.to_string(),
        fps: 30.,
    };

    // Setup test directories

    // Generate animations RON
    generate_animations_ron(params);

    // Verify the RON file was created
    let ron_path = Path::new(test_folder).join("animations.ron");
    assert!(ron_path.exists(), "animations.ron file was not created");

    // Clean up test directories
}
