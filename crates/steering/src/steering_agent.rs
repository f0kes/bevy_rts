use bevy::prelude::*;
use bevy_spatial::{kdtree::KDTree3, AutomaticUpdate, SpatialAccess};

use crate::plugin::SteeringAgent;

#[derive(Component)]
pub struct SpatialEntity;

pub type SteeringAgentTree = KDTree3<SpatialEntity>;

pub fn add_spatial_entity_to_steering_agents(
    mut commands: Commands,
    query: Query<Entity, (With<SteeringAgent>, Without<SpatialEntity>)>,
) {
    for entity in query.iter() {
        commands.entity(entity).insert(SpatialEntity);
    }
}
