use bevy::prelude::*;

use crate::{
    shader_material::OutlineMaterial, toon_shader::ToonShaderMaterial,
};

pub struct RecolorPlugin;

impl Plugin for RecolorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, recolor_models::<StandardMaterial>);
        app.add_systems(Update, recolor_models::<ToonShaderMaterial>);
        app.add_systems(Update, recolor_models::<OutlineMaterial>);
    }
}

pub trait ColorReplacableMaterial: Material {
    fn set_color(&mut self, color: Color);
}
impl ColorReplacableMaterial for StandardMaterial {
    fn set_color(&mut self, color: Color) {
        self.base_color = color;
    }
}
impl ColorReplacableMaterial for ToonShaderMaterial {
    fn set_color(&mut self, color: Color) {
        self.color = color;
    }
}
impl ColorReplacableMaterial for OutlineMaterial {
    fn set_color(&mut self, color: Color) {
        self.color = color.into();
    }
}

#[derive(Component, Clone, Copy)]
pub struct Recolor {
    pub into_color: Color,
}
pub fn recolor_models<T: ColorReplacableMaterial>(
    mut commands: Commands,
    query: Query<(Entity, &Recolor, &MeshMaterial3d<T>), Added<Recolor>>,
    mut materials: ResMut<Assets<T>>,
) {
    for (entity, recolor, material_handle) in query.iter() {
        if let Some(material) = materials.get_mut(material_handle) {
            material.set_color(recolor.into_color);
            commands.entity(entity).remove::<Recolor>();
        }
    }
}
