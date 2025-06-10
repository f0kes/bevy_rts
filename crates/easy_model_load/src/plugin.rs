use bevy::{prelude::*, utils::HashMap};

use crate::cube::spawn_cube;
pub struct EasyModelLoadPlugin;

#[derive(Component)]
pub struct LoadModel {
    pub path: String,
}
#[derive(Component)]
pub struct PreLoadModel {
    pub path: String,
}

#[derive(Component)]
pub struct UnloadModel {
    pub path: String,
}

#[derive(Resource, Default)]
pub struct PreloadedModels {
    pub models: HashMap<String, Handle<Scene>>,
}

impl Plugin for EasyModelLoadPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PreloadedModels>();
        app.add_systems(Update, load_models);
        app.add_systems(Update, preload_models);
        app.add_systems(Update, unload_models);
        app.add_systems(Update, spawn_cube);
    }
}

pub fn load_models(
    query: Query<(Entity, &LoadModel)>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    for (entity, request) in query.iter() {
        let handle = asset_server.load(request.path.as_str());
        commands.entity(entity).insert(SceneRoot(handle));
    }
}

pub fn preload_models(
    query: Query<&PreLoadModel>,
    asset_server: Res<AssetServer>,
    mut preloaded_models: ResMut<PreloadedModels>,
) {
    for request in query.iter() {
        preloaded_models.models.insert(
            request.path.clone(),
            asset_server.load(request.path.as_str()),
        );
    }
}
pub fn unload_models(
    query: Query<&UnloadModel>,
    mut preloaded_models: ResMut<PreloadedModels>,
) {
    for request in query.iter() {
        preloaded_models.models.remove(&request.path);
    }
}
