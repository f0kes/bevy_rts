use avian3d::prelude::*;

use bevy::prelude::*;

use camera::camera::spawn_camera_to_follow;
use input_actions::{
    action::Action, input_map::InputMap, plugin::InputActionsPlugin,
};
use movement::collide_and_slide::CollideAndSlide;
use movement::kinematic_character_controller::{
    KinematicCharacterController, KinematicCharacterControllerBundle,
};
use movement::movement::{ApplyGravity, MoveInput};
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
                transform: Transform::from_xyz(0., 10., 1.),
                ..Default::default()
            },
            Player,
            KinematicCharacterControllerBundle::default(),
            Collider::sphere(0.47),
            SweptCcd::default(),
            RigidBody::Kinematic,
            LockedAxes::new()
                .lock_rotation_z()
                .lock_rotation_x()
                .lock_rotation_y(),
            RotateInDirectionOfMovement::default(),
            TiltInDirectionOfMovement::default(),
            ReplaceMaterialKeepTextureMarker {
                material: default_toon_shader_material(),
            },
            StepAnimation::default(),
            ApplyGravity,

            //CollideAndSlide::default(),
        ))
        .id();
     let (mut commands, _rig_id, camera_id) =
        spawn_camera_to_follow(p_id, commands); 
    commands.entity(camera_id).insert(ToonShaderMainCamera);
}
#[derive(Component)]
pub struct Move(Vec3);
impl MoveInput for Move {
    fn direction(&self) -> Vec3 {
        self.0
    }
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
