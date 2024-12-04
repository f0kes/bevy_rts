use bevy::prelude::*;
use bevy_spatial::{kdtree::KDTree3, AutomaticUpdate, SpatialAccess};

use crate::plugin::SteeringAgent;

#[derive(Component)]
pub struct SpatialEntity;

pub type SteeringAgentTree = KDTree3<SpatialEntity>;

pub trait SpatialStructure {
    //fn update(&mut self, entity: Entity, position: Vec3);
    //fn remove(&mut self, entity: Entity);
    fn within_distance(
        &self,
        position: Vec3,
        distance: f32,
    ) -> Vec<(Vec3, Option<Entity>)>;
}
pub fn add_spatial_entity_to_steering_agents(
    mut commands: Commands,
    query: Query<Entity, (With<SteeringAgent>, Without<SpatialEntity>)>,
) {
    for entity in query.iter() {
        commands.entity(entity).insert(SpatialEntity);
    }
}
impl SpatialStructure for SteeringAgentTree {
    /* fn update(&mut self, entity: Entity, position: Vec3) {}

    fn remove(&mut self, entity: Entity) {
        self.remove(entity);
    } */

    fn within_distance(
        &self,
        position: Vec3,
        distance: f32,
    ) -> Vec<(Vec3, Option<Entity>)> {
        <SteeringAgentTree as SpatialAccess>::within_distance(
            self, position, distance,
        )
    }
}
pub fn get_nearby_unit_positions<'a, T: SpatialStructure>(
    space: &T,
    entity: Entity,
    position: Vec3,
    distance: f32,
    entities_and_positions: &'a [(Entity, Vec3)],
    max_count: usize,
) -> impl Iterator<Item = Vec3> + 'a {
    space
        .within_distance(position, distance)
        .into_iter()
        .filter_map(|(_, other_entity_opt)| other_entity_opt)
        .filter(move |&other_entity| entity != other_entity)
        .filter_map(move |other_entity| {
            entities_and_positions
                .iter()
                .find(|(e, _)| *e == other_entity)
                .map(|(_, pos)| *pos)
        })
        .take(max_count)
}
pub fn get_nearby_unit_entities_and_positions<'a, T: SpatialStructure>(
    space: &T,
    entity: Entity,
    position: Vec3,
    distance: f32,
    entities_and_positions: &'a [(Entity, Vec3)],
    max_count: usize,
) -> impl Iterator<Item = (Entity, Vec3)> + 'a {
    space
        .within_distance(position, distance)
        .into_iter()
        .filter_map(|(_, other_entity_opt)| other_entity_opt)
        .filter(move |&other_entity| entity != other_entity)
        .filter_map(move |other_entity| {
            entities_and_positions
                .iter()
                .find(|(e, _)| *e == other_entity)
                .map(|(e, pos)| (other_entity, *pos))
        })
        .take(max_count)
}
