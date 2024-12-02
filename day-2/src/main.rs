use std::fs::read_to_string;

fn main() {
    let mut safe_reports = 0;
    for line in read_to_string("./src/input.txt").unwrap().lines() {
        let row_numbers: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
        let is_valid = check_next_numbers(&row_numbers);
        if is_valid {
            safe_reports += 1;
            println!("valid row: {:?}", row_numbers)
        }
    }
    println!("safe reports number is {}", safe_reports);
}

fn check_next_numbers(numbers: &Vec<i32>) -> bool {
    let difference_between_numbers = numbers.windows(2).all(|pair| {
        let diff = (pair[1] - pair[0]).abs();
        match diff {
            1 => true,
            2 => true,
            3 => true,
            _ => false,
        }
    });
    if !difference_between_numbers || numbers.len() < 2 {
        return difference_between_numbers;
    }

    let direction = numbers[1] - numbers[0];
    let goes_up = direction > 0;

    numbers.windows(2).all(|pair| {
        let diff = pair[1] - pair[0];
        if goes_up {
            diff > 0
        } else {
            diff < 0
        }
    })
}
