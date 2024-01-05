use itertools::Itertools;
use std::collections::HashMap;

pub fn run() {
    let data = std::fs::read("inputs/7.txt").unwrap();
    let text = std::str::from_utf8(&data).unwrap();
    let lines = text
        .lines()
        .map(str::split_whitespace)
        .map(|mut line| (line.next().unwrap(), line.next().unwrap()))
        .sorted_unstable_by(compare_hands);
    let mut total_winnings = 0;
    for (i, (_hand, bid)) in lines.enumerate() {
        total_winnings += (i + 1) * bid.parse::<usize>().unwrap();
    }
    println!("Total winnings: {}", total_winnings);
}

fn compare_hands(line1: &(&str, &str), line2: &(&str, &str)) -> std::cmp::Ordering {
    let hand1: Vec<char> = line1.0.chars().collect();
    let hand2: Vec<char> = line2.0.chars().collect();

    let type1 = get_type(hand1.clone());
    let type2 = get_type(hand2.clone());

    if type1 > type2 {
        return std::cmp::Ordering::Greater;
    } else if type1 < type2 {
        return std::cmp::Ordering::Less;
    }

    for (card1, card2) in hand1.iter().zip(hand2.iter()) {
        let value1 = card_value(card1);
        let value2 = card_value(card2);
        if value1 > value2 {
            return std::cmp::Ordering::Greater;
        } else if value1 < value2 {
            return std::cmp::Ordering::Less;
        }
    }
    return std::cmp::Ordering::Equal;
}

fn get_type(hand: Vec<char>) -> u32 {
    let mut counts: HashMap<char, usize> = HashMap::new();

    for card in hand {
        *counts.entry(card).or_insert(0) += 1;
    }

    let mut found_three = false;
    let mut found_two = false;
    for count in counts.values().sorted_unstable().rev() {
        if *count == 5 {
            return 7;
        } else if *count == 4 {
            return 6;
        } else if *count == 3 {
            found_three = true;
        } else if *count == 2 {
            if found_three {
                return 5;
            } else if found_two {
                return 3;
            } else {
                found_two = true;
            }
        } else if found_three {
            return 4;
        } else if found_two {
            return 2;
        } else {
            return 1;
        }
    }
    0
}

fn card_value(card: &char) -> u32 {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        _ => card.to_digit(10).unwrap(),
    }
}
