use std::fs;

#[derive(PartialEq, Eq)]
enum Direction {
    ASCENDING,
    DESCENDING,
}

fn report_safe(report: Vec<u32>, dampening_active: bool) -> bool {
    let mut previous_level: u32 = report[0];
    let direction: Direction = if report[0] < report[1] {
        Direction::ASCENDING
    } else {
        Direction::DESCENDING
    };

    for &level in report[1..].iter() {
        let difference = level.abs_diff(previous_level);

        if difference < 1
            || difference > 3
            || (level > previous_level && direction == Direction::DESCENDING)
            || (level < previous_level && direction == Direction::ASCENDING)
        {
            if dampening_active {
                // Remove each level and check if it is safe
                for i in 0..(report.len()) {
                    let report_without_level: Vec<_> = report
                        .iter()
                        .enumerate()
                        .filter(|(index, _)| *index != i)
                        .map(|(_, element)| *element)
                        .collect();
                    // Check if safe without dampening
                    if report_safe(report_without_level, false) {
                        return true;
                    }
                }
            }
            return false;
        }
        previous_level = level;
    }
    true
}

fn main() {
    const INPUT_FILE: &str = "inputs/input.txt";

    let contents = fs::read_to_string(INPUT_FILE).expect("Unable to read file.");
    let mut safe_count: u32 = 0;
    for line in contents.lines() {
        let is_safe = report_safe(
            line.split(" ")
                .map(|string| string.parse().expect("Invalid input"))
                .collect(),
            false,
        );
        if is_safe {
            safe_count += 1;
        }
    }
    println!("Safe count: {safe_count}");

    let mut safe_count_dampened: u32 = 0;
    for line in contents.lines() {
        let is_safe = report_safe(
            line.split(" ")
                .map(|string| string.parse().expect("Invalid input"))
                .collect(),
            true,
        );
        if is_safe {
            safe_count_dampened += 1;
        }
    }
    println!("Safe count (dampened): {safe_count_dampened}");
}
