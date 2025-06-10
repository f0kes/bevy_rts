use bevy::prelude::*;

use bevy::utils::HashMap;
use bevy::window::PrimaryWindow;
use camera::camera::{spawn_camera_to_follow, MainCamera};
use combat::inventory::{ChosenSlot, Inventory};
use combat::spells::continuos_actions::{ActionMapping, ActiveActions};
use combat::teams::TEAM_PLAYER;
use input_actions::{
    action::InputAction, input_map::InputMap, plugin::InputActionsPlugin,
};
use movement::movement::{
    AccelerationRate, ApplyGravity, CursorPos, DecelerationRate, GlueToGround,
    MaxSpeed, Move, MoveVelocity,
};
use movement::plugin::{
    ConstrainedMovementPlugin, MovementPlugin, MovementPluginConfig,
};
use movement::rotate::{
    RotateInDirectionOfMovement, TiltInDirectionOfMovement,
};
use movement::step_animation::StepAnimation;
use outline::material_replace::ReplaceMaterialKeepTextureMarker;
use outline::toon_shader::{
    default_toon_shader_material, ToonShaderMainCamera,
};
use ownership::player::OwnedBy;
use steering::steering_agent::SpatialEntity;
use world_gen::raycast::{sphere_trace_heightmap, SphereTrace};
use world_gen::terrain::Terrain;

use crate::box_select::unit_selection::{Selections, Selector};
use crate::dudliq::Destination;
use crate::navigation::{NavMeshConstraint, NavigateTo};
use crate::GameState;

pub enum Mode {
    WithModel,
    CameraOnly,
}

pub struct PlayerPlugin;
pub struct PlayerSpawnPlugin {
    pub mode: Mode,
}

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Player>();
        app.insert_resource(InputMap::wasd());
        app.add_plugins(InputActionsPlugin);
        app.add_plugins(
            ConstrainedMovementPlugin::<Move, NavMeshConstraint>::new(
                MovementPluginConfig {
                    default_acceleration: 250.0,
                    default_max_speed: 5.0,
                    default_deceleration: 200.0,
                },
            ),
        );
        app.add_systems(Update, move_player);
        app.add_systems(
            Update,
            update_cursor_pos.run_if(in_state(GameState::Loaded)),
        );
        //app.add_systems(Update, collect_units);
        app.add_systems(Update, update_active_actions_on_player);
        app.add_systems(Update, set_unit_destinations);
    }
}
impl Plugin for PlayerSpawnPlugin {
    fn build(&self, app: &mut App) {
        match self.mode {
            Mode::WithModel => {
                app.add_systems(Startup, spawn_player);
            }
            Mode::CameraOnly => {
                app.add_systems(Startup, spawn_camera_only);
            }
        }
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Player;

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let player_handle = asset_server.load("models/King.glb#Scene0");
    let p_id = commands
        .spawn((
            SceneRoot(player_handle),
            Transform::from_xyz(0., 0.6, 0.),
            Player,
            RotateInDirectionOfMovement::default(),
            TiltInDirectionOfMovement::default(),
            ReplaceMaterialKeepTextureMarker {
                material: default_toon_shader_material(),
            },
            StepAnimation::default(),
            GlueToGround::default(),
            SpatialEntity,
            Inventory::new(10000),
            TEAM_PLAYER,
            ApplyGravity,
            ActiveActions::default(),
            default_action_mapping(),
            ChosenSlot { index: 0 },
        ))
        .insert(MoveVelocity(Vec3::ZERO))
        .id();
    let (mut commands, _rig_id, camera_id) =
        spawn_camera_to_follow(p_id, commands);
    commands.entity(camera_id).insert((
        ToonShaderMainCamera,
        Selector,
        OwnedBy { entity: p_id },
    ));
}
fn spawn_camera_only(mut commands: Commands) {
    let p_id = commands
        .spawn((
            Transform::from_xyz(0., 0., 0.),
            GlueToGround::default(),
            Player,
            MoveVelocity(Vec3::ZERO),
            AccelerationRate(300.0),
            DecelerationRate(1200.0),
            MaxSpeed(30.0),
        ))
        .id();
    let (mut commands, _rig_id, camera_id) =
        spawn_camera_to_follow(p_id, commands);
    commands.entity(camera_id).insert((
        ToonShaderMainCamera,
        Selector,
        OwnedBy { entity: p_id },
    ));
}

pub fn move_player(
    mut commands: Commands,
    action_input: Res<ButtonInput<InputAction>>,
    player_query: Query<Entity, With<Player>>,
    camera_query: Query<(&Transform, &Camera)>,
) {
    let mut main_transform = Transform::default();
    for (transform, camera) in camera_query.iter() {
        if camera.is_active {
            main_transform = transform.clone();
        }
    }
    let mut mv = Vec3::ZERO;
    if action_input.pressed(InputAction::MoveForward) {
        mv.z += 1.0;
    }
    if action_input.pressed(InputAction::MoveBack) {
        mv.z -= 1.0;
    }
    if action_input.pressed(InputAction::MoveLeft) {
        mv.x -= 1.0;
    }
    if action_input.pressed(InputAction::MoveRight) {
        mv.x += 1.0;
    }
    for entity in player_query.iter() {
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
    camera_query: Query<(&GlobalTransform, &Camera), With<MainCamera>>,
    terrain: Res<Terrain>,
) {
    let window = window_query.single();
    let cursor_position = if let Some(pos) = window.cursor_position() {
        pos
    } else {
        return;
    };

    let (camera_global_transform, camera) = camera_query.single();

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
            //println!("Update Cursor: No hit point");
        }
    } else {
        println!("Update Cursor: No ray direction");
    }
}
pub fn update_active_actions_on_player(
    action_input: Res<ButtonInput<InputAction>>,
    mut player_query: Query<&mut ActiveActions, With<Player>>,
) {
    for mut active_actions in player_query.iter_mut() {
        active_actions.0.clear();
    }
    action_input.get_pressed().for_each(|action| {
        for mut active_actions in player_query.iter_mut() {
            active_actions.0.insert(*action);
        }
    });
}
fn set_unit_destinations(
    mut commands: Commands,
    mouse_input: Res<ButtonInput<MouseButton>>,
    owners: Query<(&Selections, &CursorPos)>,
) {
    if mouse_input.just_pressed(MouseButton::Right) {
        for (selections, cursor_pos) in owners.iter() {
            let unit_count = selections.0.len();
            let positions =
                calculate_formation_positions(cursor_pos.0, unit_count);

            // Pair each unit with a unique destination
            for (&entity, position) in selections.0.iter().zip(positions.iter())
            {
                //commands.entity(entity).insert(Destination(*position));
                commands.entity(entity).insert(NavigateTo(*position));
            }
        }
    }
}

fn calculate_formation_positions(center: Vec3, count: usize) -> Vec<Vec3> {
    let mut positions = Vec::with_capacity(count);

    let rows = (count as f32).sqrt().ceil() as i32;
    let cols = (count as f32 / rows as f32).ceil() as i32;

    let spacing = 2.0; // Units between positions
    let offset = Vec3::new(
        (cols as f32 - 1.0) * spacing * -0.5,
        0.0,
        (rows as f32 - 1.0) * spacing * -0.5,
    );

    // Generate grid positions
    for i in 0..count {
        let row = (i as i32) / cols;
        let col = (i as i32) % cols;

        positions.push(
            center
                + offset
                + Vec3::new(col as f32 * spacing, 0.0, row as f32 * spacing),
        );
    }

    positions
}
pub fn default_action_mapping() -> ActionMapping {
    let action_mapping = ActionMapping(HashMap::new());
    /* action_mapping.0.insert(
        InputAction::Collect,
        Action::VacuumSpell(VacuumSpell {
            range: 20.,
            width: 2.,
            pull_force: 2.,
            eat_range: 1.,
        }),
    );
    action_mapping.0.insert(
        InputAction::UseItem,
        Action::SummonSpell(SummonSpell {
            summon_interval: 0.2,
            last_summon_time: 0.,
            summon_velocity: 10.,
        }),
    ); */
    action_mapping
}

/* pub fn collect_units(
    mut commands: Commands,
    action_input: Res<ButtonInput<InputAction>>,
    player_query: Query<Entity, With<Player>>,
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
    if action_input.pressed(InputAction::Collect) && !vacuum_exists {
        commands.spawn(ActionBundle::vacuum_spell(
            VacuumSpell {
                range: 20.,
                width: 2.,
                pull_force: 2.,
                eat_range: 1.,
            },
            player,
        ));
    } else if !action_input.pressed(InputAction::Collect) {
        for (entity, _) in vacuum_query.iter() {
            commands.entity(entity).despawn();
        }
    }
} */

/* pub fn spawn_units(
    mut commands: Commands,
    action_input: Res<ButtonInput<InputAction>>,
    player_query: Query<Entity, With<Player>>,
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
    if action_input.pressed(InputAction::Spawn) && !vacuum_exists {
        commands.spawn(ActionBundle::summon_spell(
            SummonSpell {
                summon_interval: 1.,
                last_summon_time: 0.,
                summon_velocity: 10.,
            },
            player,
        ));
    } else if !action_input.pressed(InputAction::Spawn) {
        for (entity, _) in vacuum_query.iter() {
            commands.entity(entity).despawn();
        }
    }
} */
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
