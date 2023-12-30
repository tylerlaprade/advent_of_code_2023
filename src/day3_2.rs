use std::collections::HashMap;

pub fn run() {
    let data = std::fs::read("inputs/3.txt").unwrap();
    let text = std::str::from_utf8(&data).unwrap();
    let grid: Vec<Vec<char>> = text.lines().map(|line| line.chars().collect()).collect();
    let mut gears: HashMap<(usize, usize), Vec<i32>> = HashMap::new();

    for (i, row) in grid.iter().enumerate() {
        let mut current_number = String::new();
        for (j, c) in row.iter().enumerate() {
            if c.is_ascii_digit() {
                current_number.push(*c);
            } else if !current_number.is_empty() {
                for neighboring_asterisk in get_neighboring_asterisks(
                    &grid,
                    j as isize - current_number.len() as isize,
                    j as isize,
                    i as isize,
                ) {
                    gears
                        .entry(neighboring_asterisk)
                        .or_default()
                        .push(current_number.parse::<i32>().unwrap());
                }
                current_number = String::new();
            }
        }
        if !current_number.is_empty() {
            for neighboring_asterisk in get_neighboring_asterisks(
                &grid,
                row.len() as isize - current_number.len() as isize,
                row.len() as isize,
                i as isize,
            ) {
                gears
                    .entry(neighboring_asterisk)
                    .or_default()
                    .push(current_number.parse::<i32>().unwrap());
            }
        }
    }
    let mut sum = 0;
    for gear in gears.values() {
        if gear.len() == 2 {
            sum += gear.iter().product::<i32>();
        }
    }

    println!("SUM: {}", sum);
}

fn get_neighboring_asterisks(
    grid: &Vec<Vec<char>>,
    start_j: isize,
    end_j: isize,
    mid_i: isize,
) -> Vec<(usize, usize)> {
    let mut res = Vec::new();
    for i in [mid_i - 1, mid_i + 1] {
        if i >= grid.len() as isize || i < 0 {
            continue;
        }
        for j in (start_j - 1)..end_j + 1 {
            if j < 0 || j >= grid[i as usize].len() as isize {
                continue;
            }
            let neighbor = grid[i as usize][j as usize];
            if neighbor == '*' {
                res.push((i as usize, j as usize));
            }
        }
    }
    for j in [start_j - 1, end_j] {
        if j < 0 || j >= grid[mid_i as usize].len() as isize {
            continue;
        }
        let neighbor = grid[mid_i as usize][j as usize];
        if neighbor == '*' {
            res.push((mid_i as usize, j as usize));
        }
    }
    res
}
