pub fn run() {
    let data = std::fs::read("inputs/1.txt").unwrap();
    let text = std::str::from_utf8(&data).unwrap();
    let mut lines = text.lines().map(str::trim_end);
    let mut sum = 0;
    loop {
        let group: Vec<_> = lines.by_ref().take_while(|line| !line.is_empty()).collect();
        if group.is_empty() {
            break;
        }
        for line in &group {
            let digits: Vec<_> = line.chars().filter(|c| c.is_ascii_digit()).collect();
            if let (Some(&first), Some(&last)) = (digits.first(), digits.last()) {
                let number = format!("{}{}", first, last).parse::<i32>().unwrap();
                sum += number;
            } else {
                panic!("No digits detected!");
            }
        }
    }
    println!("Sum: {}", sum);
}
