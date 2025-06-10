// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use avian3d::prelude::{
    ColliderConstructor, CollisionMargin, PhysicsDebugPlugin, PhysicsGizmos,
    RigidBody,
};
use avian3d::PhysicsPlugins;
use bevy::asset::AssetMetaCheck;
use bevy::scene::ScenePlugin;
use bevy_game::box_select::draw_rectangle::{
    DrawRectanglePlugin, RectangleTestPlugin,
};
use bevy_game::box_select::mouse_drag::{
    MouseDragPlugin, MouseDragRectangleTestPlugin,
};
use bevy_game::box_select::unit_selection::{
    UnitSelectionPlugin, UnitSelectionTestPlugin,
};
use bevy_game::loading::LoadingPlugin;
use bevy_game::navigation::NavigationPlugin;
use bevy_game::particles::plugin::ParticlesPlugin;
use bevy_game::scene::DefaultScenePlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::pbr::{
    CascadeShadowConfig, CascadeShadowConfigBuilder, ExtendedMaterial,
};
use bevy::prelude::*;

use bevy::window::PresentMode;
use bevy::DefaultPlugins;

use bevy_game::dudliq::{spawn_a_lot_of_dudliqs, DudliqPlugin};
use bevy_game::player::{Mode, PlayerPlugin};

use bevy_hanabi::HanabiPlugin;
use camera::plugin::SmoothCameraPlugin;
use combat::inventory::plugin::InventoryPlugin;
use combat::spells::plugin::SpellsPlugin;
use combat::units::plugin::UnitsPlugin;
use easy_model_load::plugin::EasyModelLoadPlugin;
use meta_components::plugin::MetaComponentsPlugin;
use outline::clash_grass::{CheckerGrassExtension, CheckerGrassMaterialConfig};
use outline::plugin::MyMaterialsPlugin;
use outline::toon_shader::ToonShaderSun;
use steering::plugin::{SpatialStructure, SteeringMode, SteeringPlugin};
use vleue_navigator::VleueNavigatorPlugin;
use world_gen::terrain::{Terrain, TerrainLike, TerrainPlaneOptions};

use std::f32::consts::PI;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Dude;

fn main() {
    let mut app = App::new();
    let gismo_config = GizmoConfig {
        enabled: false,
        ..default()
    };

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

    app.register_type::<Dude>();

    app.add_plugins(LoadingPlugin);
    app.add_plugins(PlayerPlugin);
    app.add_plugins(DefaultScenePlugin);
    app.add_plugins(MyMaterialsPlugin);
    app.add_plugins(FrameTimeDiagnosticsPlugin::default());
    app.add_plugins(LogDiagnosticsPlugin::default());

    app.insert_gizmo_config(
        PhysicsGizmos {
            //aabb_color: Some(Color::linear_rgb(0., 0., 1.)),
            collider_color: Some(Color::linear_rgb(0., 1., 0.)),
            ..default()
        },
        gismo_config,
    );
    app.add_plugins(PhysicsPlugins::default());
    app.add_plugins(PhysicsDebugPlugin::default());
    app.add_plugins(SmoothCameraPlugin);
    app.add_plugins(SteeringPlugin {
        spatial_structure: SpatialStructure::Hashmap { grid_size: 5.0 },
        steering_mode: SteeringMode::Boids,
    });
    /* app.add_plugins(SteeringPlugin {
        spatial_structure: SpatialStructure::KdTree,
    });*/
    app.add_plugins(UnitsPlugin);
    app.add_plugins(DudliqPlugin);
    app.add_plugins(SpellsPlugin);
    app.add_plugins(InventoryPlugin);
    app.add_plugins(MetaComponentsPlugin);
    app.add_plugins(WorldInspectorPlugin::new());

    app.add_plugins(HanabiPlugin);
    app.add_plugins(ParticlesPlugin);
    app.add_plugins(DrawRectanglePlugin);
    app.add_plugins(MouseDragPlugin);
    app.add_plugins(MouseDragRectangleTestPlugin);
    app.add_plugins(UnitSelectionPlugin);
    app.add_plugins(UnitSelectionTestPlugin);
    app.add_plugins(EasyModelLoadPlugin);
    app.add_plugins(NavigationPlugin { debug: true });
    app.insert_resource(ClearColor(Color::linear_rgb(0.529, 0.808, 0.922)));
    app.run();
}

/* fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,

    mut grass_materials: ResMut<
        Assets<ExtendedMaterial<StandardMaterial, CheckerGrassExtension>>,
    >,
) {
    //commands.spawn((SceneRoot(asset_server.load("levels/World.glb#Scene0")),));
    let terrain_height = 2000.;
    let terrain_width = 2000.;
    let terrain = Terrain::new_perlin(
        TerrainPlaneOptions {
            noise_scale: 0.01,
            height: terrain_height,
            width: terrain_width,
            ..Default::default()
        },
        64,
    );
    let mesh = meshes.add(terrain.get_mesh().clone());
    commands.insert_resource(terrain.clone());
    commands.spawn((
        terrain,
        ColliderConstructor::TrimeshFromMesh,
        CollisionMargin(1.),
        RigidBody::Static,
        Mesh3d(mesh),
        MeshMaterial3d(grass_materials.add(create_grass_material(
            asset_server,
            terrain_height,
            terrain_width,
        ))), //SpawnGrass,
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
    commands.spawn((
        DirectionalLight {
            illuminance: 2000.,
            shadows_enabled: true,
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 6.)  // tilts down about 30 degrees
                    * Quat::from_rotation_y(-PI / 10.), // rotates towards left/west about 60 degrees
            ..default()
        },
        CascadeShadowConfig::from(CascadeShadowConfigBuilder {
            maximum_distance: 1000.0,
            ..default()
        }),
        ToonShaderSun,
    ));
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 100.,
    });
}
pub fn create_grass_material(
    asset_server: Res<AssetServer>,
    width: f32,
    height: f32,
) -> ExtendedMaterial<StandardMaterial, CheckerGrassExtension> {
    ExtendedMaterial {
        base: StandardMaterial {
            base_color: Color::srgb(0.4, 0.8, 0.4),
            normal_map_texture: Some(
                asset_server.load("textures/checker_normal.png"),
            ),
            ..default()
        },
        extension: CheckerGrassExtension {
            config: CheckerGrassMaterialConfig {
                plane_size_x: width,
                plane_size_z: height,
                tile_size: 1.,
                normal_tiles_x: 4,
            },
        },
    }
} */
