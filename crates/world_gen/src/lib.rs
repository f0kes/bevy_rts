pub mod mesh;
pub mod terrain;
pub mod perlin_terrain;
pub mod raycast;

pub trait Heightmap {
    fn height(&self, x: f32, z: f32) -> f32;
    
}

pub trait WithBounds {
    fn bounds(&self) -> (f32, f32, f32, f32);
}