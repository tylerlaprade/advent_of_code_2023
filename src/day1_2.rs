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
            println!("Line: {}", line);
            let (first, last) = first_and_last_digit(line);
            let number = format!("{}{}", first, last).parse::<i32>().unwrap();
            println!("Adding: {}", number);
            sum += number;
        }
    }
    println!("Sum: {}", sum);
}

fn first_and_last_digit(s: &str) -> (char, char) {
    let digit_names = [
        ("zero", '0'),
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ];

    let mut digits = Vec::new();
    let mut current_word = String::new();

    for c in s.chars() {
        if c.is_ascii_digit() {
            digits.push(c);
            current_word.clear();
        } else if c.is_alphabetic() {
            current_word.push(c);
            for &(name, digit) in &digit_names {
                if current_word.ends_with(name) {
                    digits.push(digit);
                    break;
                }
            }
        }
    }

    (digits[0], *digits.last().unwrap())
}
