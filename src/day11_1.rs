use std::collections::{HashMap, HashSet};

use itertools::Itertools;

pub fn run() {
    let data = std::fs::read("inputs/11.txt").unwrap();
    let text = std::str::from_utf8(&data).unwrap();
    let mut galaxies = HashSet::new();
    let mut empty_lines = 0;
    let mut nonempty_columns = HashSet::new();
    let mut biggest_x = 0;
    for (y, line) in text.lines().enumerate() {
        let mut line_is_empty = true;
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                galaxies.insert((x, y + empty_lines));
                line_is_empty = false;
                nonempty_columns.insert(x);
            }
            if x > biggest_x {
                biggest_x = x;
            }
        }
        if line_is_empty {
            empty_lines += 1;
        }
    }
    let mut column_map = HashMap::new();
    let mut offset = 0;
    for i in 0..biggest_x + 1 {
        if !nonempty_columns.contains(&i) {
            offset += 1;
        }
        column_map.insert(i, i + offset);
    }
    let count = galaxies.len() as isize;
    let mut distance = 0;
    let mut multiplier = 1 - count;
    for x in galaxies.iter().map(|galaxy| galaxy.0).sorted() {
        distance += column_map[&x] as isize * multiplier;
        println!("{} * {}", column_map[&x], multiplier);
        multiplier += 2
    }
    multiplier = 1 - count;
    for y in galaxies.iter().map(|galaxy| galaxy.1).sorted() {
        distance += y as isize * multiplier;
        println!("{} * {}", y, multiplier);
        multiplier += 2
    }
    println!("Distance traveled: {}", distance);
}
