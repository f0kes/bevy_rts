use bevy::prelude::*;

use crate::follow::{LerpData, TweenMode};

#[derive(Component, Clone, Copy)]
pub struct FollowRotation {
    pub target: Option<Entity>,
    pub tween_mode: TweenMode,
    pub follow_mode: FollowRotationMode,
}

#[derive(Clone, Copy)]
pub enum FollowRotationMode {
    Global,
    Delta(DeltaRotationData),
}

#[derive(Clone, Copy)]
pub struct DeltaRotationData {
    pub previous_target_rot: Quat,
}

impl Default for FollowRotation {
    fn default() -> Self {
        Self {
            target: None,
            tween_mode: TweenMode::Instant,
            follow_mode: FollowRotationMode::Global,
        }
    }
}

impl FollowRotation {
    pub fn with_target(&mut self, target: Entity) -> &mut Self {
        self.target = Some(target);
        self
    }

    pub fn lerping(&mut self, lerp_per_sec: f32) -> &mut Self {
        self.tween_mode = TweenMode::Lerping(LerpData { lerp_per_sec });
        self
    }

    pub fn with_delta_mode(&mut self) -> &mut Self {
        self.follow_mode = FollowRotationMode::Delta(DeltaRotationData {
            previous_target_rot: Quat::IDENTITY,
        });
        self
    }

    pub fn target(target: Entity) -> Self {
        Self::default().with_target(target).clone()
    }
}

pub fn follow_rotation(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut FollowRotation)>,
    target_query: Query<&GlobalTransform>,
) {
    for (mut transform, mut follow) in query.iter_mut() {
        if let Some(target) = follow.target {
            if let Ok(target_transform) = target_query.get(target) {
                let target_rot = match follow.follow_mode {
                    FollowRotationMode::Global => target_transform.rotation(),
                    FollowRotationMode::Delta(ref mut delta_data) => {
                        // Calculate the relative rotation between previous and current target rotation
                        let delta_rotation = target_transform.rotation()
                            * delta_data.previous_target_rot.inverse();

                        // Update previous rotation for next frame
                        delta_data.previous_target_rot =
                            target_transform.rotation();

                        // Apply the delta rotation to current transform
                        delta_rotation * transform.rotation
                    }
                };

                match follow.tween_mode {
                    TweenMode::Lerping(lerp_data) => {
                        transform.rotation = transform.rotation.slerp(
                            target_rot,
                            lerp_data.lerp_per_sec * time.delta_secs(),
                        );
                    }
                    TweenMode::Instant => {
                        transform.rotation = target_rot;
                    }
                }
            }
        }
    }
}
