use std::cmp::max;

use crate::day16_shared::illuminate;
use crate::day16_shared::Direction;
use crate::day16_shared::Light;

pub fn run() {
    let mut max_count = 0;

    let data = std::fs::read("inputs/16.txt").unwrap();
    let lines = std::str::from_utf8(&data).unwrap().lines();

    let collected_lines: Vec<_> = lines.clone().collect();
    let grid_height = collected_lines.len();
    let grid_width = collected_lines[0].len();

    for x in 0..grid_width {
        max_count = max(
            max_count,
            illuminate(
                lines.clone(),
                Light {
                    x,
                    y: 0,
                    direction: Direction::Down,
                },
            ),
        );
        max_count = max(
            max_count,
            illuminate(
                lines.clone(),
                Light {
                    x,
                    y: grid_height - 1,
                    direction: Direction::Up,
                },
            ),
        );
    }

    for y in 0..grid_height {
        max_count = max(
            max_count,
            illuminate(
                lines.clone(),
                Light {
                    x: 0,
                    y,
                    direction: Direction::Right,
                },
            ),
        );
        max_count = max(
            max_count,
            illuminate(
                lines.clone(),
                Light {
                    x: grid_width - 1,
                    y,
                    direction: Direction::Left,
                },
            ),
        );
    }

    println!("Max count: {}", max_count);
}
