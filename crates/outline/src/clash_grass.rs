use bevy::{
    pbr::{ExtendedMaterial, MaterialExtension},
    prelude::*,
    reflect::TypePath,
    render::render_resource::*,
};
#[derive(ShaderType, Debug, Clone, Reflect)]

pub struct CheckerGrassMaterialConfig {
    pub plane_size_x: f32,
    pub plane_size_z: f32,
    pub tile_size: f32,
    pub normal_tiles_x: u32,
}

#[derive(Asset, AsBindGroup, Debug, Clone, Reflect)]
pub struct CheckerGrassExtension {
    // We need to ensure that the bindings of the base material and the extension do not conflict,
    // so we start from binding slot 100, leaving slots 0-99 for the base material.
    #[uniform(100)]
    pub config: CheckerGrassMaterialConfig,
}

impl MaterialExtension for CheckerGrassExtension {
    fn prepass_fragment_shader() -> ShaderRef {
        "shaders/grass.wgsl".into()
    }
    fn fragment_shader() -> ShaderRef {
        "shaders/grass.wgsl".into()
    }

    // fn deferred_fragment_shader() -> ShaderRef {
    //     "shaders/dissolve_material_prepass.wgsl".into()
    // }
}

pub struct CheckerGrassMaterialPlugin;
impl Plugin for CheckerGrassMaterialPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<CheckerGrassExtension>();
        app.register_type::<ExtendedMaterial<StandardMaterial, CheckerGrassExtension>>();
        app.add_plugins(MaterialPlugin::<
            ExtendedMaterial<StandardMaterial, CheckerGrassExtension>,
        >::default());
    }
}
