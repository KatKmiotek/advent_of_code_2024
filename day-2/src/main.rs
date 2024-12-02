use std::fs::read_to_string;

fn main() {
    let mut safe_reports = 0;
    for line in read_to_string("./src/input.txt").unwrap().lines() {
        let row_numbers: Vec<i64> = line
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
        let is_valid = check_next_numbers(&row_numbers);
        if is_valid {
            safe_reports += 1;
        }
    }
    println!("safe reports number is {}", safe_reports);
}

fn check_next_numbers(numbers: &Vec<i64>) -> bool {
    numbers.windows(2).all(|pair| {
        let diff = (pair[1] - pair[0]).abs();
        diff == 1 || diff == 2
    })
}
