use std::collections::HashMap;
use std::fs;
use std::time::Instant;

fn digit_even(number: u64) -> bool {
    match u64::checked_ilog10(number) {
        Some(num) => (num + 1) % 2 == 0,
        None => false,
    }
}

fn get_split_engravings(number: u64) -> (u64, u64) {
    let digit_count = (number as f64).log10() as u32 + 1;
    let half_len = digit_count / 2;
    let divisor = 10u64.pow(half_len);

    let right = number % divisor;
    let left = number / divisor;

    (left, right)
}

fn step_stone(stone: u64, num_steps: usize, cache: &mut HashMap<(u64, usize), u64>) -> u64 {
    if let Some(&cached) = cache.get(&(stone, num_steps)) {
        return cached;
    }

    let count = if num_steps == 1 {
        if stone == 0 {
            1
        } else if digit_even(stone) {
            2
        } else {
            1
        }
    } else {
        if stone == 0 {
            step_stone(1, num_steps - 1, cache)
        } else if digit_even(stone) {
            let (left, right) = get_split_engravings(stone);
            step_stone(left, num_steps - 1, cache) + step_stone(right, num_steps - 1, cache)
        } else {
            step_stone(stone * 2024, num_steps - 1, cache)
        }
    };

    cache.insert((stone, num_steps), count);
    count
}

fn main() {
    const INPUT_FILE: &str = "inputs/input.txt";
    let contents = fs::read_to_string(INPUT_FILE).expect("Unable to read file");
    let stones: Vec<u64> = contents
        .trim()
        .split(" ")
        .map(|number| number.parse().expect("Invalid number found {number}"))
        .collect();

    let mut stone_cache = HashMap::new();

    let start = Instant::now();
    let total_stones_25: u64 = stones
        .iter()
        .map(|&stone| step_stone(stone, 25, &mut stone_cache))
        .sum();
    println!("Number of stones after 25 steps: {}", total_stones_25);

    let total_stones_75: u64 = stones
        .iter()
        .map(|&stone| step_stone(stone, 75, &mut stone_cache))
        .sum();
    println!("Number of stones after 75 steps: {}", total_stones_75);

    println!("Total time: {:?}", start.elapsed());
}
