use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use movement::follow::Follow;
use movement::follow_rotation::FollowRotation;

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
    pub rotation_x: f32,
    pub min_rotation: f32,
    pub max_rotation: f32,
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
            rotation_x: 0.0,
            min_rotation: -89.0,
            max_rotation: 89.0,
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
    pub rotation_delta_x: f32,
    pub rotation_delta_y: f32,
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
    Projection::Orthographic({
        let mut projection = OrthographicProjection::default_3d();
        projection.near = 0.1;
        projection.far = 200.0;
        projection.viewport_origin = Vec2::new(0.5, 0.5);
        projection.scaling_mode = ScalingMode::FixedVertical {
            viewport_height: 15.,
        };
        projection
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
pub fn update_camera_rotation_input(
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut motion_evr: EventReader<MouseMotion>,
    mut query: Query<&mut CameraInput>,
) {
    for mut input in query.iter_mut() {
        input.rotation_delta_x = 0.0;
        input.rotation_delta_y = 0.0;

        if mouse_input.pressed(MouseButton::Middle) {
            for ev in motion_evr.read() {
                input.rotation_delta_x = -ev.delta.x * 0.1;
                input.rotation_delta_y = -ev.delta.y * 0.1;
            }
        }
    }
}

pub fn rotate(
    mut query: Query<(&CameraInput, &mut CameraHolder, &mut Transform)>,
    target_query: Query<&Transform, Without<CameraHolder>>,
) {
    for (input, mut holder, mut transform) in query.iter_mut() {
        // Get rotation point based on camera mode
        let rotation_point = match holder.mode {
            CameraMode::FollowEntity {
                target: Some(target_entity),
                ..
            } => {
                if let Ok(target_transform) = target_query.get(target_entity) {
                    target_transform.translation
                } else {
                    transform.translation
                }
            }
            _ => transform.translation,
        };

        // Clamp vertical rotation
        let new_rotation_x = (holder.rotation_x + input.rotation_delta_y)
            .clamp(holder.min_rotation, holder.max_rotation);
        let rotation_delta_y = new_rotation_x - holder.rotation_x;
        holder.rotation_x = new_rotation_x;

        // Apply horizontal rotation around Y axis
        let rotation_y =
            Quat::from_axis_angle(Vec3::Y, input.rotation_delta_x.to_radians());
        transform.rotate_around(rotation_point, rotation_y);

        // Apply vertical rotation around local right axis
        let right = transform.right();
        let rotation_x =
            Quat::from_axis_angle(right.into(), rotation_delta_y.to_radians());
        transform.rotate_around(rotation_point, rotation_x);
        transform.look_at(rotation_point, Vec3::Y);
    }
}

pub fn zoom(
    mut rig_query: Query<(
        &CameraInput,
        &mut CameraHolder,
        Entity,
        &mut Transform,
    )>,
    camera_query: Query<
        &mut GlobalTransform,
        (With<MainCamera>, Without<CameraHolder>),
    >,
    target_entity_query: Query<
        &Transform,
        (Without<MainCamera>, Without<CameraInput>),
    >,
) {
    for (input, mut holder, _entity, mut rig_transform) in rig_query.iter_mut()
    {
        if let Ok(_) = camera_query.get_single() {
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

            let new_zoom =
                holder.current_zoom - input.zoom * holder.zoom_percentage_speed;
            holder.current_zoom =
                new_zoom.clamp(holder.min_zoom, holder.max_zoom);

            let zoom_direction =
                (rig_transform.translation - pivot_pos).normalize_or_zero();
            let zoom_delta = zoom_direction * holder.current_zoom;

            rig_transform.translation += zoom_delta - holder.last_zoom_delta;
            holder.last_zoom_delta = zoom_delta;
        }
    }
}

pub fn spawn_camera_to_follow<'a, 'b>(
    entity_to_follow: Entity,
    mut commands: Commands<'a, 'b>,
) -> (Commands<'a, 'b>, Entity, Entity) {
    //println!("Spawning camera");
    let camera_rig = commands
        .spawn(
            Transform::from_translation(Vec3::new(0.0, 5.0, 12.0))
                .looking_at(Vec3::ZERO, Vec3::Y),
        )
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
            rotation_delta_x: 0.0,
            rotation_delta_y: 0.0,
        })
        .insert(
            Follow::default()
                .with_target(entity_to_follow)
                .with_delta_mode()
                .clone(),
        )
        .id();

    let camera = commands
        .spawn((
            Camera3d::default(),
            Transform::from_translation(Vec3::new(0.0, 15.0, 35.0))
                .looking_at(Vec3::ZERO, Vec3::Y),
            get_default_perspective_projection(),
        ))
        .insert(MainCamera)
        .insert(*Follow::default().with_target(camera_rig).lerping(15.0))
        .insert(
            *FollowRotation::default()
                .with_target(camera_rig)
                .lerping(15.0),
        )
        .insert(Msaa::Off)
        .insert(DistanceFog {
            color: Color::linear_rgb(0.529, 0.808, 0.922),
            /* falloff: FogFalloff::Linear {
                start: 200.,
                end: 800.,
            }, */
            falloff: FogFalloff::Exponential {
                density: 0.0005,
            },
        

            ..default()
        })
        .id();
    return (commands, camera_rig, camera);
}
