use crate::material_replace::{
    replace_standart_materials, 
    replace_standart_materials_keep_texture,
    TexturableMaterial
};
use bevy::prelude::*;
use std::hash::Hash;

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