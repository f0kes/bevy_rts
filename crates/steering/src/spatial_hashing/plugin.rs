use bevy::prelude::*;

use crate::steering_agent::{SpatialEntity, SpatialStructure};

use super::{spatial_hashmap::SpatialHashmap, spatial_query::CircleQuery};

pub struct SpatialHashmapPlugin {
    pub grid_size: f32,
}
impl Default for SpatialHashmapPlugin {
    fn default() -> Self {
        Self { grid_size: 10.0 }
    }
}

impl Plugin for SpatialHashmapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpatialHashmap::new(self.grid_size));
        app.add_systems(Update, insert_trackers);
        app.add_systems(Update, update_locations);
    }
}

#[derive(Component)]
pub struct SpatialHashmapTracked {
    pub last_position: Vec3,
}

pub fn insert_trackers(
    mut commands: Commands,
    query: Query<Entity, (With<SpatialEntity>, Without<SpatialHashmapTracked>)>,
) {
    for entity in query.iter() {
        commands.entity(entity).insert(SpatialHashmapTracked {
            last_position: Vec3::ZERO,
        });
    }
}

pub fn update_locations(
    mut spatial_hashmap: ResMut<SpatialHashmap>,
    mut query: Query<
        (Entity, &Transform, &mut SpatialHashmapTracked),
        With<SpatialEntity>,
    >,
) {
    for (entity, transform, mut tracker) in query.iter_mut() {
        let position = transform.translation.xz();
        let last_position = tracker.last_position.xz();
        spatial_hashmap.update(entity, last_position, position);
        tracker.last_position = transform.translation;
    }
}
impl SpatialStructure for SpatialHashmap {
    fn within_distance(
        &self,
        position: Vec3,
        distance: f32,
    ) -> Vec<(Vec3, Option<Entity>)> {
        let query = CircleQuery::new(position.xz(), distance);
        let mut result = Vec::new();
        for (entity, entity_position) in self.query(query) {
            result.push((entity_position.extend(0.0), Some(entity)));
        }
        result
    }
}
