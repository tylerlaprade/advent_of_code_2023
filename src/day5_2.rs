use colored::*;
use std::ops::Range;

pub fn run() {
    let data = std::fs::read("inputs/5.txt").unwrap();
    let text = std::str::from_utf8(&data).unwrap();
    let mut lines = text.lines().map(str::trim_end);
    let seed_values: Vec<i64> = lines
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    let mut seeds: Vec<Range<i64>> = seed_values
        .chunks(2)
        .map(|chunk| chunk[0]..(chunk[0] + chunk[1] - 1))
        .collect();
    lines.next();
    loop {
        lines.next();
        let group: Vec<&str> = lines.by_ref().take_while(|line| !line.is_empty()).collect();
        if group.is_empty() {
            break;
        }
        let mut new_seeds = Vec::new();
        for line in group {
            println!("{}", format!("Line: {}", line).bright_red());
            let parts: Vec<i64> = line
                .split_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect();
            let (destination_start, source_start, range) = (parts[0], parts[1], parts[2]);
            let offset = source_start - destination_start;
            let mapping_range_end = source_start + range - 1;
            println!("{}", format!("Destination start: {}, source start: {}, range: {}, offset: {}, mapping range end: {}", destination_start, source_start, range, offset, mapping_range_end).bright_blue());
            let mut remaining_seeds = Vec::new();
            for seed in seeds {
                if seed.start >= source_start && seed.end < mapping_range_end {
                    println!("Fully contained");
                    new_seeds.push((seed.start - offset)..(seed.end - offset));
                } else if seed.start < source_start && seed.end >= mapping_range_end {
                    println!("In between");
                    remaining_seeds.push(seed.start..source_start);
                    new_seeds.push((source_start - offset)..(mapping_range_end + 1 - offset));
                    remaining_seeds.push((mapping_range_end + 1)..seed.end);
                } else if seed.start < source_start && seed.end >= source_start {
                    println!("Start overlap");
                    remaining_seeds.push(seed.start..source_start);
                    new_seeds.push(source_start-offset..(seed.end - offset));
                } else if seed.start < mapping_range_end && seed.end >= mapping_range_end {
                    println!("End overlap");
                    new_seeds.push((seed.start - offset)..mapping_range_end + 1 - offset);
                    remaining_seeds.push((mapping_range_end + 1)..seed.end);
                } else {
                    println!("No overlap");
                    remaining_seeds.push(seed);
                }
            }
            seeds = remaining_seeds;
        }
        seeds.append(&mut new_seeds);
        seeds.sort_by_key(|seed| seed.start);
        seeds = merge_overlapping_ranges(seeds);
        println!("{}", format!("Seeds: {:?}", seeds).bright_green());
    }
    println!(
        "Minimal location: {}",
        seeds.iter().map(|seed| seed.start).min().unwrap()
    );
}

fn merge_overlapping_ranges(mut ranges: Vec<Range<i64>>) -> Vec<Range<i64>> {
    ranges.sort_by_key(|r| r.start);
    let mut result = Vec::new();
    let mut current = ranges[0].clone();
    for range in ranges.into_iter().skip(1) {
        if range.start <= current.end {
            current.end = current.end.max(range.end);
        } else {
            result.push(current);
            current = range;
        }
    }
    result.push(current);
    result
}
