use std::collections::HashMap;

pub fn run() {
    let data = std::fs::read("inputs/8.txt").unwrap();
    let text = std::str::from_utf8(&data).unwrap();
    let mut lines = text.lines().map(str::trim_end);
    let instructions = lines.next().unwrap().chars();
    lines.next(); // Skip empty line
    let mapping_dict: HashMap<&str, (&str, &str)> = lines
        .map(|line| {
            let mut parts = line.split(" = (");
            let from = parts.next().unwrap();
            let to: Vec<&str> = parts
                .next()
                .unwrap()
                .split(", ")
                .map(|item| item.trim_end_matches(")"))
                .collect();
            (from, (to[0], to[1]))
        })
        .collect();
    let mut i = 0;
    let mut current_position = "AAA";
    loop {
        for instruction in instructions.clone() {
            let target = mapping_dict.get(current_position).unwrap();
            match instruction {
                'L' => current_position = target.0,
                'R' => current_position = target.1,
                _ => panic!("Unknown instruction"),
            }
            i += 1;
            if current_position == "ZZZ" {
                println!("Found ZZZ in {} steps", i);
                return;
            }
        }
    }
}
