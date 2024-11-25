use bevy::prelude::*;

#[derive(Component, Clone, Copy)]
pub struct Follow {
    pub target: Option<Entity>,
    pub tween_mode: TweenMode,
    pub follow_mode: FollowMode,
}
#[derive(Clone, Copy)]
pub enum TweenMode {
    Lerping(LerpData),
    Instant,
}
#[derive(Clone, Copy)]
pub enum FollowMode {
    Global,
    Delta(DeltaData),
}

#[derive(Clone, Copy)]
pub struct LerpData {
    pub lerp_per_sec: f32,
}
#[derive(Clone, Copy)]
pub struct DeltaData {
    pub previos_target_pos: Vec3,
}
impl Default for Follow {
    fn default() -> Self {
        Self {
            target: None,
            tween_mode: TweenMode::Instant,
            follow_mode: FollowMode::Global,
        }
    }
}
impl Follow {
    pub fn with_target(&mut self, target: Entity) -> &mut Self {
        self.target = Some(target);
        self
    }
    pub fn lerping(&mut self, lerp_per_sec: f32) -> &mut Self {
        self.tween_mode = TweenMode::Lerping(LerpData { lerp_per_sec });
        self
    }
    pub fn with_delta_mode(&mut self) -> &mut Self {
        self.follow_mode = FollowMode::Delta(DeltaData {
            previos_target_pos: Vec3::ZERO,
        });
        self
    }
}
pub fn follow_target(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Follow)>,
    target_query: Query<&GlobalTransform>,
) {
    for (mut transform, mut follow) in query.iter_mut() {
        if let Some(target) = follow.target {
            if let Ok(target_transform) = target_query.get(target) {
                let target_pos = match follow.follow_mode {
                    FollowMode::Global => target_transform.translation(),
                    FollowMode::Delta(ref mut delta_data) => {
                        let delta = target_transform.translation()
                            - delta_data.previos_target_pos;
                        delta_data.previos_target_pos =
                            target_transform.translation();
                        transform.translation + delta
                    }
                };
                match follow.tween_mode {
                    TweenMode::Lerping(lerp_data) => {
                        transform.translation = transform.translation.lerp(
                            target_pos,
                            lerp_data.lerp_per_sec * time.delta_seconds(),
                        );
                    }
                    TweenMode::Instant => {
                        transform.translation = target_pos;
                    }
                }
            }
        }
    }
}
