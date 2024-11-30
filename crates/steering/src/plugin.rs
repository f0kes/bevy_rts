use bevy::prelude::*;
use bevy_spatial::AutomaticUpdate;

use crate::{
    context_map::{
        add_context_map_to_steering_agents, move_based_on_context_map,
    },
    spatial_hashing::plugin::SpatialHashmapPlugin,
    steering_agent::{add_spatial_entity_to_steering_agents, SpatialEntity},
};

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct SteeringBehaviourSet;

pub enum SpatialStructure {
    Hashmap { grid_size: f32 },
    KdTree,
}

#[derive(Component)]
pub struct SteeringAgent;
pub struct SteeringPlugin {
    pub spatial_structure: SpatialStructure,
}
impl Plugin for SteeringPlugin {
    fn build(&self, app: &mut App) {
        match self.spatial_structure {
            SpatialStructure::Hashmap { grid_size } => {
                app.add_plugins(SpatialHashmapPlugin { grid_size });
            }
            SpatialStructure::KdTree => {
                app.add_plugins(AutomaticUpdate::<SpatialEntity>::new());
            }
        }

        app.add_systems(Update, add_context_map_to_steering_agents);
        app.add_systems(
            Update,
            move_based_on_context_map.after(SteeringBehaviourSet),
        );
        app.add_systems(Update, add_spatial_entity_to_steering_agents);
    }
}
pub trait SteeringBehavioursAppExt {
    fn add_behaviours<M>(
        &mut self,
        systems: impl IntoSystemConfigs<M>,
    ) -> &mut Self;
}
impl SteeringBehavioursAppExt for App {
    fn add_behaviours<M>(
        &mut self,
        systems: impl IntoSystemConfigs<M>,
    ) -> &mut Self {
        self.add_systems(Update, systems.in_set(SteeringBehaviourSet));
        self
    }
}
