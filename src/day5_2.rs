pub fn run() {
    let data = std::fs::read("inputs/5.txt").unwrap();
    let text = std::str::from_utf8(&data).unwrap();
    let mut lines = text.lines().map(str::trim_end);
    let mut seeds: Vec<i64> = lines
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect();
    lines.next();
    loop {
        lines.next();
        let group: Vec<&str> = lines.by_ref().take_while(|line| !line.is_empty()).collect();
        if group.is_empty() {
            break;
        }
        let mut new_seeds = Vec::new();
        for line in group {
            let parts: Vec<i64> = line
                .split_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect();
            let (destination_start, source_start, range) = (parts[0], parts[1], parts[2]);
            let mut to_remove = Vec::new();
            for (i, seed) in seeds.iter().enumerate() {
                if (source_start..source_start + range).contains(seed) {
                    to_remove.push(i);
                    new_seeds.push(seed - source_start + destination_start);
                }
            }
            to_remove.sort_unstable();
            for i in to_remove.iter().rev() {
                seeds.remove(*i);
            }
        }
        seeds.append(&mut new_seeds);
    }
    println!("Minimal location: {}", seeds.iter().min().unwrap());
}
