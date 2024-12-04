use std::fs;

use regex::Regex;

fn multiply_all(input: &str) -> u32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut sum: u32 = 0;

    for capture in re.captures_iter(input) {
        sum += capture[1].parse::<u32>().unwrap() * capture[2].parse::<u32>().unwrap();
        // Capture group 0 is the entire pattern, 1 and 2 the capture groups
        // println!(
        //     "{} {} {}",
        //     capture[0].parse::<String>().unwrap(),
        //     capture[1].parse::<String>().unwrap(),
        //     capture[2].parse::<String>().unwrap()
        // );
    }

    sum
}

fn conditional_multiply(input: &str) -> u32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();
    let mut sum: u32 = 0;
    let mut active = true;

    for capture in re.captures_iter(input) {
        if &capture[0] == "do()" {
            active = true;
        } else if &capture[0] == "don't()" {
            active = false;
        } else if active {
            sum += &capture[1].parse::<u32>().unwrap() * &capture[2].parse::<u32>().unwrap();
        }
    }

    sum
}

fn main() {
    const INPUT_FILE: &str = "inputs/input.txt";
    let contents = fs::read_to_string(INPUT_FILE).expect("Unable to read file");

    println!(
        "Unconditional: {}\nConditional: {}",
        multiply_all(contents.as_str()),
        conditional_multiply(contents.as_str())
    );
}
