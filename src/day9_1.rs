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
    loop {
        println!("{:?}", diffs);
        let last = diffs.clone().pop().unwrap();
        if diffs.iter().all(|&item| item == 0) {
            return sum;
        }
        sum += last;
        let mut new_diffs: Vec<i32> = Vec::new();
        for i in 0..diffs.len() - 1 {
            new_diffs.push(diffs[i + 1] - diffs[i]);
        }

        diffs = new_diffs;
    }
}
