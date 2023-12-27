use std::collections::HashMap;

pub fn run() {
    let data = std::fs::read("inputs/2.txt").unwrap();
    let text = std::str::from_utf8(&data).unwrap();
    let mut lines = text.lines().map(str::trim_end);
    let mut sum = 0;
    let maxes: HashMap<_, i32> = [("red", 12), ("green", 13), ("blue", 14)]
        .iter()
        .cloned()
        .collect();
    loop {
        let group: Vec<_> = lines.by_ref().take_while(|line| !line.is_empty()).collect();
        if group.is_empty() {
            break;
        }
        'line_loop: for line in &group {
            let split_line = line.split(':');
            let game_label = split_line.clone().next().unwrap();
            let game_id = game_label.split(' ').last().unwrap();
            let rounds = split_line.last().unwrap();
            for round in rounds.split(';') {
                for ball_event in round.split(',') {
                    let split_ball_event = ball_event.trim().split(' ');
                    let ball_count = split_ball_event.clone().next().unwrap();
                    let color = split_ball_event.last().unwrap();
                    if ball_count.parse::<i32>().unwrap() > maxes[color] {
                        continue 'line_loop;
                    }
                }
            }
            sum += game_id.parse::<i32>().unwrap();
        }
    }
    println!("Sum: {}", sum);
}
