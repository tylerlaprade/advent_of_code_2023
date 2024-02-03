use std::collections::HashSet;

use itertools::Itertools;

pub fn dig_trenches(parse_line: fn(&str, &str, &str) -> (String, isize)) {
    let data = std::fs::read("inputs/17.txt").unwrap();
    let lines = std::str::from_utf8(&data).unwrap().lines();
    let mut filled_squares = HashSet::new();
    let mut position: (isize, isize) = (0, 0);
    let mut upper_left_corner = (100000, 100000);
    for line in lines {
        let (direction, distance, color) = line
            .split_ascii_whitespace()
            .into_iter()
            .collect_tuple()
            .unwrap();
        let (direction, distance) = parse_line(direction, distance, color);
        match direction.as_str() {
            "U" => {
                for i in 0..distance {
                    filled_squares.insert((position.0, position.1 - i));
                }
                position.1 -= distance;
            }
            "D" => {
                for i in 0..distance {
                    filled_squares.insert((position.0, position.1 + i));
                }
                position.1 += distance;
            }
            "L" => {
                for i in 0..distance {
                    filled_squares.insert((position.0 - i, position.1));
                }
                position.0 -= distance;
            }
            "R" => {
                for i in 0..distance {
                    filled_squares.insert((position.0 + i, position.1));
                }
                position.0 += distance;
            }
            _ => panic!("Unknown direction"),
        }

        if position.1 < upper_left_corner.1 || position.0 < upper_left_corner.0 {
            upper_left_corner = position;
        }
    }
    let mut stack = vec![(upper_left_corner.0 + 1, upper_left_corner.1 + 1)];
    while let Some((x, y)) = stack.pop() {
        if !filled_squares.contains(&(x, y)) {
            filled_squares.insert((x, y));
            println!("({}, {})", x, y);
            if !filled_squares.contains(&(x, y - 1)) {
                stack.push((x, y - 1));
            }
            if !filled_squares.contains(&(x, y + 1)) {
                stack.push((x, y + 1));
            }
            if !filled_squares.contains(&(x - 1, y)) {
                stack.push((x - 1, y));
            }
            if !filled_squares.contains(&(x + 1, y)) {
                stack.push((x + 1, y));
            }
        }
    }
    println!("Area: {}", filled_squares.len());
}
