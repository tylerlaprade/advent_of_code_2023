use itertools::Itertools;

pub fn run() {
    let data = std::fs::read("inputs/17.txt").unwrap();
    let lines = std::str::from_utf8(&data).unwrap().lines();
    let mut grid = vec![vec!['.'; 1]; 1];
    let mut position = (0, 0);
    let mut filled_area = 0;
    for line in lines {
        let (direction, distance, _color) = line
            .split_ascii_whitespace()
            .into_iter()
            .collect_tuple()
            .unwrap();
        let distance = distance.parse::<usize>().unwrap();
        filled_area += distance;
        match direction {
            "U" => {
                for _i in 0..distance {
                    if position.1 == 0 {
                        grid.insert(0, vec!['.'; grid[0].len()]);
                    } else {
                        position.1 -= 1;
                    }
                    grid[position.1][position.0] = '#';
                }
            }
            "D" => {
                for _i in 0..distance {
                    if position.1 == grid.len() - 1 {
                        grid.push(vec!['.'; grid[0].len()]);
                    }
                    position.1 += 1;
                    grid[position.1][position.0] = '#';
                }
            }
            "L" => {
                for _i in 0..distance {
                    if position.0 == 0 {
                        for row in &mut grid {
                            row.insert(0, '.');
                        }
                    } else {
                        position.0 -= 1;
                    }
                    grid[position.1][position.0] = '#';
                }
            }
            "R" => {
                for _i in 0..distance {
                    if position.0 == grid[0].len() - 1 {
                        for row in &mut grid {
                            row.push('.');
                        }
                    }
                    position.0 += 1;
                    grid[position.1][position.0] = '#';
                }
            }
            _ => panic!("Unknown direction"),
        }
    }
    let mut stack = Vec::new();
    'outer: for (y, line) in grid.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c == '#' {
                stack.push((x + 1, y + 1));
                break 'outer;
            }
        }
    }
    while let Some((x, y)) = stack.pop() {
        if grid[y][x] == '.' {
            filled_area += 1;
            grid[y][x] = 'O';
            stack.push((x, y - 1));
            stack.push((x, y + 1));
            stack.push((x - 1, y));
            stack.push((x + 1, y));
        }
    }
    println!("Area: {}", filled_area)
}
