use crate::{
    clash_grass::CheckerGrassMaterialPlugin,
    material_replace::{
        replace_standart_materials, replace_standart_materials_keep_texture,
        TexturableMaterial,
    },
    shader_material::OutlineMaterial,
    toon_shader::{update_toon_shader, ToonShaderMaterial},
};
use bevy::prelude::*;
use std::hash::Hash;

pub struct MyMaterialsPlugin;
impl Plugin for MyMaterialsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TexturableMaterialPlugin::<OutlineMaterial>::default());
        app.add_plugins(ToonShaderPlugin);
        app.add_plugins(CheckerGrassMaterialPlugin);
    }
}

pub struct CustomMaterialPlugin<T: Material> {
    _marker: std::marker::PhantomData<T>,
}

impl<T: Material> Default for CustomMaterialPlugin<T> {
    fn default() -> Self {
        CustomMaterialPlugin {
            _marker: std::marker::PhantomData,
        }
    }
}

impl<M: Material> Plugin for CustomMaterialPlugin<M>
where
    M::Data: PartialEq + Eq + Hash + Clone,
{
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<M>::default())
            .add_systems(Update, replace_standart_materials::<M>);
    }
}

pub struct TexturableMaterialPlugin<T: TexturableMaterial> {
    _marker: std::marker::PhantomData<T>,
}

impl<T: TexturableMaterial> Default for TexturableMaterialPlugin<T> {
    fn default() -> Self {
        TexturableMaterialPlugin {
            _marker: std::marker::PhantomData,
        }
    }
}

impl<M: TexturableMaterial> Plugin for TexturableMaterialPlugin<M>
where
    M::Data: PartialEq + Eq + Hash + Clone,
{
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<M>::default())
            .add_systems(Update, replace_standart_materials_keep_texture::<M>);
    }
}

#[derive(Default)]
pub struct ToonShaderPlugin;

impl Plugin for ToonShaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            TexturableMaterialPlugin::<ToonShaderMaterial>::default(),
        )
        .add_systems(Update, update_toon_shader);
    }
}
