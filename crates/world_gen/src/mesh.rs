use bevy::{prelude::*, render::render_asset::RenderAssetUsages};
use noise::{NoiseFn, Perlin};



/* pub fn create_mesh_from_noise(
    terrain_plane_options: TerrainPlaneOptions,
    noise: impl NoiseFn<f64, 2>,
) -> Mesh {
    let mut positions = Vec::new();

    let mut uvs = Vec::new();
    let mut indices = Vec::new();

    let TerrainPlaneOptions {
        width,
        height,
        width_segments,
        height_segments,
        noise_scale,
        height_scale,
    } = terrain_plane_options;

    let width_half = width / 2.0;
    let height_half = height / 2.0;
    let grid_x = width_segments;
    let grid_y = height_segments;
    let segment_width = width / grid_x as f32;
    let segment_height = height / grid_y as f32;

    

    // Generate vertices with noise-based displacement
    let mut vertex_grid =
        vec![vec![Vec3::ZERO; (grid_x + 1) as usize]; (grid_y + 1) as usize];

    for iy in 0..=grid_y {
        let y = iy as f32 * segment_height - height_half;
        for ix in 0..=grid_x {
            let x = ix as f32 * segment_width - width_half;

            // Generate height using noise
            let noise_value = noise.get([
                x as f64 * noise_scale as f64,
                y as f64 * noise_scale as f64,
            ]) as f32;
            let height_offset = noise_value * height_scale;

            let vertex = Vec3::new(x, height_offset, y);
            vertex_grid[iy as usize][ix as usize] = vertex;
            positions.push([vertex.x, vertex.y, vertex.z]);
            uvs.push([ix as f32 / grid_x as f32, iy as f32 / grid_y as f32]);
        }
    }

    // Calculate normals

    // Generate indices
    for iy in 0..grid_y {
        for ix in 0..grid_x {
            let a = ix + (grid_x + 1) * iy;
            let b = ix + (grid_x + 1) * (iy + 1);
            let c = (ix + 1) + (grid_x + 1) * (iy + 1);
            let d = (ix + 1) + (grid_x + 1) * iy;

            indices.push(a);
            indices.push(b);
            indices.push(d);
            indices.push(b);
            indices.push(c);
            indices.push(d);
        }
    }

    // Create the mesh
    let mut mesh = Mesh::new(
        bevy::render::render_resource::PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(bevy::render::mesh::Indices::U32(indices));
    mesh.compute_smooth_normals();
    mesh
}
 */