use avian3d::prelude::*;
use bevy::math::VectorSpace;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::window::PrimaryWindow;

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
    pub lerp_speed: f32,
    pub offset: Vec3,
}

impl Default for CameraHolder {
    fn default() -> Self {
        CameraHolder {
            mode: CameraMode::FollowEntity {
                target: None,
                weight: 0.35,
            },
            lerp_speed: 5.0,
            offset: Vec3::new(0., 10., -10.),
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
    mut camera: Query<(&CameraHolder, &CameraInput, &mut Transform)>,
    entities: Query<&GlobalTransform>,
    time: Res<Time>,
) {
    for (camera_holder, input, mut holder_transform) in camera.iter_mut() {
        match camera_holder.mode {
            CameraMode::FollowEntity { target, weight } => {
                if let Some(target) = target {
                    if let Ok(target_global_transform) = entities.get(target) {
                        let target_global_translation =
                            target_global_transform.translation();
                        let camera_translation =
                            holder_transform.translation.remove_y();
                        let input_pos = input.pos.remove_y();
                        let target_pos = target_global_translation.remove_y();
                        let weighted_mid_point =
                            target_pos.lerp(input_pos, weight);
                        let camera_translation = camera_translation.lerp(
                            weighted_mid_point,
                            camera_holder.lerp_speed * time.delta_seconds(),
                        );
                        holder_transform.translation = camera_translation;
                    }
                    //println!("Camera translation: {:?}", camera_transform.translation);
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
    mut camera_query: Query<(&CameraInput, Entity, &mut Transform)>,
    windows: Query<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
) {
    let window = windows.single();
    if let Some(cursor_pos) = window.cursor_position() {
        // Convert to normalized device coordinates (-1 to 1)
        let ndc_x = (cursor_pos.x / window.width() * 2.0) - 1.0;
        let ndc_y = (cursor_pos.y / window.height() * 2.0) - 1.0;

        // Define dead zone in the middle of the screen
        let dead_zone = 0.0;
        let move_speed = 100.0;

        for (_input, _entity, mut transform) in camera_query.iter_mut() {
            // Only move if cursor is outside dead zone
            if ndc_x.abs() > dead_zone {
                transform.translation.x += ndc_x ;
            }
            if ndc_y.abs() > dead_zone {
                transform.translation.z += ndc_y ;
            }
        }
    }
}
pub fn spawn_camera_to_follow<'a, 'b>(
    entity_to_follow: Entity,
    mut commands: Commands<'a, 'b>,
) -> (Commands<'a, 'b>, Entity, Entity) {
    println!("Spawning camera");
    let camera_rig = commands
        .spawn(TransformBundle::from_transform(Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            rotation: Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, 0.0),
            scale: Vec3::ONE,
        }))
        .insert(CameraHolder {
            mode: CameraMode::FollowEntity {
                target: Some(entity_to_follow),
                weight: 0.35,
            },
            lerp_speed: 5.0,
            offset: Vec3::new(0., 10., -10.),
        })
        .insert(CameraInput {
            pos: Default::default(),
            zoom: 0.0,
        })
        .id();

    let camera = commands
        .spawn(Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 15.0, 35.0))
                .looking_at(Vec3::ZERO, Vec3::Y),
            projection: get_default_perspective_projection(),
            ..default()
        })
        .insert(MainCamera)
        .set_parent(camera_rig)
        .id();
    return (commands, camera_rig, camera);
}
