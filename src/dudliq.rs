use avian3d::prelude::{Collider, RigidBody};
use bevy::prelude::*;
use bevy_spatial::{kdtree::KDTree3, AutomaticUpdate, SpatialAccess};
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
use steering::{
    context_map::ContextMap,
    plugin::{SteeringAgent, SteeringBehavioursAppExt},
    steering_agent::{SpatialEntity, SteeringAgentTree},
};

#[derive(Component)]
pub struct Unit;

pub struct DudliqPlugin;
impl Plugin for DudliqPlugin {
    fn build(&self, app: &mut App) {
        app.add_behaviours(avoid_others);
    }
}

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
            SteeringAgent,
            Unit,
        ))
        .id();
}
pub fn spawn_a_lot_of_dudliqs(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<OutlineMaterial>>,
) {
    for n in 0..500 {
        let angle = (n as f32 / 100.0) * std::f32::consts::TAU;
        let radius = (rand::random::<f32>() * 10.0).max(1.0);
        let x = angle.cos() * radius;
        let z = angle.sin() * radius;

        let player_handle = asset_server.load("models/dudliq.glb#Scene0");
        commands.spawn((
            SceneBundle {
                scene: player_handle,
                transform: Transform::from_xyz(x, 0.3, z),
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
            SteeringAgent,
            Unit,
        ));
    }
}
pub fn avoid_others(
    tree: Res<SteeringAgentTree>,
    mut query: Query<(Entity, &Transform, &mut ContextMap, &Unit)>,
    mut others_query: Query<
        (Entity, &Transform),
        (Without<Unit>, With<SpatialEntity>),
    >,
) {
    let mut entities_and_positions: Vec<(Entity, Vec3)> = query
        .iter()
        .map(|(entity, transform, _, _)| (entity, transform.translation))
        .collect();
    let others_entities_and_positions: Vec<(Entity, Vec3)> = others_query
        .iter()
        .map(|(entity, transform)| (entity, transform.translation))
        .collect();

    entities_and_positions.extend(others_entities_and_positions);
    for (entity, transform, mut context_map, _unit) in query.iter_mut() {
        tree.within_distance(transform.translation, 1.5)
            .into_iter()
            .filter_map(|(_, other_entity_opt)| other_entity_opt)
            .filter(|&other_entity| entity != other_entity)
            .filter_map(|other_entity| {
                entities_and_positions
                    .iter()
                    .find(|(e, _)| *e == other_entity)
                    .map(|(_, pos)| *pos)
            })
            .for_each(|other_pos| {
                let direction = other_pos - transform.translation;
                //context_map.add_vector_danger(direction.xz());
                context_map.add_vector_interest(
                    -direction.xz() * 1.0 / direction.length(),
                );
            });
    }
}
