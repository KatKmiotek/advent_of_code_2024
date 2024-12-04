use std::fs::read_to_string;

use ndarray::Array2;
use regex::Regex;

fn main() {
    let data = read_to_string("./src/input.txt").unwrap();
    let lines: Vec<&str> = data.lines().collect();
    let table_width = lines[0].len();
    let table_length = lines.len();
    let chars: Vec<char> = lines.iter().flat_map(|line| line.chars()).collect();

    let grid: Array2<char> = Array2::from_shape_vec((table_length, table_width), chars).unwrap();
    let horizontal_forward = search_horizontal(&grid, "XMAS");
    let horizontal_backward = search_horizontal(&grid, "SAMX");
    println!(
        "Horizontal: forward={}, backward={}",
        horizontal_forward, horizontal_backward
    );

    let vertical_forward = search_vertical(&grid, "XMAS");
    let vertical_backward = search_vertical(&grid, "SAMX");
    println!(
        "Vertical: forward={}, backward={}",
        vertical_forward, vertical_backward
    );

    let total = horizontal_forward + horizontal_backward + vertical_forward + vertical_backward;
    println!("total: {}", total);
}

fn search_horizontal(grid: &Array2<char>, pattern: &str) -> usize {
    let re = Regex::new(&pattern).unwrap();
    let mut total_matches = 0;

    for row in grid.rows() {
        let row_string: String = row.iter().collect();
        total_matches += re.find_iter(&row_string).count();
    }
    total_matches
}

fn search_vertical(grid: &Array2<char>, pattern: &str) -> usize {
    let re = Regex::new(&pattern).unwrap();

    let mut total_matches = 0;

    for column in grid.columns() {
        let column_string: String = column.iter().collect();
        total_matches += re.find_iter(&column_string).count();
    }
    total_matches
}
