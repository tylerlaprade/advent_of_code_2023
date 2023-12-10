use std::collections::HashMap;

mod day1_1;
mod day1_2;
// TODO: Add more modules each day

fn main() {
    let mut map = HashMap::new();
    map.insert("1_1", day1_1::run as fn());
    map.insert("1_2", day1_2::run as fn());
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
