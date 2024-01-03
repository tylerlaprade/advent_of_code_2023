use colored::*;
use itertools::Itertools;

pub fn run() {
    let data = std::fs::read("inputs/6.txt").unwrap();
    let text = std::str::from_utf8(&data).unwrap();
    let lines: Vec<&str> = text.lines().collect();

    let time: i64 = lines[0]
        .split_whitespace()
        .skip(1) // Skip the "Time:" part
        .join("")
        .parse()
        .unwrap();

    let target_distance: i64 = lines[1]
        .split_whitespace()
        .skip(1) // Skip the "Distance:" part
        .join("")
        .parse()
        .unwrap();

    let mut travel_time = time - 1;
    let mut winners = 0;
    for charge_time in 1..=time {
        if charge_time * travel_time > target_distance {
            winners += 1;
        }
        travel_time -= 1;
    }

    println!("{}", format!("Winners: {}", winners).on_bright_green());
}
