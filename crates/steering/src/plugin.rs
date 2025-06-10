use bevy::prelude::*;
use movement::movement::Move;

use crate::{
    boid::BoidPlugin,
    context_map::{move_based_on_context_map, ContextMap},
    simple_steering::SimpleSteeringComponent,
    spatial_hashing::plugin::SpatialHashmapPlugin,
    steering_agent::add_spatial_entity_to_steering_agents,
    steering_behaviour::SteeringBehavior,
};

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct SteeringBehaviourSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct SteeringConversionSet;

pub enum SpatialStructure {
    Hashmap { grid_size: f32 },
    KdTree,
}

pub enum SteeringMode {
    ContextMap,
    SimpleVectors,
    Boids,
}

#[derive(Component)]
pub struct SteeringAgent;

pub struct SteeringPlugin {
    pub spatial_structure: SpatialStructure,
    pub steering_mode: SteeringMode,
}

impl Default for SteeringPlugin {
    fn default() -> Self {
        Self {
            spatial_structure: SpatialStructure::Hashmap { grid_size: 10.0 },
            steering_mode: SteeringMode::ContextMap,
        }
    }
}

impl Plugin for SteeringPlugin {
    fn build(&self, app: &mut App) {
        match self.spatial_structure {
            SpatialStructure::Hashmap { grid_size } => {
                app.add_plugins(SpatialHashmapPlugin { grid_size });
            }
            SpatialStructure::KdTree => {
                todo!("KdTree is not implemented yet")
            }
        }

        // Common systems for all steering modes
        app.add_systems(Update, add_spatial_entity_to_steering_agents);
        app.add_systems(Update, add_steering_behavior_to_agents);

        // Configure specific steering implementation
        match self.steering_mode {
            SteeringMode::ContextMap => {
                app.add_systems(
                    Update,
                    (
                        convert_steering_to_context_map
                            .after(SteeringBehaviourSet)
                            .in_set(SteeringConversionSet),
                        move_based_on_context_map.after(SteeringConversionSet),
                    ),
                );
            }
            SteeringMode::SimpleVectors => {
                app.add_systems(
                    Update,
                    (
                        convert_steering_to_simple_vectors
                            .after(SteeringBehaviourSet)
                            .in_set(SteeringConversionSet),
                        apply_simple_steering.after(SteeringConversionSet),
                    ),
                );
            }
            SteeringMode::Boids => {
                app.add_plugins(BoidPlugin);
            }
        }
    }
}

fn add_steering_behavior_to_agents(
    mut commands: Commands,
    query: Query<Entity, (With<SteeringAgent>, Without<SteeringBehavior>)>,
) {
    for entity in query.iter() {
        commands.entity(entity).insert(SteeringBehavior::new());
    }
}

fn convert_steering_to_context_map(
    mut commands: Commands,
    mut query: Query<
        (Entity, &mut SteeringBehavior),
        (With<SteeringAgent>, Changed<SteeringBehavior>),
    >,
    mut context_maps: Query<&mut ContextMap>,
) {
    for (entity, mut steering) in query.iter_mut() {
        let mut context_map = if let Ok(map) = context_maps.get_mut(entity) {
            map
        } else {
            commands.entity(entity).insert(ContextMap::new());
            continue; // Will process in the next frame after component is added
        };
        context_map.reset();

        // Apply all interests and dangers to the context map
        for (direction, value) in &steering.interests {
            context_map.add_vector_interest(*direction * *value);
        }

        for (direction, value) in &steering.dangers {
            context_map.add_vector_danger(*direction * *value);
        }

        steering.reset();
    }
}

fn convert_steering_to_simple_vectors(
    mut commands: Commands,
    mut query: Query<
        (Entity, &mut SteeringBehavior),
        (With<SteeringAgent>, Changed<SteeringBehavior>),
    >,
    mut simple_steering: Query<&mut SimpleSteeringComponent>,
) {
    for (entity, mut steering) in query.iter_mut() {
        let mut simple = if let Ok(s) = simple_steering.get_mut(entity) {
            s
        } else {
            commands
                .entity(entity)
                .insert(SimpleSteeringComponent::new());
            continue; // Will process in the next frame after component is added
        };
        simple.reset();
        // Apply all interests and dangers
        for (direction, value) in &steering.interests {
            simple.add_vector_interest(*direction * *value);
        }

        for (direction, value) in &steering.dangers {
            simple.add_vector_danger(*direction * *value);
        }
        steering.reset();
    }
}

fn apply_simple_steering(
    mut commands: Commands,
    query: Query<(Entity, &SimpleSteeringComponent)>,
) {
    for (entity, simple_steering) in query.iter() {
        if let Some(movement) = simple_steering.get_final_movement() {
            commands
                .entity(entity)
                .insert(Move(Vec3::new(movement.x, 0.0, movement.y)));
        } else {
            commands.entity(entity).insert(Move(Vec3::ZERO));
        }
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
