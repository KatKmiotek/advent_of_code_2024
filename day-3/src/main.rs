use std::fs::read_to_string;

use regex::Regex;

fn main() {
    let line = read_to_string("./src/input.txt").unwrap();
    let pattern = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let matches: Vec<&str> = pattern.find_iter(&line).map(|m| m.as_str()).collect();
    println!("number of matches {:?}", matches.len());
    let mut total = 0;
    for item in &matches {
        let trimmed = item.replace("mul", "").replace("(", "").replace(")", "");
        println!("trimmed {}", trimmed);
        let numbers: Vec<i32> = trimmed
            .split(',')
            .filter_map(|n| n.trim().parse().ok())
            .collect();
        total += numbers[0] * numbers[1]
    }

    println!("total is {}", total)
}
