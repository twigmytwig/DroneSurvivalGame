use bevy::platform::collections::HashMap;
use bevy::prelude::*;

pub const GRID_CELL_SIZE: u32 = 64;

// (i32,i32) is like a grid coordinate
#[derive(Resource, Default)]
pub struct BuildGrid{
    pub occupied_cells: HashMap<(i32,i32), Entity>,
}

// need to convert between world coordinates and grid coordinates.
pub fn world_to_grid(world_pos: Vec2) -> (i32, i32) {
    let size = GRID_CELL_SIZE as f32;
    ((world_pos.x / size).floor() as i32, (world_pos.y / size).floor() as i32)
}

pub fn grid_to_world(grid_pos: (i32, i32)) -> Vec2 {
    let size = GRID_CELL_SIZE as f32;
    //the +size /2 is for center of cell coordinate
    Vec2::new(grid_pos.0 as f32 * size + size / 2.0, grid_pos.1 as f32 * size + size / 2.0)
}