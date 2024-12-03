use regex::Regex;
use std::fs::read_to_string;

fn main() {
    let line = read_to_string("./src/input.txt").unwrap();
    task_one(&line);
    task_two(line);
}

fn task_one(line: &String) {
    let pattern = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let matches: Vec<&str> = pattern.find_iter(&line).map(|m| m.as_str()).collect();
    println!("number of matches {:?}", matches.len());
    let mut total = 0;
    for item in &matches {
        total += trim(item);
    }

    println!("total is {}", total)
}

fn task_two(line: String) {
    let dont_pattern = Regex::new(r"don't\(\)").unwrap();
    let do_pattern = Regex::new(r"do\(\)").unwrap();
    let mul_pattern = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let all_donts: Vec<&str> = dont_pattern.find_iter(&line).map(|m| m.as_str()).collect();
    let mut matches: Vec<&str> = dont_pattern.splitn(&line, all_donts.len() + 1).collect();
    let mut total: i64 = 0;
    let start_dos: Vec<&str> = mul_pattern
        .find_iter(matches[0])
        .map(|m| m.as_str())
        .collect();
    for item in start_dos {
        total += trim(item)
    }
    matches.remove(0);
    for string_piece in &matches {
        let matches: Vec<&str> = do_pattern.splitn(&string_piece, 2).collect();
        if matches.len() > 1 {
            let only_dos: Vec<&str> = mul_pattern
                .find_iter(matches[1])
                .map(|m| m.as_str())
                .collect();

            for item in only_dos {
                total += trim(item);
            }
        }
    }

    println!("total {}", total)
}

fn trim(item: &str) -> i64 {
    let trimmed = item.replace("mul", "").replace("(", "").replace(")", "");
    println!("trimmed {}", trimmed);
    let numbers: Vec<i64> = trimmed
        .split(',')
        .filter_map(|n| n.trim().parse().ok())
        .collect();
    let total: i64 = numbers[0] * numbers[1];
    total
}
