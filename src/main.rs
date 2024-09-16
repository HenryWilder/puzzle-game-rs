//! Puzzle game inspired by *Can of Wormholes* and *BABA IS YOU*.

#![warn(missing_docs)]

use raylib::prelude::*;

pub mod spacial;
pub mod rules;
pub mod worm;
use spacial::{direction3::Direction3, vector3i::Vector3i};
use worm::*;

/// The sidelength of a cell in the game's grid.
pub const CELL_SIZE: f32 = 8.0;

/// [`CELL_SIZE`] as a [`Vector3`].
pub const VOXEL: Vector3 = Vector3::new(CELL_SIZE, CELL_SIZE, CELL_SIZE);

/// Converts from grid coordinates to world coordinates.
pub fn cell_to_world(cell: Vector3i) -> Vector3 {
    Vector3::new(
        cell.x as f32 * CELL_SIZE,
        cell.y as f32 * CELL_SIZE,
        cell.z as f32 * CELL_SIZE,
    )
}

/// Converts from world coordinates to grid coordinates.
pub fn world_to_cell(position: Vector3) -> Vector3i {
    const RECIPROCAL_CELL_SIZE: f32 = 1.0 / CELL_SIZE;
    Vector3i::new(
        (position.x * RECIPROCAL_CELL_SIZE).round() as i32,
        (position.y * RECIPROCAL_CELL_SIZE).round() as i32,
        (position.z * RECIPROCAL_CELL_SIZE).round() as i32,
    )
}

fn main() {
    let (mut rl, thread) = init()
        .size(640, 480)
        .title("puzzle game")
        .build();

    let mut worm = Worm::new(
        Vector3i::new(0, 0, 0),
        [
            Direction3::South,
            Direction3::East,
            Direction3::East,
            Direction3::East,
            Direction3::South,
            Direction3::South,
            Direction3::South,
            Direction3::West,
            Direction3::West,
            Direction3::West,
        ]
    );

    let camera = Camera3D::perspective(
        Vector3::new(0.0, 0.0, CELL_SIZE * 8.0),
        Vector3::zero(),
        Vector3::new(0.0, 1.0, 0.0),
        90.0,
    );

    while !rl.window_should_close() {
        // Tick

        let crawl_direction = {
            let input_left  = rl.is_key_pressed(KeyboardKey::KEY_A) || rl.is_key_pressed(KeyboardKey::KEY_LEFT);
            let input_right = rl.is_key_pressed(KeyboardKey::KEY_D) || rl.is_key_pressed(KeyboardKey::KEY_RIGHT);
            let input_up    = rl.is_key_pressed(KeyboardKey::KEY_W) || rl.is_key_pressed(KeyboardKey::KEY_UP);
            let input_down  = rl.is_key_pressed(KeyboardKey::KEY_S) || rl.is_key_pressed(KeyboardKey::KEY_DOWN);
            let input_horizontal = (input_right as isize) - (input_left as isize);
            let input_vertical   = (input_up    as isize) - (input_down as isize);
            // horizontal takes priority
            match (input_horizontal, input_vertical) {
                (    1..,   _  ) => Some(Direction3::East),
                (..=-1  ,   _  ) => Some(Direction3::West),
                (    0,     1..) => Some(Direction3::North),
                (    0, ..=-1  ) => Some(Direction3::South),
                (    0,     0  ) => None,
            }
        };

        if let Some(direction) = crawl_direction {
            worm.crawl(direction);
        }

        // Draw

        {
            let worm = &worm; // Immutable while drawing
            let mut d = rl.begin_drawing(&thread);
            d.clear_background(Color::BLACK);
            {
                let mut d3 = d.begin_mode3D(camera);
                let tail_index = worm.num_segments() - 1;
                for (i, segment) in worm.segment_positions().enumerate() {
                    let world_pos = cell_to_world(segment);
                    let growth = (((i == 0) as isize) - ((i == tail_index) as isize)) as f32;
                    d3.draw_sphere(world_pos, CELL_SIZE / 2.0 + growth, Color::ORANGE);
                }
            }
        }
    }
}
