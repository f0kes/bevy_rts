use avian3d::prelude::*;
use bevy::input::mouse::MouseWheel;
use bevy::math::VectorSpace;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::window::PrimaryWindow;

use crate::follow::{Follow, TweenMode};

#[derive(Reflect)]
pub enum CameraMode {
    FollowEntity { target: Option<Entity>, weight: f32 },
    Free,
}

#[derive(Component)]
pub struct MainCamera;

#[derive(Component, Reflect)]
pub struct CameraHolder {
    pub mode: CameraMode,

    pub offset: Vec3,
    pub zoom_percentage_speed: f32,
    pub min_zoom: f32,
    pub max_zoom: f32,
    pub last_zoom_delta: Vec3,
    pub current_zoom: f32,
}

impl Default for CameraHolder {
    fn default() -> Self {
        CameraHolder {
            mode: CameraMode::FollowEntity {
                target: None,
                weight: 0.35,
            },

            offset: Vec3::new(0., 10., -10.),
            zoom_percentage_speed: 6.5,
            min_zoom: 0.0,
            max_zoom: 100.0,
            last_zoom_delta: Vec3::ZERO,
            current_zoom: 13.0,
        }
    }
}

impl CameraHolder {
    pub fn new_with_target(target: Entity) -> Self {
        CameraHolder {
            mode: CameraMode::FollowEntity {
                target: Some(target),
                weight: 0.35,
            },
            ..default()
        }
    }
}

#[derive(Bundle)]
pub struct CameraRigBundle {
    pub camera_input: CameraInput,
    pub camera_holder: CameraHolder,
}

#[derive(Component)]
pub struct CameraInput {
    pub pos: Vec3,
    pub zoom: f32,
}

pub fn update_follow_camera(
    mut commands: Commands,
    mut camera: Query<(Entity, &CameraHolder, &CameraInput, &mut Transform)>,
    entities: Query<&GlobalTransform>,
    time: Res<Time>,
    mut gizmos: Gizmos,
) {
    for (rig_entity, camera_holder, input, mut holder_transform) in
        camera.iter_mut()
    {
        match camera_holder.mode {
            CameraMode::FollowEntity { target, weight } => {
                if let Some(target) = target {
                    if let Ok(_target_global_transform) = entities.get(target) {
                        /*  commands.entity(rig_entity).insert(
                            Follow::default()
                                .with_target(target)
                                .with_delta_mode()
                                .clone(),
                        ); */
                        /* gizmos.cuboid(
                            *holder_transform,
                            Color::srgb(1.0, 0.0, 0.0),
                        ); */
                    }
                }
            }
            CameraMode::Free => {}
        }
    }
}

pub trait RemoveY {
    fn remove_y(&self) -> Self;
}

impl RemoveY for Vec3 {
    fn remove_y(&self) -> Self {
        Vec3::new(self.x, 0., self.z)
    }
}

pub trait RemoveZ {
    fn remove_z(&self) -> Self;
}
impl RemoveZ for Vec3 {
    fn remove_z(&self) -> Self {
        Vec3::new(self.x, self.y, 0.)
    }
}

pub fn get_default_orthographic_projection() -> Projection {
    Projection::Orthographic(OrthographicProjection {
        near: 0.1,
        far: 200.,
        viewport_origin: Vec2 { x: 0.5, y: 0.5 },
        scaling_mode: ScalingMode::FixedVertical(15.0),
        ..default()
    })
}
pub fn get_default_perspective_projection() -> Projection {
    Projection::Perspective(PerspectiveProjection {
        fov: std::f32::consts::PI / 6.0,
        near: 0.1,
        far: 200.0,
        aspect_ratio: 1.0,
        ..default()
    })
}

/* pub fn client_update_camera_target(
    mut commands: Commands,
    mut controlled_player_query: Query<Entity, With<ControlledPlayer>>,
    unit_query: Query<(Entity, &ControlledBy)>,
    mut camera_query: Query<(&CameraHolder, Entity)>,
) {
    for player_entity in controlled_player_query.iter_mut() {
        for (unit_entity, controlled_by) in unit_query.iter() {
            if controlled_by.0 == player_entity {
                for (_camera_holder, holder_entity) in camera_query.iter_mut() {
                    commands
                        .entity(holder_entity)
                        .insert(CameraHolder::new_with_target(unit_entity));
                }
            }
        }
    }
} */

pub fn update_camera_input(
    //TODO: this should be a system that updates the camera input component, not a component itself
    mut camera_query: Query<(&mut CameraInput, Entity, &mut Transform)>,

    mut evr_scroll: EventReader<MouseWheel>,
) {
    for (mut input, _entity, mut _transform) in camera_query.iter_mut() {
        input.zoom = 0.0;
        for scroll in evr_scroll.read() {
            input.zoom = scroll.y;
        }
    }
}

pub fn zoom(
    mut rig_query: Query<(
        &CameraInput,
        &mut CameraHolder,
        Entity,
        &mut Transform,
    )>,
    mut camera_query: Query<
        (&mut GlobalTransform),
        (With<MainCamera>, Without<CameraHolder>),
    >,
    mut target_entity_query: Query<
        &Transform,
        (Without<MainCamera>, Without<CameraInput>),
    >,
    time: Res<Time>,
) {
    for (input, mut holder, _entity, mut rig_transform) in rig_query.iter_mut()
    {
        if let Ok(mut camera_transform) = camera_query.get_single_mut() {
            let pivot_pos = match holder.mode {
                CameraMode::FollowEntity { target, .. } => {
                    if let Some(target_entity) = target {
                        if let Ok(target_transform) =
                            target_entity_query.get(target_entity)
                        {
                            target_transform.translation
                        } else {
                            rig_transform.translation
                        }
                    } else {
                        rig_transform.translation
                    }
                }
                CameraMode::Free => rig_transform.translation,
            };
            let target_pos = rig_transform.translation;
            let new_zoom =
                holder.current_zoom - input.zoom * holder.zoom_percentage_speed;
            holder.current_zoom =
                new_zoom.clamp(holder.min_zoom, holder.max_zoom);

            let zoom_direction =
                (rig_transform.translation - pivot_pos).normalize_or_zero();
            let zoom_delta =
                 zoom_direction * holder.current_zoom;

            rig_transform.translation +=
                zoom_delta - holder.last_zoom_delta;
            holder.last_zoom_delta = zoom_delta;

          
        }
    }
}

pub fn spawn_camera_to_follow<'a, 'b>(
    entity_to_follow: Entity,
    mut commands: Commands<'a, 'b>,
) -> (Commands<'a, 'b>, Entity, Entity) {
    println!("Spawning camera");
    let camera_rig = commands
        .spawn(TransformBundle::from_transform(
            Transform::from_translation(Vec3::new(0.0, 5.0, 12.0))
                .looking_at(Vec3::ZERO, Vec3::Y),
        ))
        .insert(CameraHolder {
            mode: CameraMode::FollowEntity {
                target: Some(entity_to_follow),
                weight: 0.35,
            },
            ..default()
        })
        .insert(CameraInput {
            pos: Default::default(),
            zoom: 10.0,
        })
        .insert(
            Follow::default()
                .with_target(entity_to_follow)
                .with_delta_mode()
                .clone(),
        )
        .id();

    let camera = commands
        .spawn(Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 15.0, 35.0))
                .looking_at(Vec3::ZERO, Vec3::Y),
            projection: get_default_perspective_projection(),
            ..default()
        })
        .insert(MainCamera)
        .insert(
            Follow::default()
                .with_target(camera_rig)
                .lerping(1.0)
                .clone(),
        )
        .id();
    return (commands, camera_rig, camera);
}
