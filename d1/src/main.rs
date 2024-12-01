use std::{collections::HashMap, fs};

fn get_difference(input_file: &str) -> i32 {
    let line_count = fs::read_to_string(input_file).expect("Unable to read file").lines().count();
    let contents = fs::read_to_string(input_file).expect("Unable to read file");

    let mut left_nums: Vec<i32> = vec![0; line_count];
    let mut right_nums: Vec<i32> = vec![0; line_count];

    let mut sum: i32 = 0;
    for (i, line) in contents.lines().enumerate() {
        let split_line: Vec<&str> = line.split("   ").collect();
        let left_num: i32 = split_line[0].parse().unwrap();
        let right_num: i32 = split_line[1].parse().unwrap();
        left_nums[i] = left_num;
        right_nums[i] = right_num;
    }

    left_nums.sort();
    right_nums.sort();

    for i in 0..line_count {
        sum += (left_nums[i] - right_nums[i]).abs();
    }

    sum
}

fn get_simmilarity(input_file: &str) -> usize {
    let line_count = fs::read_to_string(input_file).expect("Unable to read file").lines().count();
    let contents = fs::read_to_string(input_file).expect("Unable to read file");

    let mut left_nums: Vec<usize> = vec![0; line_count];
    let mut right_nums: HashMap<usize, usize> = HashMap::new();

    let mut sum: usize = 0;
    for (i, line) in contents.lines().enumerate() {
        let split_line: Vec<&str> = line.split("   ").collect();
        let left_num: usize = split_line[0].parse().unwrap();
        let right_num: usize = split_line[1].parse().unwrap();
        left_nums[i] = left_num;
        let num = right_nums.entry(right_num).or_insert(0);
        *num += 1;
    }

    for i in 0..line_count {
        sum += left_nums[i] * right_nums.get(&(left_nums[i] as usize)).unwrap_or(&0);
    }

    sum
}

fn main() {
    const INPUT_FILE: &str = "inputs/input.txt";
    let difference = get_difference(INPUT_FILE);
    let simmilarity = get_simmilarity(INPUT_FILE);

    println!("Difference: {difference}\nSimmilarity: {simmilarity}");
}
