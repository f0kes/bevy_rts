// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;
use bevy::render::texture::ImageSamplerDescriptor;
use bevy::window::PrimaryWindow;
use bevy::winit::WinitWindows;
use bevy::DefaultPlugins;
use bevy::{asset::AssetMetaCheck, render::texture::ImageSampler};
use bevy_editor_pls::prelude::*;
use bevy_game::animation_defintions::HiveMindAnimationTypes;
use bevy_game::GamePlugin;
use directional_animation::ron_generation::plugin::{AnimatePlugin, LoadAnimationPlugin};

use std::io::Cursor;
use winit::window::Icon;

fn main() {
    let mut app = App::new();
    app.insert_resource(Msaa::Off);

    app.insert_resource(ClearColor(Color::linear_rgb(0.4, 0.4, 0.4)));

    let window_plugin = WindowPlugin {
        primary_window: Some(Window {
            title: "Bevy game".to_string(), // ToDo
            // Bind to canvas included in `index.html`
            canvas: Some("#bevy".to_owned()),
            fit_canvas_to_parent: true,
            // Tells wasm not to override default event handling, like F5 and Ctrl+R
            prevent_default_event_handling: false,
            ..default()
        }),
        ..default()
    };

    let asset_plugin = AssetPlugin {
        meta_check: AssetMetaCheck::Never,
        ..default()
    };
    let image_plugin = ImagePlugin {
        default_sampler: ImageSamplerDescriptor::nearest(),
    };

    app.add_plugins(
        DefaultPlugins
            .set(window_plugin)
            .set(asset_plugin)
            .set(image_plugin),
    );
    app.add_plugins(EditorPlugin::default());

    app.add_plugins(AnimatePlugin::<HiveMindAnimationTypes>::default());
    app.add_plugins(GamePlugin);
    app.add_systems(Startup, set_window_icon);

    app.run();
}

// Sets the icon on windows and X11
fn set_window_icon(
    windows: NonSend<WinitWindows>,
    primary_window: Query<Entity, With<PrimaryWindow>>,
) {
    let primary_entity = primary_window.single();
    let Some(primary) = windows.get_window(primary_entity) else {
        return;
    };
    let icon_buf = Cursor::new(include_bytes!(
        "../build/macos/AppIcon.iconset/icon_256x256.png"
    ));
    if let Ok(image) = image::load(icon_buf, image::ImageFormat::Png) {
        let image = image.into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        let icon = Icon::from_rgba(rgba, width, height).unwrap();
        primary.set_window_icon(Some(icon));
    };
}
