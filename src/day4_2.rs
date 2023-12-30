use std::collections::{HashSet, VecDeque};

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
        let mut pending_cards: VecDeque<usize> = VecDeque::new();
        let mut i = 1;
        for line in &group {
            let multiplier = pending_cards.pop_front().unwrap_or(1);
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
            let mut number_of_matches = 0;
            for ticket in tickets.iter().filter(|ticket| !ticket.is_empty()) {
                if winning_numbers.contains(ticket) {
                    number_of_matches += 1;
                }
            }
            println!(
                "Ticket {} has {} matches with multiplier {}",
                i, number_of_matches, multiplier
            );
            i += 1;
            sum += multiplier;
            for num in pending_cards.iter_mut().take(number_of_matches) {
                *num += multiplier;
            }
            if number_of_matches > pending_cards.len() {
                pending_cards.resize(number_of_matches, 1 + multiplier)
            }
            println!("Pending cards: {:?}", pending_cards);
        }
    }
    println!("Sum: {}", sum);
}
