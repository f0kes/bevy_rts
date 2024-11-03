use bevy::{math::Vec3, reflect::Enum, utils::HashMap};
use serde::Serialize;
use std::fs::{self, DirEntry};
use std::path::{Path, PathBuf};


pub trait DirectionalRotationMatcher {
    fn get_similarity(&self, movement_vector: Vec3) -> f32;
}
pub trait AnimationTypes {
    type CharacterName: Clone + Serialize;
    type AnimationName: Clone + Serialize;
    type Rotation: DirectionalRotationMatcher + Clone + Serialize;
}
pub struct AnimationGenerationParameters<T: AnimationTypes> {
    pub character_aliases: HashMap<String, T::CharacterName>,
    pub animation_aliases: HashMap<String, T::AnimationName>,
    pub rotation_aliases: HashMap<String, T::Rotation>,
    pub root_folder: String,
}
#[derive(Serialize)]
struct AnimationData<T: AnimationTypes> {
    character: T::CharacterName,
    animation: T::AnimationName,
    rotation: T::Rotation,
    frames: Vec<String>,
}
#[derive(Serialize)]
struct AnimationsCollection<T: AnimationTypes> {
    animations: Vec<AnimationData<T>>,
}
//recursively traverses all folders. the root folder contains character folders
//next level name is character name, use alias to get the character name
//next level folder is animation name, use alias to get the animation name
//next level folder is rotation name, use alias to get the rotation
//the rotation folder contains a bunch of png files. extract file paths, sort them with natural sort(alphabetically)
// the final ron is list (character name, animation name, rotation, vec<frame path>)

fn generate_animations_ron<T: AnimationTypes>(params: AnimationGenerationParameters<T>) {
    let root_path = Path::new(&params.root_folder);

    // Create a vector to store all animation data
    let mut animations: Vec<AnimationData<T>> = Vec::new();
    let char_dirs = match fs::read_dir(root_path) {
        Ok(dirs) => dirs,
        Err(_) => return,
    };

    // Process each character directory
    for char_entry in char_dirs
        .filter_map(Result::ok)
        .filter(|e| e.path().is_dir())
    {
        let char_name = match char_entry.file_name().to_string_lossy().to_string() {
            name => params.character_aliases.get(&name).cloned(),
            _ => continue,
        };
        let char_name = match char_name {
            Some(name) => name,
            None => continue,
        };

        // Get all animation directories for this character
        let anim_dirs = match fs::read_dir(char_entry.path()) {
            Ok(dirs) => dirs,
            Err(_) => continue,
        };

        // Process each animation directory
        for anim_entry in anim_dirs
            .filter_map(Result::ok)
            .filter(|e| e.path().is_dir())
        {
            let anim_name = match anim_entry.file_name().to_string_lossy().to_string() {
                name => params.animation_aliases.get(&name).cloned(),
                _ => continue,
            };
            let anim_name = match anim_name {
                Some(name) => name,
                None => continue,
            };

            // Get all rotation directories for this animation
            let rot_dirs = match fs::read_dir(anim_entry.path()) {
                Ok(dirs) => dirs,
                Err(_) => continue,
            };

            // Process each rotation directory
            for rot_entry in rot_dirs
                .filter_map(Result::ok)
                .filter(|e| e.path().is_dir())
            {
                let rot_name = match rot_entry.file_name().to_string_lossy().to_string() {
                    name => params.rotation_aliases.get(&name).cloned(),
                    _ => continue,
                };
                let rot_name = match rot_name {
                    Some(name) => name,
                    None => continue,
                };

                // Get and sort all PNG files
                let mut frames: Vec<String> = fs::read_dir(rot_entry.path())
                    .into_iter()
                    .flatten()
                    .flatten()
                    .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("png"))
                    .map(|e| e.path().to_string_lossy().to_string())
                    .collect();

                frames.sort();

                if !frames.is_empty() {
                    animations.push(AnimationData {
                        character: char_name.clone(),
                        animation: anim_name.clone(),
                        rotation: rot_name,
                        frames,
                    });
                }
            }
        }
    }

    // Create the final collection
    let collection = AnimationsCollection { animations };

    // Serialize to RON format
    let ron_string = ron::ser::to_string_pretty(&collection, ron::ser::PrettyConfig::default())
        .unwrap_or_else(|e| {
            println!("Error serializing to RON: {}", e);
            String::new()
        });

    // Write to file
    if let Err(e) = fs::write(root_path.join("animations.ron"), ron_string) {
        println!("Error writing RON file: {}", e);
    }
}
