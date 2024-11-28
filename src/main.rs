// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use avian3d::prelude::{
    ColliderConstructor, CollisionMargin, PhysicsDebugPlugin, PhysicsGizmos,
    RigidBody,
};
use avian3d::PhysicsPlugins;
use bevy::asset::AssetMetaCheck;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::pbr::{CascadeShadowConfigBuilder, ExtendedMaterial};
use bevy::prelude::*;

use bevy::window::{PresentMode, PrimaryWindow};
use bevy::winit::WinitWindows;
use bevy::DefaultPlugins;

use bevy_editor_pls::EditorPlugin;
use bevy_game::player::PlayerPlugin;

use camera::plugin::SmoothCameraPlugin;
use outline::clash_grass::{CheckerGrassExtension, CheckerGrassMaterialConfig};
use outline::grass::{GrassPlugin, SpawnGrass};
use outline::plugin::{
    MyMaterialsPlugin, TexturableMaterialPlugin, ToonShaderPlugin,
};
use outline::shader_material::OutlineMaterial;
use outline::toon_shader::{ToonShaderMaterial, ToonShaderSun};

use world_gen::perlin_terrain::PerlinTerrain;
use world_gen::terrain::{Terrain, TerrainLike, TerrainPlaneOptions};

use std::f32::consts::PI;
use std::io::Cursor;

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
            present_mode: PresentMode::AutoNoVsync, // TODO: Investigate left click render extraction spike when optimizing performance with VSync enabled
            ..default()
        }),
        ..default()
    };

    let asset_plugin = AssetPlugin {
        meta_check: AssetMetaCheck::Never,
        watch_for_changes_override: Some(true),
        ..default()
    };

    app.add_plugins(DefaultPlugins.set(window_plugin).set(asset_plugin));
    app.add_plugins(EditorPlugin::default());

    app.register_type::<Dude>();

    app.add_systems(Startup, set_window_icon);
    app.add_systems(Startup, setup);
    //app.add_systems(Update, animation_control);
    app.add_plugins(PlayerPlugin);
    app.add_plugins(MyMaterialsPlugin);
    app.add_plugins(PhysicsPlugins::default());
    app.add_plugins(PhysicsDebugPlugin::default());
    //app.add_plugins(GrassPlugin::<Terrain>::default());
    app.add_plugins(FrameTimeDiagnosticsPlugin::default());
    app.add_plugins(LogDiagnosticsPlugin::default());

    app.insert_gizmo_config(
        PhysicsGizmos {
            aabb_color: Some(Color::linear_rgb(0., 0., 1.)),
            ..default()
        },
        gismo_config,
    );
    app.add_plugins(SmoothCameraPlugin);
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

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut toon_materials: ResMut<Assets<ToonShaderMaterial>>,
    mut standart_materials: ResMut<Assets<StandardMaterial>>,
    mut grass_materials: ResMut<
        Assets<ExtendedMaterial<StandardMaterial, CheckerGrassExtension>>,
    >,
) {
    commands.spawn((
        SceneBundle {
            scene: asset_server.load("levels/World.glb#Scene0"),
            ..Default::default()
        },
        // SpawnBlueprint,
        // HideUntilReady,
        // GameWorldTag,
    ));

    let terrain = Terrain::new_perlin(TerrainPlaneOptions::default(), 64);
    let mesh = meshes.add(terrain.get_mesh().clone());
    commands.insert_resource(terrain.clone());
    commands.spawn((
        terrain,
        ColliderConstructor::TrimeshFromMesh,
        CollisionMargin(1.),
        RigidBody::Static,
        MaterialMeshBundle {
            mesh: mesh,
            material: grass_materials.add(ExtendedMaterial {
                base: StandardMaterial {
                    base_color: Color::srgb(0.4, 0.8, 0.4),
                    normal_map_texture: Some(
                        asset_server.load("textures/checker_normal.png"),
                    ),

                    ..default()
                },
                extension: CheckerGrassExtension {
                    config: CheckerGrassMaterialConfig {
                        plane_size_x: TerrainPlaneOptions::default().width,
                        plane_size_z: TerrainPlaneOptions::default().height,
                        tile_size: 1.,
                        normal_tiles_x: 4,
                    },
                },
            }),
            /* standart_materials.add(StandardMaterial {
                base_color: Color::srgb(0.4, 0.8, 0.4),
                normal_map_texture: Some(asset_server.load("textures/checker_normal.png")),
                ..default()
            }), */
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        //SpawnGrass,
    ));
    commands.spawn((
        DirectionalLightBundle {
            directional_light: DirectionalLight {
                illuminance: 2000.,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, 2.0, 0.0),
                rotation: Quat::from_rotation_x(-PI / 6.)  // tilts down about 30 degrees
                    * Quat::from_rotation_y(-PI / 10.), // rotates towards left/west about 60 degrees
                ..default()
            },
            // The default cascade config is designed to handle large scenes.
            // As this example has a much smaller world, we can tighten the shadow
            // bounds for better visual quality.
            cascade_shadow_config: CascadeShadowConfigBuilder {
                
                maximum_distance: 100.0,
                ..default()
            }
            .into(),
            ..default()
        },
        ToonShaderSun,
    ));
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 100.,
    });

    //commands.spawn(DebugRender::default());
}

/* pub fn animation_control(
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
 */
