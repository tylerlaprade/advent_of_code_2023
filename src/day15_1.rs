pub fn run() {
    let data = std::fs::read("inputs/15.txt").unwrap();
    let text = std::str::from_utf8(&data).unwrap();
    let line = text.lines().next().unwrap();
    let steps = line.split(",");
    let mut sum = 0;
    for step in steps {
        sum += hash(step)
    }
    println!("Sum: {}", sum);
}

fn hash(step: &str) -> u64 {
    let mut value = 0;
    for c in step.chars() {
        value += c as u64;
        value *= 17;
        value %= 256;
    }
    return value;
}
