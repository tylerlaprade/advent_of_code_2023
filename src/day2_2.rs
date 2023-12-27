use std::collections::HashMap;

pub fn run() {
    let data = std::fs::read("inputs/2.txt").unwrap();
    let text = std::str::from_utf8(&data).unwrap();
    let mut lines = text.lines().map(str::trim_end);
    let mut sum = 0;
    loop {
        let group: Vec<_> = lines.by_ref().take_while(|line| !line.is_empty()).collect();
        if group.is_empty() {
            break;
        }
        for line in &group {
            let mut mins: HashMap<_, i32> = [("red", 0), ("green", 0), ("blue", 0)]
                .iter()
                .cloned()
                .collect();
            let split_line = line.split(':');
            let rounds = split_line.last().unwrap();
            for round in rounds.split(';') {
                for ball_event in round.split(',') {
                    let split_ball_event = ball_event.trim().split(' ');
                    let ball_count = split_ball_event.clone().next().unwrap();
                    let color = split_ball_event.last().unwrap();
                    if ball_count.parse::<i32>().unwrap() > mins[color] {
                        mins.insert(color, ball_count.parse::<i32>().unwrap());
                    }
                }
            }
            sum += mins["red"] * mins["green"] * mins["blue"];
        }
    }
    println!("Sum: {}", sum);
}
