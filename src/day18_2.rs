use crate::day18_shared::dig_trenches;

pub fn run() {
    dig_trenches(parse_line);
}

fn parse_line(_direction: &str, _distance: &str, color: &str) -> (String, isize) {
    let mut chars = color.chars();
    let distance = chars.by_ref().skip(2).take(5).collect::<String>();
    let direction_char = chars.next();

    (
        match direction_char {
            Some('0') => "R".to_string(),
            Some('1') => "D".to_string(),
            Some('2') => "L".to_string(),
            Some('3') => "U".to_string(),
            _ => panic!("Unknown direction: {}", direction_char.unwrap_or('X')),
        },
        isize::from_str_radix(&distance, 16).unwrap(),
    )
}
