use std::fs::read_to_string;

fn main() {
    let lookup_line = read_to_string("./src/lookup.txt").unwrap();
    let mut rules = Vec::new();

    for line in lookup_line.lines() {
        let parts: Vec<&str> = line.split('|').collect();
        let before = parts[0].parse::<i32>().unwrap();
        let after = parts[1].parse::<i32>().unwrap();
        rules.push((before, after));
    }

    let updates = read_to_string("./src/input.txt")
        .unwrap()
        .lines()
        .map(|line| {
            line.split(',')
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<_>>();

    let valid_updates: Vec<_> = updates
        .clone()
        .into_iter()
        .filter(|update| is_valid_update(update, &rules))
        .collect();
    println!("number of valid updates: {}", valid_updates.len());

    let invalid_updates: Vec<_> = updates
        .into_iter()
        .filter(|update| !is_valid_update(update, &rules))
        .collect();
    println!("number of invalid updates: {}", invalid_updates.len());

    let mut fixed_invalid_updates: Vec<Vec<i32>> = Vec::new();
    for mut update in invalid_updates {
        update.sort();
        fixed_invalid_updates.push(update);
    }

    let total_from_valid: i32 = valid_updates
        .iter()
        .map(|update| update[update.len() / 2])
        .sum();

    let total_from_invalid: i32 = fixed_invalid_updates
        .iter()
        .map(|update| update[update.len() / 2])
        .sum();

    println!("Total from valid is: {}", total_from_valid);
    println!("Total from invalid is: {}", total_from_invalid);
}

fn is_valid_update(update: &[i32], rules: &[(i32, i32)]) -> bool {
    for (before, after) in rules {
        let before_pos = update.iter().position(|&x| x == *before);
        let after_pos = update.iter().position(|&x| x == *after);

        match (before_pos, after_pos) {
            (Some(pos1), Some(pos2)) => {
                if pos1 > pos2 {
                    return false;
                }
            }
            _ => continue,
        }
    }
    true
}
