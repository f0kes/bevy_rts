use bevy::prelude::*;

use crate::teams::Team;
pub trait ExtractEntity {
    fn extract_entity(&self) -> Entity;
}

// Add more implementations as needed for other types that contain Entity

pub trait TeamFilterExt: Iterator
where
    Self::Item: ExtractEntity,
{
    fn same_team(
        self,
        teams: &Query<(Entity, &Team)>,
        target_team: &Team,
    ) -> impl Iterator<Item = Self::Item>
    where
        Self: Sized,
    {
        let same_team_entities: Vec<Entity> = teams
            .iter()
            .filter(|(_, team)| **team == *target_team)
            .map(|(entity, _)| entity)
            .collect();

        self.filter(move |item| {
            same_team_entities.contains(&item.extract_entity())
        })
    }

    fn other_team(
        self,
        teams: &Query<(Entity, &Team)>,
        target_team: &Team,
    ) -> impl Iterator<Item = Self::Item>
    where
        Self: Sized,
    {
        let different_team_entities: Vec<Entity> = teams
            .iter()
            .filter(|(_, team)| **team != *target_team)
            .map(|(entity, _)| entity)
            .collect();

        self.filter(move |item| {
            different_team_entities.contains(&item.extract_entity())
        })
    }
}

// Implement for any iterator whose Item implements ExtractEntity
impl<T> TeamFilterExt for T
where
    T: Iterator,
    T::Item: ExtractEntity,
{
}

impl ExtractEntity for Entity {
    fn extract_entity(&self) -> Entity {
        *self
    }
}

impl ExtractEntity for (Entity, Vec3) {
    fn extract_entity(&self) -> Entity {
        self.0
    }
}
