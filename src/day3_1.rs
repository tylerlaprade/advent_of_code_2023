pub fn run() {
    let data = std::fs::read("inputs/3.txt").unwrap();
    let text = std::str::from_utf8(&data).unwrap();
    let grid: Vec<Vec<char>> = text.lines().map(|line| line.chars().collect()).collect();
    let mut sum = 0;

    for (i, row) in grid.iter().enumerate() {
        let mut current_number = String::new();
        for (j, c) in row.iter().enumerate() {
            if c.is_ascii_digit() {
                current_number.push(*c);
            } else if !current_number.is_empty() {
                if has_neighboring_symbol(
                    &grid,
                    j as isize - current_number.len() as isize,
                    j as isize,
                    i as isize,
                ) {
                    let parsed_number: i32 = current_number.parse().unwrap();
                    sum += parsed_number;
                }
                current_number = String::new();
            }
        }
        if !current_number.is_empty() && has_neighboring_symbol(
                &grid,
                row.len() as isize - current_number.len() as isize,
                row.len() as isize,
                i as isize,
            ) {
            let parsed_number: i32 = current_number.parse().unwrap();
            sum += parsed_number;
        }
    }

    println!("SUM: {}", sum);
}

fn has_neighboring_symbol(
    grid: &Vec<Vec<char>>,
    start_j: isize,
    end_j: isize,
    mid_i: isize,
) -> bool {
    for i in [mid_i - 1, mid_i + 1] {
        if i >= grid.len() as isize || i < 0 {
            continue;
        }
        for j in (start_j - 1)..end_j + 1 {
            if j < 0 || j >= grid[i as usize].len() as isize {
                continue;
            }
            let neighbor = grid[i as usize][j as usize];
            if neighbor != '.' && !neighbor.is_ascii_digit() {
                if mid_i == 119 {
                    println!("TOP: i: {} j: {} neighbor: {}", i, j, neighbor);
                }
                return true;
            }
        }
    }
    for j in [start_j - 1, end_j] {
        if j < 0 || j >= grid[mid_i as usize].len() as isize {
            continue;
        }
        let neighbor = grid[mid_i as usize][j as usize];
        if neighbor != '.' && !neighbor.is_alphanumeric() {
            if mid_i == 119 {
                println!("BOT: i: {} j: {} neighbor: {}", mid_i, j, neighbor);
            }
            return true;
        }
    }
    // println!("start_j: {} end_j: {} mid_i: {}", start_j, end_j, mid_i);
    false
}
