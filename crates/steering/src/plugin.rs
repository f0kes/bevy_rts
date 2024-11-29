use bevy::prelude::*;
use bevy_spatial::AutomaticUpdate;

use crate::{
    context_map::{
        add_context_map_to_steering_agents, move_based_on_context_map,
    },
    steering_agent::{add_spatial_entity_to_steering_agents, SpatialEntity},
};

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct SteeringBehaviourSet;

#[derive(Component)]
pub struct SteeringAgent;
pub struct SteeringPlugin;
impl Plugin for SteeringPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AutomaticUpdate::<SpatialEntity>::new());
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
