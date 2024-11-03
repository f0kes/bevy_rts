pub mod ron_generation;
use bevy::ecs::system::SystemState;
use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::utils::HashMap;
use bevy_asset_loader::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            RonAssetPlugin::<CustomDynamicAssetCollection>::new(&["my-assets.ron"]),
        ))
        // We need to make sure that our dynamic asset collections can be loaded from the asset file
        .init_state::<MyStates>()
        .add_loading_state(
            LoadingState::new(MyStates::AssetLoading)
                .continue_to_state(MyStates::Next)
                .load_collection::<MyAssets>()
                .register_dynamic_asset_collection::<CustomDynamicAssetCollection>()
                .with_dynamic_assets_file::<CustomDynamicAssetCollection>("custom.my-assets.ron"),
        )
        .add_systems(OnEnter(MyStates::Next), render_stuff)
        .run();
}
