pub fn run() {
    let data = std::fs::read("inputs/14.txt").unwrap();
    let text = std::str::from_utf8(&data).unwrap();
    let lines: Vec<Vec<char>> = text.lines().map(|line| line.chars().collect()).collect();
    let line_count = lines.len();
    let mut columns: Vec<Vec<char>> = vec![vec![]; lines[0].len()];

    for line in lines {
        for (i, character) in line.into_iter().enumerate() {
            columns[i].push(character);
        }
    }

    let mut total_load = 0;
    for column in columns {
        let mut power = line_count;

        for (i, character) in column.iter().enumerate() {
            match character {
                'O' => {
                    total_load += power;
                    power -= 1;
                }
                '#' => {
                    power = line_count - i - 1;
                }
                _ => {}
            }
        }
    }
    println!("Total load: {}", total_load)
}
