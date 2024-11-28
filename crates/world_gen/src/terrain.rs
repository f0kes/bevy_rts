use bevy::prelude::*;

use bevy::render::render_asset::RenderAssetUsages;
use noise::NoiseFn;
use outline::grass::{Heightmap, WithBounds};

pub struct TerrainPlaneOptions {
    pub width: f32,
    pub height: f32,
    pub width_segments: u32,
    pub height_segments: u32,
    pub noise_scale: f32, // Controls the frequency of the noise
    pub height_scale: f32, // Controls the amplitude of the displacement
}

impl Default for TerrainPlaneOptions {
    fn default() -> Self {
        TerrainPlaneOptions {
            width: 200.0,
            height: 200.0,
            width_segments: 50,
            height_segments: 50,
            noise_scale: 0.1,
            height_scale: 2.0,
        }
    }
}
#[derive(Resource, Component, Clone)]
pub struct Terrain {
    mesh: Mesh,
    vertex_grid: Vec<Vec<Vec3>>,
    width: f32,
    height: f32,
    width_segments: u32,
    height_segments: u32,
}

pub trait TerrainLike {
    fn get_height(&self, x: f32, z: f32) -> f32;
    fn get_normal(&self, x: f32, z: f32) -> Vec3;
    fn get_mesh(&self) -> &Mesh;
}
impl Heightmap for Terrain {
    fn height(&self, x: f32, z: f32) -> f32 {
        self.get_height(x, z)
    }
}
impl WithBounds for Terrain {
    fn bounds(&self) -> (f32, f32, f32, f32) {
        (
            -self.width / 2.0,
            self.width / 2.0,
            -self.height / 2.0,
            self.height / 2.0,
        )
    }
}

impl Terrain {
    pub fn new(
        terrain_plane_options: TerrainPlaneOptions,
        noise: impl NoiseFn<f64, 2>,
    ) -> Self {
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
        let segment_width = width / width_segments as f32;
        let segment_height = height / height_segments as f32;

        let mut positions = Vec::new();
        let mut uvs = Vec::new();
        let mut indices = Vec::new();

        // Generate vertices with noise-based displacement
        let mut vertex_grid =
            vec![
                vec![Vec3::ZERO; (width_segments + 1) as usize];
                (height_segments + 1) as usize
            ];

        for iy in 0..=height_segments {
            let y = iy as f32 * segment_height - height_half;
            for ix in 0..=width_segments {
                let x = ix as f32 * segment_width - width_half;
                let noise_value = noise.get([
                    x as f64 * noise_scale as f64,
                    y as f64 * noise_scale as f64,
                ]) as f32;
                let height_offset = noise_value * height_scale;
                let vertex = Vec3::new(x, height_offset, y);
                vertex_grid[iy as usize][ix as usize] = vertex;
                positions.push([vertex.x, vertex.y, vertex.z]);
                uvs.push([
                    ix as f32 / width_segments as f32,
                    iy as f32 / height_segments as f32,
                ]);
            }
        }

        // Generate indices
        for iy in 0..height_segments {
            for ix in 0..width_segments {
                let a = ix + (width_segments + 1) * iy;
                let b = ix + (width_segments + 1) * (iy + 1);
                let c = (ix + 1) + (width_segments + 1) * (iy + 1);
                let d = (ix + 1) + (width_segments + 1) * iy;
                indices.push(a);
                indices.push(b);
                indices.push(d);
                indices.push(b);
                indices.push(c);
                indices.push(d);
            }
        }

        let mut mesh = Mesh::new(
            bevy::render::render_resource::PrimitiveTopology::TriangleList,
            RenderAssetUsages::RENDER_WORLD,
        );
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh.insert_indices(bevy::render::mesh::Indices::U32(indices));
        mesh.compute_smooth_normals();
        mesh.generate_tangents().unwrap_or_else(|e| {
            println!("Failed to generate tangents: {:?}", e);
        });

        Self {
            mesh,
            vertex_grid,
            width,
            height,
            width_segments,
            height_segments,
        }
    }
    pub fn new_perlin(
        terrain_plane_options: TerrainPlaneOptions,
        noise: u32,
    ) -> Self {
        let noise = noise::Perlin::new(noise);
        Self::new(terrain_plane_options, noise)
    }
}

impl TerrainLike for Terrain {
    fn get_height(&self, x: f32, z: f32) -> f32 {
        // Convert world coordinates to grid coordinates
        let width_half = self.width / 2.0;
        let height_half = self.height / 2.0;

        let grid_x = ((x + width_half) / self.width
            * self.width_segments as f32)
            .floor();
        let grid_z = ((z + height_half) / self.height
            * self.height_segments as f32)
            .floor();

        // Ensure we're within bounds
        if grid_x < 0.0
            || grid_x >= self.width_segments as f32
            || grid_z < 0.0
            || grid_z >= self.height_segments as f32
        {
            return 0.0;
        }

        // Get the four corners of the grid cell
        let x0 = grid_x as usize;
        let z0 = grid_z as usize;
        let x1 = (x0 + 1).min(self.width_segments as usize);
        let z1 = (z0 + 1).min(self.height_segments as usize);

        // Calculate local coordinates within the grid cell (0 to 1)
        let local_x =
            (x + width_half) / self.width * self.width_segments as f32 - grid_x;
        let local_z = (z + height_half) / self.height
            * self.height_segments as f32
            - grid_z;

        // Get heights at the four corners
        let h00 = self.vertex_grid[z0][x0].y;
        let h10 = self.vertex_grid[z0][x1].y;
        let h01 = self.vertex_grid[z1][x0].y;
        let h11 = self.vertex_grid[z1][x1].y;

        // Bilinear interpolation
        let h0 = h00 * (1.0 - local_x) + h10 * local_x;
        let h1 = h01 * (1.0 - local_x) + h11 * local_x;
        h0 * (1.0 - local_z) + h1 * local_z
    }

    fn get_normal(&self, x: f32, z: f32) -> Vec3 {
        // Calculate normal using central differences
        const EPSILON: f32 = 0.01;
        let h_center = self.get_height(x, z);
        let h_right = self.get_height(x + EPSILON, z);
        let h_up = self.get_height(x, z + EPSILON);

        // Calculate tangent vectors
        let tangent_x = Vec3::new(EPSILON, h_right - h_center, 0.0);
        let tangent_z = Vec3::new(0.0, h_up - h_center, EPSILON);

        // Cross product gives us the normal
        tangent_x.cross(tangent_z).normalize()
    }

    fn get_mesh(&self) -> &Mesh {
        &self.mesh
    }
}
