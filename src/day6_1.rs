use colored::*;

pub fn run() {
    let data = std::fs::read("inputs/6.txt").unwrap();
    let text = std::str::from_utf8(&data).unwrap();
    let lines: Vec<&str> = text.lines().collect();

    let times: Vec<i32> = lines[0]
        .split_whitespace()
        .skip(1) // Skip the "Time:" part
        .map(|s| s.parse().unwrap())
        .collect();

    let distances: Vec<i32> = lines[1]
        .split_whitespace()
        .skip(1) // Skip the "Distance:" part
        .map(|s| s.parse().unwrap())
        .collect();

    let mut product = 1;
    for (time, target_distance) in itertools::zip_eq(times, distances) {
        let mut travel_time = time-1;
        let mut winners = 0;
        for charge_time in 1..=time {
            if charge_time * travel_time > target_distance {
                winners += 1;
            }
            travel_time -= 1;
        }
        product *= winners;
        println!("{}", format!("Winners: {}", winners).on_bright_purple())
    }

    println!("{}", format!("Product: {}", product).on_bright_green());
}
