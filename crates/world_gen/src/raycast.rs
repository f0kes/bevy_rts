use bevy::prelude::*;
use outline::grass::Heightmap;

pub struct SphereTrace {
    pub ray_start: Vec3,
    pub ray_dir: Vec3,
    pub max_steps: usize,
    pub epsilon: f32,
}
impl SphereTrace {
    pub fn new(ray_start: Vec3, ray_dir: Vec3) -> Self {
        Self {
            ray_start,
            ray_dir,
            ..Default::default()
        }
    }
    pub fn with_max_steps(mut self, max_steps: usize) ->  Self {
        self.max_steps = max_steps;
        self
    }
    pub fn with_epsilon(mut self, epsilon: f32) -> Self {
        self.epsilon = epsilon;
        self
    }
}
impl Default for SphereTrace {
    fn default() -> Self {
        Self {
            ray_start: Vec3::ZERO,
            ray_dir: Vec3::Y,
            max_steps: 100,
            epsilon: 0.1,
        }
    }
}

pub fn sphere_trace_heightmap(
    sphere_trace: SphereTrace,
    heightmap: &impl Heightmap,
) -> Option<Vec3> {
    let mut t = 0.0;
    let SphereTrace {
        ray_start,
        ray_dir,
        max_steps,
        epsilon,
    } = sphere_trace;

    for _ in 0..max_steps {
        let p = ray_start + ray_dir * t;
        let h = heightmap.height(p.x, p.z);
        let distance = p.y - h;

        if distance.abs() < epsilon {
            return Some(p);
        }

        if distance < 0.0 {
            t -= distance * 0.5;
        } else {
            t += distance * 0.5;
        }
    }
    None
}
