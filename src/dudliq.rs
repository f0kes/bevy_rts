use bevy::prelude::*;
use combat::units::unit::{get_unit_data, Unit, UnitName};
use meta_components::propagating::PropagateAppExt;
use outline::change_color::Recolor;
use steering::{
    boid::BoidBundle,
    context_map::ContextMap,
    plugin::SteeringBehavioursAppExt,
    spatial_hashing::spatial_hashmap::SpatialHashmap,
    steering_agent::{
        get_nearby_unit_positions, SpatialEntity, SpatialStructure,
    },
    steering_behaviour::SteeringBehavior,
};

use crate::{
    box_select::unit_selection::Selectable,
    particles::on_hit::SpawnOnHitParticles,
};

pub struct DudliqPlugin;
impl Plugin for DudliqPlugin {
    fn build(&self, app: &mut App) {
        app.add_behaviours(avoid_others::<SpatialHashmap>);
        app.add_behaviours(go_towards);
        app.propagate::<Recolor>();
        //app.add_behaviours(avoid_others::<SteeringAgentTree>);
    }
}

pub fn spawn_dudliq(mut commands: Commands) {
    let _u_id = commands
        .spawn((
            get_unit_data(UnitName::Dudliq),
            Selectable,
            BoidBundle::default(),
        ))
        .id();
}
pub fn spawn_a_lot_of_dudliqs(mut commands: Commands) {
    for position in get_random_positions_in_circle(Vec2::ZERO, 10.0, 200) {
        let mut entity_commands = commands.spawn((
            get_unit_data(UnitName::Dudliq),
            Transform::from_translation(Vec3::new(position.x, 0.0, position.y)),
            Selectable,
            BoidBundle::default(),
        ));
    }
}
pub fn get_random_positions_in_circle(
    center: Vec2,
    radius: f32,
    count: i32,
) -> Vec<Vec2> {
    let mut positions = Vec::new();
    for n in 0..count {
        let angle = (n as f32 / count as f32) * std::f32::consts::TAU;
        let exact_radius = rand::random::<f32>() * radius;
        let x = angle.cos() * exact_radius;
        let z = angle.sin() * exact_radius;
        positions.push(Vec2::new(center.x + x, center.y + z));
    }
    positions
}
pub fn avoid_others<T: Resource + SpatialStructure>(
    tree: Res<T>,
    mut query: Query<(Entity, &Transform, &mut SteeringBehavior, &Unit)>,
    others_query: Query<
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
#[derive(Component)]
pub struct Destination {
    pub position: Vec3,
    pub tolerance: f32,
}
pub fn go_towards(
    mut query: Query<(&Transform, &mut SteeringBehavior, &Unit, &Destination)>,
) {
    for (transform, mut context_map, _unit, destination) in query.iter_mut() {
        let direction = destination.position - transform.translation;

        if direction.length_squared()
            > destination.tolerance * destination.tolerance
        {
            context_map.add_vector_interest(direction.xz().normalize());
        }
    }
}
