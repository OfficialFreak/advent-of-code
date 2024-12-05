use std::{cmp::Ordering, collections::HashMap, fs};

fn get_middle_number(update: &Vec<u32>) -> u32 {
    // We assume all updates contain an odd number of pages
    assert_ne!(update.len() % 2, 0);

    update[update.len() / 2]
}

fn get_correct_middle_nums(contents: &str) -> u32 {
    let mut rules: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut updates: Vec<Vec<u32>> = Vec::new();

    let mut updates_section = false;
    for line in contents.lines() {
        if line == "" {
            updates_section = true;
            continue;
        }
        if updates_section {
            updates.push(
                line.split(",")
                    .map(|num| num.parse().expect("Found invalid number"))
                    .collect(),
            );
        } else {
            // 97|75 -> 75: [97] aka. welche Zahlen dürfen keine Nachfolger sein
            // Ich gucke mir eine Zahl an z.B. 75 und checke in der Hashmap auf Nachfolger die in meinem Array enthalten sind
            // Wenn ja: Konflikt
            let rule: Vec<u32> = line
                .split("|")
                .map(|num| num.parse().expect("Found invalid number"))
                .collect();
            assert_eq!(rule.len(), 2);
            rules.entry(rule[1]).or_insert(Vec::new()).push(rule[0]);
        }
    }

    let mut sum = 0;
    'updates: for update in &updates {
        for (i, num) in update.iter().enumerate() {
            // Check if any of the successors to our number are contained in our HashMap which would be a conflict
            if update[i + 1..].iter().any(|x| match rules.get(num) {
                Some(arr) => arr.contains(x),
                None => false,
            }) {
                // Conflict
                continue 'updates;
            }
        }
        sum += get_middle_number(update);
    }

    sum
}

fn get_incorrect_middle_nums(contents: &str) -> u32 {
    let mut rules: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut updates: Vec<Vec<u32>> = Vec::new();

    let mut updates_section = false;
    for line in contents.lines() {
        if line == "" {
            updates_section = true;
            continue;
        }
        if updates_section {
            updates.push(
                line.split(",")
                    .map(|num| num.parse().expect("Found invalid number"))
                    .collect(),
            );
        } else {
            let rule: Vec<u32> = line
                .split("|")
                .map(|num| num.parse().expect("Found invalid number"))
                .collect();
            assert_eq!(rule.len(), 2);
            rules.entry(rule[1]).or_insert(Vec::new()).push(rule[0]);
        }
    }

    let mut sum = 0;
    'updates: for update in &updates {
        for (i, num) in update.iter().enumerate() {
            // Check if any of the successors to our number are contained in our HashMap which would be a conflict
            if update[i + 1..].iter().any(|x| match rules.get(num) {
                Some(arr) => arr.contains(x),
                None => false,
            }) {
                // Conflict
                let mut new_update = update.clone();
                new_update.sort_by(|a, b| match rules.get(a) {
                    Some(arr) => {
                        if arr.contains(b) {
                            Ordering::Less
                        } else {
                            Ordering::Equal
                        }
                    }
                    None => Ordering::Equal,
                });
                sum += get_middle_number(&new_update);
                continue 'updates;
            }
        }
    }

    sum
}

fn main() {
    const INPUT_FILE: &str = "inputs/input.txt";
    let contents = fs::read_to_string(INPUT_FILE).expect("Unable to read file");

    let correct_sum = get_correct_middle_nums(&contents);
    let incorrect_sum = get_incorrect_middle_nums(&contents);

    println!("Correct Sum: {correct_sum}\nIncorrect Sum: {incorrect_sum}");
}
