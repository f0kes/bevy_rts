use bevy::prelude::*;

pub struct BoxData {
    pub start: Vec3,
    pub end: Vec3,
    pub dimensions: Vec3,
}

pub trait RotatedBoxFilter<'a>:
    IntoIterator<Item = (Entity, Vec3)> + Sized + 'a
{
    fn in_rotated_box(
        self,
        box_data: BoxData,
    ) -> impl Iterator<Item = (Entity, Vec3)> + 'a {
        let box_center = (box_data.start + box_data.end) * 0.5;
        let box_forward = (box_data.end - box_data.start).normalize_or_zero();

        let (box_right, box_up) = if box_forward.dot(Vec3::Y).abs() > 0.9999 {
            (Vec3::X, box_forward.signum() * Vec3::Y)
        } else {
            let right = Vec3::Y.cross(box_forward).normalize_or_zero();
            let up = box_forward.cross(right).normalize_or_zero();
            (right, up)
        };

        self.into_iter().filter(move |(_, pos)| {
            let relative_pos = *pos - box_center;

            let local_x = relative_pos.dot(box_right);
            let local_y = relative_pos.dot(box_up);
            let local_z = relative_pos.dot(box_forward);

            let half_width = box_data.dimensions.x * 0.5;
            let half_height = box_data.dimensions.y * 0.5;
            let half_length = box_data.dimensions.z * 0.5;

            let epsilon = f32::EPSILON * 100.0;
            local_x.abs() <= half_width + epsilon
                && local_y.abs() <= half_height + epsilon
                && local_z.abs() <= half_length + epsilon
        })
    }
}

impl<'a, T> RotatedBoxFilter<'a> for T where
    T: IntoIterator<Item = (Entity, Vec3)> + 'a
{
}
