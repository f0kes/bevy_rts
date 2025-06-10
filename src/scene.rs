use std::f32::consts::{FRAC_PI_2, PI};

use avian3d::prelude::{
    Collider, ColliderConstructor, CollisionMargin, RigidBody,
};
use bevy::{
    pbr::{CascadeShadowConfig, CascadeShadowConfigBuilder, ExtendedMaterial},
    prelude::*,
};
use easy_model_load::{cube::SpawnCube, plugin::LoadModel};
use movement::{bob::Bob, movement::GlueToGround, revolve::Revolve};
use outline::{
    clash_grass::{CheckerGrassExtension, CheckerGrassMaterialConfig},
    toon_shader::ToonShaderSun,
};
use steering::steering_agent::SpatialEntity;
use vleue_navigator::prelude::*;
use world_gen::terrain::{Terrain, TerrainLike, TerrainPlaneOptions};

use crate::{
    dudliq::spawn_a_lot_of_dudliqs, navigation::Obstacle,
    player::PlayerSpawnPlugin, GameState,
};
pub struct DefaultScenePlugin;
impl Plugin for DefaultScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PlayerSpawnPlugin {
            mode: crate::player::Mode::CameraOnly,
        });
        app.add_systems(Startup, spawn_terrain);
        app.add_systems(OnEnter(GameState::Loaded), spawn_light);
        app.add_systems(OnEnter(GameState::Loaded), spawn_a_lot_of_dudliqs);
        app.add_systems(OnEnter(GameState::Loaded), spawn_logs);
        app.add_systems(OnEnter(GameState::Loaded), spawn_house);
    }
}

pub fn spawn_terrain(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,

    mut grass_materials: ResMut<
        Assets<ExtendedMaterial<StandardMaterial, CheckerGrassExtension>>,
    >,
) {
    let terrain_height = 2000.;
    let terrain_width = 2000.;
    let terrain = Terrain::new_perlin(
        TerrainPlaneOptions {
            noise_scale: 0.1, //0.01
            height: terrain_height,
            width: terrain_width,
            ..Default::default()
        },
        64,
    );
    let mesh = meshes.add(terrain.get_mesh().clone());
    let navmesh = vleue_navigator::NavMesh::from_bevy_mesh(terrain.get_mesh());

    commands.insert_resource(terrain.clone());
    commands.spawn((
        terrain,
        ColliderConstructor::TrimeshFromMesh,
        CollisionMargin(0.),
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
        NavMeshSettings {
            fixed: Triangulation::from_mesh(navmesh.get().as_ref(), 0),
            build_timeout: Some(5.0),
            upward_shift: 0.1, // Adjust based on your needs
            merge_steps: 2,
            agent_radius: 0.5,

            ..default()
        },
        //Transform::from_xyz(0.0, 0.0, 0.0),
        NavMeshUpdateMode::Direct,
        //NavMeshDebug(Color::srgba(1.0, 0.0, 0.0, 0.8)),
        Transform::from_rotation(Quat::from_rotation_x(FRAC_PI_2)),

    ));
}
pub fn spawn_light(mut commands: Commands) {
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

pub fn spawn_logs(mut commands: Commands) {
    for i in 0..10 {
        commands.spawn((
            Transform::from_translation(Vec3::new(i as f32, 0.7, 0.0))
                .with_scale(Vec3::splat(0.5))
                .with_rotation(Quat::from_rotation_x(PI / 8.)),
            GlobalTransform::default(),
            LoadModel {
                path: "models/log.glb#Scene0".to_string(),
            },
            GlueToGround::default(),
            SpatialEntity,
            Bob::default(),
            Revolve::default(),
        ));
    }
}
pub fn spawn_house(mut commands: Commands) {
    commands.spawn((
        Transform::from_translation(Vec3::new(15.0, 0.5, 15.0)).with_scale(
            Vec3 {
                x: 5.,
                y: 1.5,
                z: 5.,
            },
        ),
        SpawnCube {
            color: Color::srgb(0.8, 0.2, 0.2),
        },
        GlueToGround::default(),
        Obstacle,
        RigidBody::Static,
        Collider::cuboid(1.0, 1.0, 1.0),
        Name::new("House"),
    ));
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
}
