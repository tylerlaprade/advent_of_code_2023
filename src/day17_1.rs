use crate::day17_shared::dig_trenches;

pub fn run() {
    dig_trenches(parse_line);
}

fn parse_line(direction: &str, distance: &str, _color: &str) -> (String, usize) {
    (direction.to_string(), distance.parse::<usize>().unwrap())
}
