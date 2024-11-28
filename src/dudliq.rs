use avian3d::prelude::{Collider, RigidBody};
use bevy::prelude::*;
use movement::{
    kinematic_character_controller::KinematicCharacterControllerBundle,
    movement::GlueToGround,
    rotate::{RotateInDirectionOfMovement, TiltInDirectionOfMovement},
    step_animation::StepAnimation,
};
use outline::{
    material_replace::ReplaceMaterialKeepTextureMarker,
    shader_material::OutlineMaterial,
    toon_shader::default_toon_shader_material,
};
pub fn spawn_dudliq(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<OutlineMaterial>>,
) {
    let player_handle = asset_server.load("models/dudliq.glb#Scene0");
    let u_id = commands
        .spawn((
            SceneBundle {
                scene: player_handle,
                transform: Transform::from_xyz(4., 0.45, 4.),
                ..Default::default()
            },
            KinematicCharacterControllerBundle::default(),
            Collider::sphere(0.47),
            RigidBody::Kinematic,
            RotateInDirectionOfMovement::default(),
            TiltInDirectionOfMovement::default(),
            ReplaceMaterialKeepTextureMarker {
                material: default_toon_shader_material(),
            },
            StepAnimation::default(),
            GlueToGround::default(),
        ))
        .id();
}
