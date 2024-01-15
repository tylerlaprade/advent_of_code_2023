pub fn run() {
    let data = std::fs::read("inputs/10.txt").unwrap();
    let text = std::str::from_utf8(&data).unwrap();
    let grid = text.lines().map(format_line).collect::<Vec<Vec<char>>>();
    let start_position = find_start_position(&grid);
    let mut left_right_positions = vec![];
    for direction in [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ] {
        let potential_start = step_along_pipe(&grid, start_position, direction);
        match potential_start {
            Ok(_) => {
                left_right_positions.push(potential_start.unwrap());
            }
            Err(_) => {}
        }
    }
    let mut left_right_iter = left_right_positions.into_iter();
    let mut left_position = left_right_iter.next().unwrap();
    let mut right_position = left_right_iter.next().unwrap();
    let first_left_direction = &left_position.2;
    let first_right_direction = &right_position.2;
    let actual_start_tile = match (first_left_direction, first_right_direction) {
        (Direction::Up, Direction::Down) | (Direction::Down, Direction::Up) => '|',
        (Direction::Up, Direction::Left) | (Direction::Left, Direction::Up) => 'J',
        (Direction::Down, Direction::Left) | (Direction::Left, Direction::Down) => '7',
        (Direction::Up, Direction::Right) | (Direction::Right, Direction::Up) => 'L',
        (Direction::Down, Direction::Right) | (Direction::Right, Direction::Down) => 'F',
        _ => panic!("Invalid start tile"),
    };
    let mut loop_tiles = HashSet::new();
    loop_tiles.insert((start_position.0, start_position.1));
    loop {
        loop_tiles.insert((left_position.0, left_position.1));
        loop_tiles.insert((right_position.0, right_position.1));
        if left_position.0 == right_position.0 && left_position.1 == right_position.1 {
            break;
        }
        left_position =
            step_along_pipe(&grid, (left_position.0, left_position.1), left_position.2).unwrap();
        right_position = step_along_pipe(
            &grid,
            (right_position.0, right_position.1),
            right_position.2,
        )
        .unwrap();
    }
    println!("Loop tiles: {:?}", loop_tiles);
    let mut inside_count = 0;
    for (y, line) in grid.iter().enumerate() {
        let mut is_inside = false;
        let mut twist_terminator = 'X';
        for (x, mut character) in line.iter().enumerate() {
            if character == &'S' {
                character = &actual_start_tile;
            }
            if loop_tiles.contains(&(x, y)) {
                if &twist_terminator == character {
                    is_inside = !is_inside;
                } else {
                    match character {
                        '|' => {
                            is_inside = !is_inside;
                        }
                        'F' => {
                            twist_terminator = 'J';
                        }
                        'L' => {
                            twist_terminator = '7';
                        }
                        _ => {}
                    }
                }
            } else if is_inside {
                println!("Inside: {} {}", y, x);
                inside_count += 1;
            }
        }
    }
    println!("Inside count: {}", inside_count);
}

fn format_line(line: &str) -> Vec<char> {
    line.chars().collect::<Vec<char>>()
}

fn find_start_position(grid: &Vec<Vec<char>>) -> (usize, usize) {
    for (y, line) in grid.iter().enumerate() {
        for (x, character) in line.iter().enumerate() {
            if *character == 'S' {
                return (x, y);
            }
        }
    }
    panic!("No start position found");
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

use std::{collections::HashSet, error::Error};

fn step_along_pipe(
    grid: &Vec<Vec<char>>,
    start_position: (usize, usize),
    direction: Direction,
) -> Result<(usize, usize, Direction), Box<dyn Error>> {
    let mut x = start_position.0 as isize;
    let mut y = start_position.1 as isize;
    match direction {
        Direction::Up => y -= 1,
        Direction::Down => y += 1,
        Direction::Left => x -= 1,
        Direction::Right => x += 1,
    }
    if x < 0 || y < 0 {
        return Err("Out of bounds".into());
    }
    let x = x as usize;
    let y = y as usize;
    if y >= grid.len() || x >= grid[y].len() {
        return Err("Out of bounds".into());
    }
    Ok((
        x,
        y,
        match (grid[y][x], &direction) {
            ('|' | '-', _) => direction,
            ('L', Direction::Down) | ('F', Direction::Up) => Direction::Right,
            ('J', Direction::Down) | ('7', Direction::Up) => Direction::Left,
            ('L', Direction::Left) | ('J', Direction::Right) => Direction::Up,
            ('F', Direction::Left) | ('7', Direction::Right) => Direction::Down,
            (character, _) => return Err(format!("Unknown character: {}", character).into()),
        },
    ))
}
