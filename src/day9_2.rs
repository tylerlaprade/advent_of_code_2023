pub fn run() {
    let data = std::fs::read("inputs/9.txt").unwrap();
    let text = std::str::from_utf8(&data).unwrap();
    let lines = text.lines().map(str::trim);
    //Print sum
    println!(
        "Total sum: {}",
        lines
            .clone()
            .map(|line| process_line(line))
            .fold(0, |acc, item| acc + item)
    );
}

fn process_line(line: &str) -> i32 {
    let mut diffs: Vec<i32> = line
        .split_whitespace()
        .map(|item| item.parse::<i32>().unwrap())
        .collect();
    let mut sum = 0;
    let mut polarity = 1;
    loop {
        println!("{:?}", diffs);
        let first = diffs.clone()[0];
        if diffs.iter().all(|&item| item == 0) {
            let extrapolated_value = sum;
            println!("Extrapolated value: {}", extrapolated_value);
            return extrapolated_value;
        }
        sum += polarity * first;
        let mut new_diffs: Vec<i32> = Vec::new();
        for i in 0..diffs.len() - 1 {
            let new_diff = diffs[i + 1] - diffs[i];
            new_diffs.push(new_diff);
        }

        diffs = new_diffs;
        polarity = -polarity;
    }
}
