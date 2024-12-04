use bevy::prelude::*;

use combat::units::unit::{get_unit_data, Unit, UnitName};
use steering::{
    context_map::ContextMap,
    plugin::SteeringBehavioursAppExt,
    spatial_hashing::spatial_hashmap::SpatialHashmap,
    steering_agent::{
        get_nearby_unit_positions, SpatialEntity, SpatialStructure,
    },
};

pub struct DudliqPlugin;
impl Plugin for DudliqPlugin {
    fn build(&self, app: &mut App) {
        app.add_behaviours(avoid_others::<SpatialHashmap>);
        //app.add_behaviours(avoid_others::<SteeringAgentTree>);
    }
}

pub fn spawn_dudliq(mut commands: Commands) {
    let u_id = commands.spawn((get_unit_data(UnitName::Dudliq),)).id();
}
pub fn spawn_a_lot_of_dudliqs(mut commands: Commands) {
    for n in 0..1000 {
        let angle = (n as f32 / 100.0) * std::f32::consts::TAU;
        let radius = (rand::random::<f32>() * 10.0).max(1.0);
        let x = angle.cos() * radius;
        let z = angle.sin() * radius;

        commands.spawn((
            get_unit_data(UnitName::Dudliq),
            Transform::from_translation(Vec3::new(x, 0.0, z)),
        ));
    }
}
pub fn avoid_others<T: Resource + SpatialStructure>(
    tree: Res<T>,
    mut query: Query<(Entity, &Transform, &mut ContextMap, &Unit)>,
    mut others_query: Query<
        (Entity, &Transform),
        (Without<Unit>, With<SpatialEntity>),
    >,
) {
    let mut entities_and_positions: Vec<(Entity, Vec3)> = query
        .iter()
        .map(|(entity, transform, _, _)| (entity, transform.translation))
        .collect();
    let others_entities_and_positions: Vec<(Entity, Vec3)> = others_query
        .iter()
        .map(|(entity, transform)| (entity, transform.translation))
        .collect();

    entities_and_positions.extend(others_entities_and_positions);
    for (entity, transform, mut context_map, _unit) in query.iter_mut() {
        let nearby_positions = get_nearby_unit_positions(
            tree.as_ref(),
            entity,
            transform.translation,
            2.0,
            &entities_and_positions,
            10,
        );
        for other_pos in nearby_positions {
            let direction = other_pos - transform.translation;
            context_map.add_vector_interest(
                -direction.xz() * 1.0 / direction.length(),
            );
        }
    }
}
