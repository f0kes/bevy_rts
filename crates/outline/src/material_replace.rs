use bevy::prelude::*;

pub trait TexturableMaterial: Material {
    fn set_texture(&mut self, texture: Handle<Image>);
}

#[derive(Component, Debug, Reflect)]
pub struct ReplaceMaterialMarker<T: Material> {
    pub material: T,
}

#[derive(Component, Debug, Reflect)]
pub struct ReplaceMaterialKeepTextureMarker<T: TexturableMaterial> {
    pub material: T,
}

fn collect_descendants(
    entity: Entity,
    with_children: &Query<&Children>,
) -> Vec<Entity> {
    let mut candidates = vec![entity];
    let mut index = 0;
    while index < candidates.len() {
        if let Ok(children) = with_children.get(candidates[index]) {
            candidates.extend(children.iter());
        }
        index += 1;
    }
    candidates
}

fn replace_material<T: Material>(
    commands: &mut Commands,
    entity: Entity,
    new_material: Handle<T>,
) {
    commands
        .entity(entity)
        .remove::<Handle<StandardMaterial>>()
        .insert(new_material);
}

pub fn replace_standart_materials<T: Material>(
    mut commands: Commands,
    query: Query<(Entity, &ReplaceMaterialMarker<T>)>,
    with_children: Query<&Children>,
    with_standart_material: Query<&Handle<StandardMaterial>>,
    mut materials: ResMut<Assets<T>>,
) {
    for (entity, marker) in query.iter() {
        let candidates = collect_descendants(entity, &with_children);

        for &candidate in candidates
            .iter()
            .filter(|&&candidate| with_standart_material.contains(candidate))
        {
            let new_material = materials.add(marker.material.clone());
            replace_material(&mut commands, candidate, new_material);
        }
    }
}

pub fn replace_standart_materials_keep_texture<T: TexturableMaterial>(
    mut commands: Commands,
    query: Query<(Entity, &ReplaceMaterialKeepTextureMarker<T>)>,
    with_children: Query<&Children>,
    with_standart_material: Query<&Handle<StandardMaterial>>,
    mut materials: ResMut<Assets<T>>,
    standart_materials: Res<Assets<StandardMaterial>>,
) {
    for (entity, marker) in query.iter() {
        let candidates = collect_descendants(entity, &with_children);

        for &candidate in candidates.iter() {
            if with_standart_material.contains(candidate) {
                let mut new_material = marker.material.clone();

                with_standart_material
                    .get(candidate)
                    .ok()
                    .and_then(|handle| standart_materials.get(handle))
                    .and_then(|mat| mat.base_color_texture.clone())
                    .map(|texture| new_material.set_texture(texture));

                let new_material = materials.add(new_material);
                replace_material(&mut commands, candidate, new_material);
            }
        }
    }
}
