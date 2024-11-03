use bevy::app::App;
use bevy::prelude::*;
use bevy::reflect::Reflect;
use bevy::utils::HashMap;
use directional_animation::ron_generation::generate_animations_ron::generate_animations_ron;
use directional_animation::ron_generation::{
    AnimationAssetAppExt, AnimationGenerationParameters, AnimationTypes, AnimationsCollection,
    DirectionalRotationMatcher,
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

#[derive(Deserialize, Serialize, Reflect, Default)]
struct TestTypes;
impl AnimationTypes for TestTypes {
    type CharacterName = TestCharacter;
    type AnimationName = TestAnimation;
    type Rotation = TestRotation;
}

#[derive(Resource, Default)]
pub struct State {
    pub collection: Handle<AnimationsCollection<TestTypes>>,
    pub printed: bool,
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
fn get_generation_params(test_folder: &str) -> AnimationGenerationParameters<TestTypes> {
    let mut character_aliases = HashMap::new();
    character_aliases.insert("wolf".to_string(), TestCharacter::Wolf);

    let mut animation_aliases = HashMap::new();
    animation_aliases.insert("WOLK".to_string(), TestAnimation::Running);
    animation_aliases.insert("PUNch".to_string(), TestAnimation::Attacking);
    animation_aliases.insert("abiliti".to_string(), TestAnimation::Casting);

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
#[test]
fn test_animation_generation() {
    let test_folder = "/run/host/var/home/f0kes/dev/bevy/bevy_rts/assets";
    let params: AnimationGenerationParameters<TestTypes> = get_generation_params(&test_folder);
    generate_animations_ron(params);

    let ron_path = Path::new(test_folder).join("anim.ron");
    assert!(ron_path.exists(), "animations.ron file was not created");
}
#[test]
fn test_load_animation() {
    let mut app = App::new();
    let test_folder = "/run/host/var/home/f0kes/dev/bevy/bevy_rts/assets";
    let params: AnimationGenerationParameters<TestTypes> = get_generation_params(&test_folder);
    app.add_plugins((MinimalPlugins, AssetPlugin::default()));
    app.init_animation_assset::<TestTypes>();
    app.add_systems(Startup, load_animations);
    app.add_systems(Update, print_on_load);
    app.run();
}
fn load_animations(mut commands: Commands, server: Res<AssetServer>, mut state: ResMut<State>) {
    state.collection = server.load("wolf.anim.ron");
}
fn print_on_load(
    mut state: ResMut<State>,
    custom_assets: Res<Assets<AnimationsCollection<TestTypes>>>,
) {
    let collection = custom_assets.get(&state.collection);

    // Can't print results if the assets aren't ready
    if state.printed {
        return;
    }

    if collection.is_none() {
        info!("Custom Asset Not Ready");
        return;
    }

    let animations = &collection.unwrap().animations;
    let stri = animations.get(0).unwrap().rotation.0;
    info!("Custom asset loaded: {:?}", stri);

    // Once printed, we won't print again
    state.printed = true;
}
