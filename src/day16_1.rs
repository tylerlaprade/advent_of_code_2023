use crate::day16_shared::illuminate;
use crate::day16_shared::Direction;
use crate::day16_shared::Light;

pub fn run() {
    let data = std::fs::read("inputs/16.txt").unwrap();
    let lines = std::str::from_utf8(&data).unwrap().lines();

    let starting_light = Light {
        x: 0,
        y: 0,
        direction: Direction::Right,
    };

    let visited_count = illuminate(lines, starting_light);
    println!("Visited count: {}", visited_count);
}
