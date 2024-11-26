//! A shader that renders a mesh multiple times in one draw call.

use bevy::{
    core_pipeline::core_3d::Transparent3d,
    ecs::{
        query::QueryItem,
        system::{lifetimeless::*, SystemParamItem},
    },
    pbr::{
        MeshPipeline, MeshPipelineKey, RenderMeshInstances, SetMeshBindGroup,
        SetMeshViewBindGroup,
    },
    prelude::*,
    render::{
        extract_component::{ExtractComponent, ExtractComponentPlugin},
        mesh::{GpuBufferInfo, GpuMesh, MeshVertexBufferLayoutRef},
        render_asset::{RenderAssetUsages, RenderAssets},
        render_phase::{
            AddRenderCommand, DrawFunctions, PhaseItem, PhaseItemExtraIndex,
            RenderCommand, RenderCommandResult, SetItemPipeline,
            TrackedRenderPass, ViewSortedRenderPhases,
        },
        render_resource::*,
        renderer::RenderDevice,
        view::{ExtractedView, NoFrustumCulling},
        Render, RenderApp, RenderSet,
    },
};
use bytemuck::{Pod, Zeroable};

pub trait Heightmap {
    fn height(&self, x: f32, z: f32) -> f32;
}
pub trait WithBounds {
    fn bounds(&self) -> (f32, f32, f32, f32);
}
#[derive(Component)]
pub struct SpawnGrass;

fn spawn_grass<T: Heightmap + WithBounds + Component>(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    query: Query<(Entity, &T, &Transform), With<SpawnGrass>>,
) {
    /*  let grass_scene: Handle<Scene> =
           asset_server.load("models/grass.glb#Scene0");
    */
    // Create a simple grass blade mesh (a rectangular plane)
    for (entity, heightmap_with_bounds, transform) in &query {
        let mut grass_mesh = Mesh::new(
            bevy::render::render_resource::PrimitiveTopology::TriangleList,
            RenderAssetUsages::RENDER_WORLD,
        );
        let height = 1.0;
        let width = 0.1;

        let vertices = vec![
            // Base points
            [-(width / 2.0), 0.0, 0.0], // 0: Bottom left
            [width / 2.0, 0.0, 0.0],    // 1: Bottom right
            // Middle control points
            [-(width * 0.4), height * 0.33, 0.0], // 2: Left middle
            [width * 0.4, height * 0.33, 0.0],    // 3: Right middle
            // Upper control points
            [-(width * 0.25), height * 0.66, 0.0], // 4: Left upper
            [width * 0.25, height * 0.66, 0.0],    // 5: Right upper
            // Tip
            [0.0, height * 1.0, 0.0], // 6: Top tip
        ];

        let indices = vec![
            0, 1, 3, // Bottom triangle
            0, 3, 2, // Lower middle triangle
            2, 3, 5, // Upper middle triangle
            2, 5, 6, // Top triangle
        ];

        let normals = vec![[0.0, 0.0, 1.0]; 7];

        let uvs = vec![
            [0.0, 0.0],  // Bottom left
            [1.0, 0.0],  // Bottom right
            [0.1, 0.33], // Left middle
            [0.9, 0.33], // Right middle
            [0.2, 0.66], // Left upper
            [0.8, 0.66], // Right upper
            [0.5, 1.0],  // Top tip
        ];

        grass_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
        grass_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        grass_mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        grass_mesh.insert_indices(bevy::render::mesh::Indices::U32(indices));

        // Create grass instances in a grid pattern
        // Get the bounds of the heightmap
        let (min_x, max_x, min_z, max_z) = heightmap_with_bounds.bounds();
        let bounds_width = max_x - min_x;
        let bounds_depth = max_z - min_z;

        // Create patches
        let num_patches = 50; // Number of dense patches
        let patch_radius = bounds_width * 0.1; // Size of each patch
        let patches: Vec<(Vec2, f32)> = (0..num_patches)
            .map(|_| {
                let center = Vec2::new(
                    min_x + fastrand::f32() * bounds_width,
                    min_z + fastrand::f32() * bounds_depth,
                );
                let density = 0.5 + fastrand::f32() * 0.5; // Random density multiplier
                (center, density)
            })
            .collect();

        // Create grass instances with varying density based on patches
        let num_grass = 250000;
        let grass_instances: Vec<InstanceData> = (0..num_grass)
            .map(|_| {
                // Generate random positions within bounds
                let random_x = min_x + fastrand::f32() * bounds_width;
                let random_z = min_z + fastrand::f32() * bounds_depth;
                let pos = Vec2::new(random_x, random_z);

                // Calculate density based on nearest patch
                let density_factor = patches.iter().map(|(patch_center, density)| {
                    let distance = pos.distance(*patch_center);
                    if distance < patch_radius {
                        let falloff = 1.0 - (distance / patch_radius).powi(2);
                        falloff * density
                    } else {
                        0.0
                    }
                }).max_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap_or(0.1); // Base density outside patches

                // Skip this instance based on density
                if fastrand::f32() > density_factor {
                    return None;
                }

                // Get the height at this position
                let height = heightmap_with_bounds.height(random_x, random_z);

                // Random variations for natural look
                let random_scale = 0.8 + fastrand::f32() * 0.4;
                let random_rotation = fastrand::f32() * std::f32::consts::TAU;

                // Create rotation quaternion around Y axis
                let rotation = Quat::from_rotation_y(random_rotation);

                // Apply the transform's position offset
                let world_position = transform.translation + Vec3::new(random_x, height, random_z);

                Some(InstanceData {
                    position: world_position,
                    scale: random_scale,
                    
                    color: [
                        0.2 + fastrand::f32() * 0.1,
                        0.7 + fastrand::f32() * 0.3,
                        0.3,
                        1.0,
                    ],
                })
            })
            .filter_map(|instance| instance)
            .collect();

        commands.spawn((
            meshes.add(grass_mesh),
            SpatialBundle::INHERITED_IDENTITY,
            InstanceMaterialData(grass_instances),
            NoFrustumCulling,
        ));
        commands.entity(entity).remove::<SpawnGrass>();
    }
}

#[derive(Component, Deref)]
struct InstanceMaterialData(Vec<InstanceData>);

impl ExtractComponent for InstanceMaterialData {
    type QueryData = &'static InstanceMaterialData;
    type QueryFilter = ();
    type Out = Self;

    fn extract_component(item: QueryItem<'_, Self::QueryData>) -> Option<Self> {
        Some(InstanceMaterialData(item.0.clone()))
    }
}

pub struct GrassPlugin<T: Heightmap + WithBounds + Component> {
    _marker: std::marker::PhantomData<T>,
}
impl<T: Heightmap + WithBounds + Component> Default for GrassPlugin<T> {
    fn default() -> Self {
        GrassPlugin {
            _marker: std::marker::PhantomData,
        }
    }
}

impl<T: Heightmap + WithBounds + Component> Plugin for GrassPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            ExtractComponentPlugin::<InstanceMaterialData>::default(),
        );
        app.sub_app_mut(RenderApp)
            .add_render_command::<Transparent3d, DrawCustom>()
            .init_resource::<SpecializedMeshPipelines<CustomPipeline>>()
            .add_systems(
                Render,
                (
                    queue_custom.in_set(RenderSet::QueueMeshes),
                    prepare_instance_buffers
                        .in_set(RenderSet::PrepareResources),
                ),
            );
        app.add_systems(Update, spawn_grass::<T>);
    }

    fn finish(&self, app: &mut App) {
        app.sub_app_mut(RenderApp).init_resource::<CustomPipeline>();
    }
}

#[derive(Clone, Copy, Pod, Zeroable)]
#[repr(C)]
struct InstanceData {
    position: Vec3,
    scale: f32,
    
    color: [f32; 4],
}

#[allow(clippy::too_many_arguments)]
fn queue_custom(
    transparent_3d_draw_functions: Res<DrawFunctions<Transparent3d>>,
    custom_pipeline: Res<CustomPipeline>,
    msaa: Res<Msaa>,
    mut pipelines: ResMut<SpecializedMeshPipelines<CustomPipeline>>,
    pipeline_cache: Res<PipelineCache>,
    meshes: Res<RenderAssets<GpuMesh>>,
    render_mesh_instances: Res<RenderMeshInstances>,
    material_meshes: Query<Entity, With<InstanceMaterialData>>,
    mut transparent_render_phases: ResMut<
        ViewSortedRenderPhases<Transparent3d>,
    >,
    mut views: Query<(Entity, &ExtractedView)>,
) {
    let draw_custom = transparent_3d_draw_functions.read().id::<DrawCustom>();

    let msaa_key = MeshPipelineKey::from_msaa_samples(msaa.samples());

    for (view_entity, view) in &mut views {
        let Some(transparent_phase) =
            transparent_render_phases.get_mut(&view_entity)
        else {
            continue;
        };

        let view_key = msaa_key | MeshPipelineKey::from_hdr(view.hdr);
        let rangefinder = view.rangefinder3d();
        for entity in &material_meshes {
            let Some(mesh_instance) =
                render_mesh_instances.render_mesh_queue_data(entity)
            else {
                continue;
            };
            let Some(mesh) = meshes.get(mesh_instance.mesh_asset_id) else {
                continue;
            };
            let key = view_key
                | MeshPipelineKey::from_primitive_topology(
                    mesh.primitive_topology(),
                );
            let pipeline = pipelines
                .specialize(
                    &pipeline_cache,
                    &custom_pipeline,
                    key,
                    &mesh.layout,
                )
                .unwrap();
            transparent_phase.add(Transparent3d {
                entity,
                pipeline,
                draw_function: draw_custom,
                distance: rangefinder
                    .distance_translation(&mesh_instance.translation),
                batch_range: 0..1,
                extra_index: PhaseItemExtraIndex::NONE,
            });
        }
    }
}

#[derive(Component)]
struct InstanceBuffer {
    buffer: Buffer,
    length: usize,
}

fn prepare_instance_buffers(
    mut commands: Commands,
    query: Query<(Entity, &InstanceMaterialData)>,
    render_device: Res<RenderDevice>,
) {
    for (entity, instance_data) in &query {
        let buffer =
            render_device.create_buffer_with_data(&BufferInitDescriptor {
                label: Some("instance data buffer"),
                contents: bytemuck::cast_slice(instance_data.as_slice()),
                usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
            });
        commands.entity(entity).insert(InstanceBuffer {
            buffer,
            length: instance_data.len(),
        });
    }
}

#[derive(Resource)]
struct CustomPipeline {
    shader: Handle<Shader>,
    mesh_pipeline: MeshPipeline,
}

impl FromWorld for CustomPipeline {
    fn from_world(world: &mut World) -> Self {
        let mesh_pipeline = world.resource::<MeshPipeline>();

        CustomPipeline {
            shader: world.load_asset("shaders/instancing.wgsl"),
            mesh_pipeline: mesh_pipeline.clone(),
        }
    }
}

impl SpecializedMeshPipeline for CustomPipeline {
    type Key = MeshPipelineKey;

    fn specialize(
        &self,
        key: Self::Key,
        layout: &MeshVertexBufferLayoutRef,
    ) -> Result<RenderPipelineDescriptor, SpecializedMeshPipelineError> {
        let mut descriptor = self.mesh_pipeline.specialize(key, layout)?;

        descriptor.vertex.shader = self.shader.clone();
        descriptor.vertex.buffers.push(VertexBufferLayout {
            array_stride: std::mem::size_of::<InstanceData>() as u64,
            step_mode: VertexStepMode::Instance,
            attributes: vec![
                VertexAttribute {
                    format: VertexFormat::Float32x4,
                    offset: 0,
                    shader_location: 3, // shader locations 0-2 are taken up by Position, Normal and UV attributes
                },
                VertexAttribute {
                    format: VertexFormat::Float32x4,
                    offset: VertexFormat::Float32x4.size(),
                    shader_location: 4,
                },
            ],
        });
        descriptor.fragment.as_mut().unwrap().shader = self.shader.clone();
        Ok(descriptor)
    }
}

type DrawCustom = (
    SetItemPipeline,
    SetMeshViewBindGroup<0>,
    SetMeshBindGroup<1>,
    DrawMeshInstanced,
);

struct DrawMeshInstanced;

impl<P: PhaseItem> RenderCommand<P> for DrawMeshInstanced {
    type Param = (SRes<RenderAssets<GpuMesh>>, SRes<RenderMeshInstances>);
    type ViewQuery = ();
    type ItemQuery = Read<InstanceBuffer>;

    #[inline]
    fn render<'w>(
        item: &P,
        _view: (),
        instance_buffer: Option<&'w InstanceBuffer>,
        (meshes, render_mesh_instances): SystemParamItem<'w, '_, Self::Param>,
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        let Some(mesh_instance) =
            render_mesh_instances.render_mesh_queue_data(item.entity())
        else {
            return RenderCommandResult::Failure;
        };
        let Some(gpu_mesh) =
            meshes.into_inner().get(mesh_instance.mesh_asset_id)
        else {
            return RenderCommandResult::Failure;
        };
        let Some(instance_buffer) = instance_buffer else {
            return RenderCommandResult::Failure;
        };

        pass.set_vertex_buffer(0, gpu_mesh.vertex_buffer.slice(..));
        pass.set_vertex_buffer(1, instance_buffer.buffer.slice(..));

        match &gpu_mesh.buffer_info {
            GpuBufferInfo::Indexed {
                buffer,
                index_format,
                count,
            } => {
                pass.set_index_buffer(buffer.slice(..), 0, *index_format);
                pass.draw_indexed(
                    0..*count,
                    0,
                    0..instance_buffer.length as u32,
                );
            }
            GpuBufferInfo::NonIndexed => {
                pass.draw(
                    0..gpu_mesh.vertex_count,
                    0..instance_buffer.length as u32,
                );
            }
        }
        RenderCommandResult::Success
    }
}
