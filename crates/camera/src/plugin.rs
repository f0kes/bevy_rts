use avian3d::prelude::*;
use bevy::prelude::*;

use crate::camera::{update_camera_input, update_follow_camera, CameraHolder, CameraMode};

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct CameraSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
enum CameraSystemSet {
    Input,
    Follow,
}

pub struct SmoothCameraPlugin;

impl Plugin for SmoothCameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .configure_sets(
                PhysicsSchedule,
                CameraSet.after(PhysicsStepSet::Last)
            )
            .configure_sets(
                PhysicsSchedule,
                (
                    CameraSystemSet::Input,
                    CameraSystemSet::Follow,
                )
                    .chain()
                    .in_set(CameraSet)
            )
            .add_systems(
                PhysicsSchedule,
                (
                    update_camera_input.in_set(CameraSystemSet::Input),
                    update_follow_camera.in_set(CameraSystemSet::Follow),
                )
            )
            .register_type::<CameraHolder>()
            .register_type::<CameraMode>();
    }
}