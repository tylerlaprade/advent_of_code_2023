use std::collections::HashMap;

pub fn run() {
    let data = std::fs::read("inputs/8.txt").unwrap();
    let text = std::str::from_utf8(&data).unwrap();
    let mut lines = text.lines().map(str::trim_end);
    let instructions: Vec<char> = lines.next().unwrap().chars().collect();
    lines.next(); // Skip empty line
    let (mut mapping_dict_left, mut mapping_dict_right): (
        HashMap<&str, &str>,
        HashMap<&str, &str>,
    ) = (HashMap::new(), HashMap::new());
    for line in lines {
        let mut parts = line.split(" = (");
        let from = parts.next().unwrap();
        let to: Vec<&str> = parts
            .next()
            .unwrap()
            .split(", ")
            .map(|item| item.trim_end_matches(")"))
            .collect();
        mapping_dict_left.insert(from, to[0]);
        mapping_dict_right.insert(from, to[1]);
    }
    let mut i: i128 = 0;
    let mut current_positions: Vec<&str> = mapping_dict_left
        .keys()
        .filter(|key| key.ends_with("A"))
        .cloned()
        .collect();
    loop {
        for instruction in &instructions {
            let mapping_dict = match instruction {
                'L' => &mapping_dict_left,
                'R' => &mapping_dict_right,
                _ => panic!("Unknown instruction"),
            };
            let mut all_z = true;
            for current_position in &mut current_positions {
                *current_position = mapping_dict.get(*current_position).unwrap();
                if all_z && !current_position.ends_with("Z") {
                    all_z = false;
                }
            }
            i += 1;
            if all_z {
                println!("Found all Z's in {} steps", i);
                return;
            }
        }
    }
}
