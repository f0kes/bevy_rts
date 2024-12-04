use avian3d::prelude::*;

use bevy::prelude::*;

use bevy::window::PrimaryWindow;
use camera::camera::{spawn_camera_to_follow, MainCamera};
use combat::inventory::Inventory;
use combat::spells::spell::{ActionBundle, ActionData};
use combat::spells::vacuum::VacuumSpell;
use combat::teams::{Team, TEAM_PLAYER};
use input_actions::{
    action::Action, input_map::InputMap, plugin::InputActionsPlugin,
};
use movement::collide_and_slide::CollideAndSlide;
use movement::kinematic_character_controller::{
    KinematicCharacterController, KinematicCharacterControllerBundle,
};
use movement::movement::{
    ApplyGravity, CursorPos, GlueToGround, Move, MoveInput,
};
use movement::plugin::{MovementPlugin, MovementPluginConfig};
use movement::rotate::{
    RotateInDirectionOfMovement, TiltInDirectionOfMovement,
};
use movement::step_animation::StepAnimation;
use outline::material_replace::{
    ReplaceMaterialKeepTextureMarker, ReplaceMaterialMarker,
};
use outline::plugin::ToonShaderPlugin;
use outline::shader_material::OutlineMaterial;
use outline::toon_shader::{
    default_toon_shader_material, ToonShaderMainCamera, ToonShaderMaterial,
};
use steering::steering_agent::SpatialEntity;
use world_gen::raycast::{sphere_trace_heightmap, SphereTrace};
use world_gen::terrain::{self, Terrain};

pub struct PlayerPlugin;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
        app.register_type::<Player>();
        app.insert_resource(InputMap::wasd());
        app.add_plugins(InputActionsPlugin);
        app.add_plugins(MovementPlugin::<Move>::new(MovementPluginConfig {
            default_acceleration: 35.0,
            default_max_speed: 5.0,
            default_deceleration: 200.0,
        }));
        app.add_systems(Update, move_player);
        app.add_systems(Update, update_cursor_pos);
        app.add_systems(Update, collect_units);
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Player;

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<OutlineMaterial>>,
) {
    let player_handle = asset_server.load("models/King.glb#Scene0");
    let p_id = commands
        .spawn((
            SceneBundle {
                scene: player_handle,
                transform: Transform::from_xyz(0., 0.6, 0.),
                ..Default::default()
            },
            Player,
            KinematicCharacterControllerBundle::default(),
            //Collider::sphere(0.47),
            //RigidBody::Kinematic,
            RotateInDirectionOfMovement::default(),
            TiltInDirectionOfMovement::default(),
            ReplaceMaterialKeepTextureMarker {
                material: default_toon_shader_material(),
            },
            StepAnimation::default(),
            GlueToGround::default(),
            SpatialEntity,
            Inventory::new(10),
            TEAM_PLAYER,
            ApplyGravity,
        ))
        .id();
    let (mut commands, _rig_id, camera_id) =
        spawn_camera_to_follow(p_id, commands);
    commands.entity(camera_id).insert(ToonShaderMainCamera);
}

pub fn move_player(
    mut commands: Commands,
    action_input: Res<ButtonInput<Action>>,
    player_query: Query<(Entity), With<Player>>,
    camera_query: Query<(&Transform, &Camera)>,
) {
    let mut main_transform = Transform::default();
    for (transform, camera) in camera_query.iter() {
        if camera.is_active {
            main_transform = transform.clone();
        }
    }
    let mut mv = Vec3::ZERO;
    if action_input.pressed(Action::MoveForward) {
        mv.z += 1.0;
    }
    if action_input.pressed(Action::MoveBack) {
        mv.z -= 1.0;
    }
    if action_input.pressed(Action::MoveLeft) {
        mv.x -= 1.0;
    }
    if action_input.pressed(Action::MoveRight) {
        mv.x += 1.0;
    }
    for (entity) in player_query.iter() {
        let movement = mv;
        let movement = Vec3::new(movement.x, 0.0, movement.z);
        let forward = main_transform.forward();
        let right = main_transform.right();

        let forward = Vec3::new(forward.x, 0.0, forward.z).normalize();
        let right = Vec3::new(right.x, 0.0, right.z).normalize();

        let transformed_movement = right * movement.x + forward * movement.z;

        commands.entity(entity).insert(Move(transformed_movement));
    }
}
pub fn update_cursor_pos(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    player_query: Query<Entity, With<Player>>,
    camera_query: Query<
        (&Transform, &GlobalTransform, &Camera),
        With<MainCamera>,
    >,
    terrain: Res<Terrain>,
) {
    let window = window_query.single();
    let cursor_position = if let Some(pos) = window.cursor_position() {
        pos
    } else {
        return;
    };

    let (camera_transform, camera_global_transform, camera) =
        camera_query.single();

    // Convert cursor to ndc
    let ndc = Vec2::new(
        (2.0 * cursor_position.x) / window.width() - 1.0,
        1.0 - (2.0 * cursor_position.y) / window.height(),
    );

    if let Some(ray_direction) = camera
        .ndc_to_world(camera_global_transform, Vec3::new(ndc.x, ndc.y, 1.0))
        .map(|world_pos| {
            (world_pos - camera_global_transform.translation()).normalize()
        })
    {
        let camera_pos = camera_global_transform.translation();
        let hit_point_opt = sphere_trace_heightmap(
            SphereTrace::new(camera_pos, ray_direction),
            &*terrain,
        );
        if let Some(hit_point) = hit_point_opt {
            for entity in player_query.iter() {
                commands.entity(entity).insert(CursorPos(hit_point));
            }
        } else {
            println!("Update Cursor: No hit point");
        }
    } else {
        println!("Update Cursor: No ray direction");
    }
}

pub fn collect_units(
    mut commands: Commands,
    action_input: Res<ButtonInput<Action>>,
    player_query: Query<(Entity), With<Player>>,
    vacuum_query: Query<(Entity, &ActionData), With<VacuumSpell>>,
) {
    let player = match player_query.get_single() {
        Ok(player) => player,
        Err(_) => return,
    };
    let mut vacuum_exists = false;
    for (_, action_data) in vacuum_query.iter() {
        if action_data.actor == player {
            vacuum_exists = true;
        }
    }
    if action_input.pressed(Action::Collect) && !vacuum_exists {
        commands.spawn(ActionBundle::vacuum_spell(
            VacuumSpell {
                range: 20.,
                width: 2.,
                pull_force: 2.,
            },
            player,
        ));
    } else if !action_input.pressed(Action::Collect) {
        for (entity, _) in vacuum_query.iter() {
            commands.entity(entity).despawn();
        }
    }
}
/* #[derive(Component)]
pub struct CursorFollower;

pub fn test_cursor_setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.spawn((
        CursorFollower,
        PbrBundle {
            mesh: meshes.add(Mesh::from(Cuboid::from_length(1.0))),
            material: materials.add(Color::WHITE),
            ..Default::default()
        },
    ));
}
pub fn test_cursor_update(
    mut follower_query: Query<&mut Transform, With<CursorFollower>>,
    cursor_query: Query<&CursorPos>,
) {
    for cursor_pos in cursor_query.iter() {
        for mut follower_transform in follower_query.iter_mut() {
            follower_transform.translation = cursor_pos.0;
        }
    }
}
 */
