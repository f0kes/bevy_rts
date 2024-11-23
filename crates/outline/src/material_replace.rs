use bevy::prelude::*;

#[derive(Component, Debug, Reflect)]
pub struct ReplaceMaterialMarker<T: Material> {
    pub material: T,
}

pub fn replace_standart_materials<T: Material>(
    mut commands: Commands,
    query: Query<(Entity, &ReplaceMaterialMarker<T>)>,
    with_children: Query<&Children>,
    with_standart_material: Query<&Handle<StandardMaterial>>,
    mut materials: ResMut<Assets<T>>,
) {
    for (entity, marker) in query.iter() {
        let mut candidates: Vec<Entity> = Vec::new();
        candidates.push(entity);
        
        // Recursively collect all descendants
        let mut index = 0;
        while index < candidates.len() {
            if let Ok(children) = with_children.get(candidates[index]) {
                candidates.extend(children.iter());
            }
            index += 1;
        }

        // Replace materials for all entities that have StandardMaterial
        for &candidate in candidates.iter() {
            if with_standart_material.contains(candidate) {
                // Remove the StandardMaterial component
                commands.entity(candidate).remove::<Handle<StandardMaterial>>();
                
                // Add the new material
                let new_material = materials.add(marker.material.clone());
                commands.entity(candidate).insert(new_material);
            }
        }
    }
}