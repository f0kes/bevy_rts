// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use avian3d::prelude::{DebugRender, PhysicsDebugPlugin, PhysicsGizmos};
use avian3d::PhysicsPlugins;
use bevy::asset::AssetMetaCheck;
use bevy::color::palettes::css::{RED, WHITE};
use bevy::prelude::*;
use bevy::render::texture::ImageSamplerDescriptor;
use bevy::window::PrimaryWindow;
use bevy::winit::WinitWindows;
use bevy::DefaultPlugins;
use bevy_editor_pls::prelude::*;

use bevy_game::player::PlayerPlugin;

use blenvy::{
    BlenvyPlugin, BlueprintAnimationPlayerLink, BlueprintAnimations,
    BlueprintInfo, GameWorldTag, HideUntilReady, SpawnBlueprint,
};
use outline::plugin::{CustomMaterialPlugin, TexturableMaterialPlugin, ToonShaderPlugin};
use outline::shader_material::OutlineMaterial;

use std::io::Cursor;
use std::time::Duration;
use winit::window::Icon;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Dude;

fn main() {
    let mut app = App::new();
    let gismo_config = GizmoConfig {
        enabled: false,
        ..default()
    };
    app.insert_resource(Msaa::Off);

    //app.insert_resource(ClearColor(Color::linear_rgb(0.4, 0.4, 0.4)));

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
    

    app.add_plugins(
        DefaultPlugins
            .set(window_plugin)
            .set(asset_plugin)
            
    );
    app.add_plugins(EditorPlugin::default());
    //app.add_plugins(BlenvyPlugin::default());
    app.register_type::<Dude>();

    //app.add_plugins(GamePlugin);
    app.add_systems(Startup, set_window_icon);
    app.add_systems(Startup, setup);
    app.add_systems(Update, animation_control);
    app.add_plugins(PlayerPlugin);
    app.add_plugins(TexturableMaterialPlugin::<OutlineMaterial>::default());
    app.add_plugins(ToonShaderPlugin);
    app.add_plugins(PhysicsPlugins::default());
    //app.add_plugins(PhysicsDebugPlugin::default());
    app.run();
    app.insert_gizmo_config(
        PhysicsGizmos {
            aabb_color: Some(Color::linear_rgb(0., 0., 1.)),
            ..default()
        },
        gismo_config,
    );
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

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SceneBundle {
            scene: asset_server.load("levels/World.glb#Scene0"),
            ..Default::default()
        },
        // SpawnBlueprint,
        // HideUntilReady,
        // GameWorldTag,
    ));
   
    //commands.spawn(DebugRender::default());
}

pub fn animation_control(
    animated_dudes: Query<
        (&BlueprintAnimationPlayerLink, &BlueprintAnimations),
        With<Dude>,
    >,

    mut animation_players: Query<(
        &mut AnimationPlayer,
        &mut AnimationTransitions,
    )>,

    keycode: Res<ButtonInput<KeyCode>>,
    // mut entities_with_animations : Query<(&mut AnimationPlayer, &mut BlueprintAnimations)>,
) {
    // robots
    if keycode.just_pressed(KeyCode::Digit1) {
        println!("scan animation for robots");
        for (link, animations) in animated_dudes.iter() {
            let (mut animation_player, mut animation_transitions) =
                animation_players.get_mut(link.0).unwrap();
            println!("got some animations");
            let anim_name = "Idle";
            animation_transitions
                .play(
                    &mut animation_player,
                    *animations
                        .named_indices
                        .get(anim_name)
                        .expect("animation name should be in the list"),
                    Duration::from_secs(5),
                )
                .repeat();
        }
    }

    // foxes
    if keycode.just_pressed(KeyCode::Digit2) {
        for (link, animations) in animated_dudes.iter() {
            let (mut animation_player, mut animation_transitions) =
                animation_players.get_mut(link.0).unwrap();

            let anim_name = "Run";
            animation_transitions
                .play(
                    &mut animation_player,
                    *animations
                        .named_indices
                        .get(anim_name)
                        .expect("animation name should be in the list"),
                    Duration::from_secs(5),
                )
                .repeat();
        }
    }

    if keycode.just_pressed(KeyCode::Digit3) {
        for (link, animations) in animated_dudes.iter() {
            let (mut animation_player, mut animation_transitions) =
                animation_players.get_mut(link.0).unwrap();

            let anim_name = "Melee";
            animation_transitions
                .play(
                    &mut animation_player,
                    *animations
                        .named_indices
                        .get(anim_name)
                        .expect("animation name should be in the list"),
                    Duration::from_secs(5),
                )
                .repeat();
        }
    }

    if keycode.just_pressed(KeyCode::Digit4) {
        for (link, animations) in animated_dudes.iter() {
            let (mut animation_player, mut animation_transitions) =
                animation_players.get_mut(link.0).unwrap();

            let anim_name = "Hit";
            animation_transitions
                .play(
                    &mut animation_player,
                    *animations
                        .named_indices
                        .get(anim_name)
                        .expect("animation name should be in the list"),
                    Duration::from_secs(5),
                )
                .repeat();
        }
    }
    if keycode.just_pressed(KeyCode::Digit5) {
        for (link, animations) in animated_dudes.iter() {
            let (mut animation_player, mut animation_transitions) =
                animation_players.get_mut(link.0).unwrap();

            let anim_name = "Dig";
            animation_transitions
                .play(
                    &mut animation_player,
                    *animations
                        .named_indices
                        .get(anim_name)
                        .expect("animation name should be in the list"),
                    Duration::from_secs(5),
                )
                .repeat();
        }
    }
}
