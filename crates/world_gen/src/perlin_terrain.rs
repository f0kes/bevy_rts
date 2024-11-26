use bevy::prelude::*;
use noise::Perlin;

use crate::terrain::{Terrain, TerrainLike, TerrainPlaneOptions};

pub struct PerlinTerrain {
    terrain: Terrain,
    pub noise: Perlin,
}
impl PerlinTerrain {
    pub fn new(terrain_options: TerrainPlaneOptions, noise: u32) -> Self {
        let noise = Perlin::new(noise);
        let terrain = Terrain::new(terrain_options, noise);
        Self { terrain, noise }
    }
}
impl Default for PerlinTerrain {
    fn default() -> Self {
        Self::new(TerrainPlaneOptions::default(), 32)
    }
}
impl TerrainLike for PerlinTerrain {
    fn get_height(&self, x: f32, z: f32) -> f32 {
        self.terrain.get_height(x, z)
    }

    fn get_normal(&self, x: f32, z: f32) -> Vec3 {
        self.terrain.get_normal(x, z)
    }

    fn get_mesh(&self) -> &Mesh {
        self.terrain.get_mesh()
    }
}
