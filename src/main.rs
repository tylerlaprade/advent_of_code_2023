use std::collections::HashMap;

mod day10_1;
mod day10_2;
mod day11_1;
mod day11_2;
mod day14_1;
mod day15_1;
mod day16_1;
mod day16_2;
mod day16_shared;
mod day17_1;
mod day17_shared;
mod day1_1;
mod day1_2;
mod day2_1;
mod day2_2;
mod day3_1;
mod day3_2;
mod day4_1;
mod day4_2;
mod day5_1;
mod day5_2;
mod day6_1;
mod day6_2;
mod day7_1;
mod day7_2;
mod day8_1;
mod day9_1;
mod day9_2;
// TODO: Add more modules each day

fn main() {
    let mut map = HashMap::new();
    map.insert("1_1", day1_1::run as fn());
    map.insert("1_2", day1_2::run as fn());
    map.insert("2_1", day2_1::run as fn());
    map.insert("2_2", day2_2::run as fn());
    map.insert("3_1", day3_1::run as fn());
    map.insert("3_2", day3_2::run as fn());
    map.insert("4_1", day4_1::run as fn());
    map.insert("4_2", day4_2::run as fn());
    map.insert("5_1", day5_1::run as fn());
    map.insert("5_2", day5_2::run as fn());
    map.insert("6_1", day6_1::run as fn());
    map.insert("6_2", day6_2::run as fn());
    map.insert("7_1", day7_1::run as fn());
    map.insert("7_2", day7_2::run as fn());
    map.insert("8_1", day8_1::run as fn());
    map.insert("9_1", day9_1::run as fn());
    map.insert("9_2", day9_2::run as fn());
    map.insert("10_1", day10_1::run as fn());
    map.insert("10_2", day10_2::run as fn());
    map.insert("11_1", day11_1::run as fn());
    map.insert("11_2", day11_2::run as fn());
    map.insert("14_1", day14_1::run as fn());
    map.insert("15_1", day15_1::run as fn());
    map.insert("16_1", day16_1::run as fn());
    map.insert("16_2", day16_2::run as fn());
    map.insert("17_1", day17_1::run as fn());
    // TODO: Add more entries each day

    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        match map.get(format!("{}_{}", args[1].as_str(), args[2].as_str()).as_str()) {
            Some(&function) => function(),
            _ => println!("Invalid argument"),
        }
    } else {
        println!("Requires exactly two arguments to specify which puzzle to run!");
    }
}
