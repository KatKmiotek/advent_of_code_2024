use std::{cmp::Ordering, fs::read_to_string, io::Error};

fn main() -> Result<(), Error> {
    let mut list_one: Vec<i64> = Vec::new();
    let mut list_two: Vec<i64> = Vec::new();
    for line in read_to_string("./src/input.txt").unwrap().lines() {
        let mut full_line = line.split_whitespace();
        let fist_column_value = full_line.next().unwrap().parse::<i64>().unwrap();

        let second_column_value = full_line.next().unwrap().parse::<i64>().unwrap();
        list_one.push(fist_column_value);
        list_two.push(second_column_value);
    }
    list_one.sort();
    list_two.sort();
    let result_one = first_task(list_one.clone(), list_two.clone());
    let result_two = second_task(list_one, list_two);
    println!("result 1 {:?}", result_one);
    println!("result 2 {:?}", result_two);

    Ok(())
}

fn get_balance(number_one: i64, number_two: i64) -> i64 {
    match number_one.cmp(&number_two) {
        Ordering::Greater => number_one - number_two,
        Ordering::Less => number_two - number_one,
        Ordering::Equal => 0,
    }
}
fn second_task(list_one: Vec<i64>, list_two: Vec<i64>) -> i64 {
    let mut result = 0;
    for index in 0..list_one.len() {
        let num_appearance = list_two.iter().filter(|&&n| n == list_one[index]).count();
        let count = list_one[index] * num_appearance as i64;
        result += count
    }
    result
}

fn first_task(list_one: Vec<i64>, list_two: Vec<i64>) -> i64 {
    let mut result = 0;
    for index in 0..list_one.len() {
        let balance = get_balance(list_one[index], list_two[index]);
        println!("balance of {} index is {}", index, balance);
        result += balance
    }
    result
}
