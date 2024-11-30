use bevy::{math::vec2, prelude::*};

use super::spatial_hashmap::Grid;

pub trait SpatialQuery {
    fn first_cell(&self, grid: Grid) -> IVec2;
    fn next_cell(&self, cell: IVec2, grid: Grid) -> Option<IVec2>;
    fn in_range(&self, position: Vec2) -> bool;
}

#[derive(Debug, Default)]
pub struct SquareQuery {
    center: Vec2,
    radius: f32,
}

impl SquareQuery {
    pub fn new(center: Vec2, radius: f32) -> Self {
        Self { center, radius }
    }
}

impl SpatialQuery for SquareQuery {
    fn first_cell(&self, grid: Grid) -> IVec2 {
        grid.index2d(self.center - vec2(self.radius, self.radius))
    }

    fn next_cell(&self, mut cell_index: IVec2, grid: Grid) -> Option<IVec2> {
        cell_index.x += 1;

        if grid.position1d_low(cell_index.x) > self.center.x + self.radius {
            cell_index.x = grid.index1d(self.center.x - self.radius);
            cell_index.y += 1;
        }

        if grid.position1d_low(cell_index.y) > self.center.y + self.radius {
            return None;
        }

        Some(cell_index)
    }

    fn in_range(&self, position: Vec2) -> bool {
        position.x >= self.center.x - self.radius
            && position.x < self.center.x + self.radius
            && position.y >= self.center.y - self.radius
            && position.y < self.center.y + self.radius
    }
}

#[derive(Debug, Default)]
pub struct CircleQuery {
    center: Vec2,
    radius: f32,
}

impl CircleQuery {
    pub fn new(center: Vec2, radius: f32) -> Self {
        Self { center, radius }
    }
}

impl SpatialQuery for CircleQuery {
    fn first_cell(&self, grid: Grid) -> IVec2 {
        // Start from the top-left corner of the bounding box
        grid.index2d(self.center - vec2(self.radius, self.radius))
    }

    fn next_cell(&self, mut cell_index: IVec2, grid: Grid) -> Option<IVec2> {
        cell_index.x += 1;

        // If we've gone past the right edge of the bounding box
        if grid.position1d_low(cell_index.x) > self.center.x + self.radius {
            // Move to the start of the next row
            cell_index.x = grid.index1d(self.center.x - self.radius);
            cell_index.y += 1;
        }

        // If we've gone past the bottom edge of the bounding box
        if grid.position1d_low(cell_index.y) > self.center.y + self.radius {
            return None;
        }

        Some(cell_index)
    }

    fn in_range(&self, position: Vec2) -> bool {
        // Check if the point is within the circle using distance squared
        let offset = position - self.center;
        offset.length_squared() < self.radius * self.radius
    }
}
