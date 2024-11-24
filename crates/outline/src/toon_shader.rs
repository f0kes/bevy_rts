use bevy::{
    asset::load_internal_asset,
    pbr::{MaterialPipeline, MaterialPipelineKey},
    prelude::*,
    render::render_resource::{
        AsBindGroup, AsBindGroupShaderType, RenderPipelineDescriptor,
        ShaderRef, ShaderType, SpecializedMeshPipelineError,
    },
};

use crate::{material_replace::TexturableMaterial, plugin::TexturableMaterialPlugin};
pub const TOON_SHADER_HANDLE: Handle<Shader> =
    Handle::weak_from_u128(11079857277321826659);



#[derive(Asset, AsBindGroup, Reflect, Debug, Clone, Default)]
#[uniform(0, ToonShaderMaterialUniform)]
#[reflect(Default, Debug)]
pub struct ToonShaderMaterial {
    pub color: Color,
    pub sun_dir: Vec3,
    pub sun_color: Color,
    pub camera_pos: Vec3,
    pub ambient_color: Color,
    #[texture(1)]
    #[sampler(2)]
    pub base_color_texture: Option<Handle<Image>>,
}
impl TexturableMaterial for ToonShaderMaterial {
    fn set_texture(&mut self, texture: Handle<Image>) {
        self.base_color_texture = Some(texture);
    }
}

impl Material for ToonShaderMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/toon_shader.wgsl".into()
    }
    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        _descriptor: &mut RenderPipelineDescriptor,
        _layout: &bevy::render::mesh::MeshVertexBufferLayoutRef,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        Ok(())
    }
}

impl AsBindGroupShaderType<ToonShaderMaterialUniform> for ToonShaderMaterial {
    fn as_bind_group_shader_type(
        &self,
        _images: &bevy::render::render_asset::RenderAssets<
            bevy::render::texture::GpuImage,
        >,
    ) -> ToonShaderMaterialUniform {
        ToonShaderMaterialUniform {
            color: self.color.into(),
            sun_dir: self.sun_dir,
            sun_color: self.sun_color.into(),
            camera_pos: self.camera_pos,
            ambient_color: self.ambient_color.into(),
        }
    }
}

#[derive(Clone, Default, ShaderType)]
pub struct ToonShaderMaterialUniform {
    pub color: LinearRgba,
    pub sun_dir: Vec3,
    pub sun_color: LinearRgba,
    pub camera_pos: Vec3,
    pub ambient_color: LinearRgba,
}

// #[derive(Eq, PartialEq, Hash, Clone)]
// pub struct ToonShaderMaterialKey {}

// impl From<&ToonShaderMaterial> for ToonShaderMaterialKey {
//     fn from(_material: &ToonShaderMaterial) -> Self {
//         Self {}
//     }
// }

#[derive(Component)]
pub struct ToonShaderMainCamera;

#[derive(Component)]
pub struct ToonShaderSun;

pub fn update_toon_shader(
    main_cam: Query<&Transform, With<ToonShaderMainCamera>>,
    sun: Query<(&Transform, &DirectionalLight), With<ToonShaderSun>>,
    ambient_light: Option<Res<AmbientLight>>,
    mut toon_materials: ResMut<Assets<ToonShaderMaterial>>,
) {
    for (_, toon_mat) in toon_materials.iter_mut() {
        if let Ok(cam_t) = main_cam.get_single() {
            toon_mat.camera_pos = cam_t.translation;
        }
        if let Ok((sun_t, dir_light)) = sun.get_single() {
            toon_mat.sun_dir = *sun_t.back();
            toon_mat.sun_color = dir_light.color;
        }
        if let Some(light) = &ambient_light {
            toon_mat.ambient_color = light.color;
        }
    }
}
