use bevy::prelude::*;

pub struct ConeData {
    pub apex: Vec3,      // Cone's tip position
    pub direction: Vec3, // Direction the cone points
    pub angle: f32,      // Half angle of the cone in radians
    pub height: f32,     // Height/length of the cone
}

pub trait ConeFilter<'a>: IntoIterator<Item = (Entity, Vec3)> + Sized + 'a {
    fn in_cone(
        self,
        cone_data: ConeData,
    ) -> impl Iterator<Item = (Entity, Vec3)> + 'a {
        let direction = cone_data.direction.normalize_or_zero();
        let cos_angle = cone_data.angle.cos();

        self.into_iter().filter(move |(_, pos)| {
            let to_point = *pos - cone_data.apex;
            let distance = to_point.length();

            // If point is beyond cone height, filter it out
            if distance > cone_data.height {
                return false;
            }

            // If distance is very close to zero, consider it inside
            if distance < f32::EPSILON {
                return true;
            }

            // Calculate angle between cone direction and point
            let cos_point_angle = to_point.dot(direction) / distance;

            // Check if point is within cone angle
            cos_point_angle >= cos_angle
        })
    }
}

impl<'a, T> ConeFilter<'a> for T where
    T: IntoIterator<Item = (Entity, Vec3)> + 'a
{
}
