/* 
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use avian3d::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_follow_camera)
            .add_systems(Update, update_camera_input)
            .add_systems(Update, client_update_camera_target)
            .register_type::<CameraHolder>()
            .register_type::<CameraMode>();
    }
}

#[derive(Reflect)]
pub enum CameraMode {
    FollowEntity { target: Option<Entity>, weight: f32 },
    Free,
}

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

pub fn client_update_camera_target(
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
}

pub fn update_camera_input(
    mut commands: Commands,
    mut camera_query: Query<(&Camera, Entity, &Transform, &GlobalTransform)>,
    mut camera_input_query: Query<(&CameraInput, Entity)>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    rapier_context: Res<RapierContext>,
) {
    let mut cursor_pos = Vec2::ZERO;
    for window in q_windows.iter() {
        if let Some(cursor_position) = window.cursor_position() {
            cursor_pos = cursor_position;
        } else {
            return;
        }
    }
    //println!("Cursor viewport: {:?}", cursor_viewport);
    for (
        camera_component,
        _camera_entity,
        _camera_transform,
        camera_global_tr,
    ) in camera_query.iter_mut()
    {
        if let Some(ray) =
            camera_component.viewport_to_world(camera_global_tr, cursor_pos)
        {
            let query_filter = QueryFilter::default();

            if let Some((_entity, toi)) = rapier_context.cast_ray(
                ray.origin,
                ray.direction.into(),
                f32::MAX,
                false,
                query_filter,
            ) {
                let hit_pos = ray.origin + ray.direction * toi;

                for (_camera_input, input_entity) in
                    camera_input_query.iter_mut()
                {
                    let camera_input = CameraInput {
                        pos: hit_pos,
                        zoom: 1.0,
                    };
                    commands.entity(input_entity).insert(camera_input);
                }
            }
        }
    }
}
 */