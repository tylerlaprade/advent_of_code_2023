use std::collections::HashSet;

pub fn run() {
    let data = std::fs::read("inputs/4.txt").unwrap();
    let text = std::str::from_utf8(&data).unwrap();
    let mut lines = text.lines().map(str::trim_end);
    let mut sum = 0;
    loop {
        let group: Vec<_> = lines.by_ref().take_while(|line| !line.is_empty()).collect();
        if group.is_empty() {
            break;
        }
        for line in &group {
            let split_line = line.split(':');
            let card = split_line.last().unwrap().split('|');
            let winning_numbers = card
                .clone()
                .next()
                .unwrap()
                .split(' ')
                .map(str::trim)
                .filter(|ticket| !ticket.is_empty())
                .collect::<HashSet<_>>();
            let tickets = card
                .clone()
                .last()
                .unwrap()
                .split(' ')
                .map(str::trim)
                .collect::<Vec<_>>();
            let mut prize = 1;
            for ticket in tickets.iter().filter(|ticket| !ticket.is_empty()) {
                if winning_numbers.contains(ticket) {
                    prize <<= 1;
                }
            }
            if prize != 0 {
                sum += prize >> 1;
            }
        }
    }
    println!("Sum: {}", sum);
}
