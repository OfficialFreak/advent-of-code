use itertools::Itertools;
use std::fs;

fn concat(a: u64, b: u64) -> u64 {
    a * 10u64.pow(b.ilog10() + 1) + b
}

fn get_sums(equations: &Vec<(u64, Vec<u64>)>, operations: Vec<fn(u64, u64) -> u64>) -> u64 {
    let mut sum: u64 = 0;
    for equation in equations {
        // Loop over all possible combinations of operations

        'sequence: for operation_sequence in std::iter::repeat(&operations)
            .take(equation.1.len() - 1)
            .multi_cartesian_product()
        {
            let mut tmp: u64 = equation.1[0];
            for (i, operation) in operation_sequence.iter().enumerate() {
                if tmp > equation.0 {
                    continue 'sequence;
                }
                tmp = operation(tmp, equation.1[i + 1]);
            }
            if tmp == equation.0 {
                sum += tmp;
                break;
            }
        }
        // If the equation was only one number, check for equality
        if equation.1.len() == 1 && equation.0 == equation.1[0] {
            sum += equation.0;
        }
    }

    sum
}

fn main() {
    const INPUT_FILE: &str = "inputs/input.txt";
    let contents = fs::read_to_string(INPUT_FILE).expect("Unable to read file");
    let mut equations: Vec<(u64, Vec<u64>)> = Vec::new();
    for line in contents.lines() {
        let split: Vec<&str> = line.split(":").collect();
        let result: u64 = split[0].parse().expect("Invalid number found");
        let parts: Vec<u64> = split[1]
            .trim_start()
            .split(" ")
            .map(|num| num.parse().expect("Invalid number found"))
            .collect();
        equations.push((result, parts));
    }

    let without_concat = get_sums(
        &equations,
        vec![|a: u64, b: u64| a * b, |a: u64, b: u64| a + b],
    );

    let with_concat = get_sums(
        &equations,
        vec![|a: u64, b: u64| a * b, |a: u64, b: u64| a + b, concat],
    );

    println!("Sum of all possible equations: {without_concat}\nSum with concat: {with_concat}");
}
