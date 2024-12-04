use std::fs;

use regex::Regex;

fn find_xmas_occurence(contents: &str, width: usize, height: usize) -> usize {
    const WORD: &str = "XMAS";

    println!("{width}x{height} grid detected");

    // Generate all the substrings that would be valid (e.g. horizontal, vertical, diagonal and all of them reversed)
    let horizontal: Vec<String> = contents
        .lines()
        .map(|string| String::from(string))
        .collect();

    let mut vertical: Vec<String> = vec![String::new(); width];
    for line in contents.lines() {
        for (i, char) in line.chars().enumerate() {
            vertical[i].push(char);
        }
    }

    let mut diagonal_ltr: Vec<String> = vec![String::new(); height + width - 1];
    let mut diagonal_rtl: Vec<String> = vec![String::new(); height + width - 1];
    for x in 0..width {
        for (y, line) in contents.lines().enumerate() {
            diagonal_rtl[x + y].push_str(&line[x..x + 1]);
            diagonal_ltr[((x as isize) - (y as isize) + (height as isize) - 1) as usize]
                .push_str(&line[x..x + 1]);
        }
    }

    let horizontal_reversed: Vec<String> = horizontal
        .iter()
        .map(|string| string.chars().rev().collect::<String>())
        .collect();
    let vertical_reversed: Vec<String> = vertical
        .iter()
        .map(|string| string.chars().rev().collect::<String>())
        .collect();
    let diagonal_ltr_reversed: Vec<String> = diagonal_ltr
        .iter()
        .map(|string| string.chars().rev().collect::<String>())
        .collect();
    let diagonal_rtl_reversed: Vec<String> = diagonal_rtl
        .iter()
        .map(|string| string.chars().rev().collect::<String>())
        .collect();

    let all_strings = [
        horizontal,
        horizontal_reversed,
        vertical,
        vertical_reversed,
        diagonal_ltr,
        diagonal_ltr_reversed,
        diagonal_rtl,
        diagonal_rtl_reversed,
    ];

    let pattern = regex::escape(WORD);
    let re = Regex::new(&pattern).unwrap();

    let mut overall_sum = 0;
    for collection in all_strings {
        for string in collection {
            overall_sum += re.find_iter(string.as_str()).count();
        }
    }

    overall_sum
}

fn find_crossmas_occurences(contents: &str, width: usize, height: usize) -> usize {
    let mut overall_sum = 0;
    let lines: Vec<&str> = contents.lines().collect();

    for x in 0..width - 2 {
        for y in 0..height - 2 {
            let diag_ltr = format!(
                "{}{}{}",
                &lines[y][x..x + 1],
                &lines[y + 1][x + 1..x + 2],
                &lines[y + 2][x + 2..x + 3]
            );
            let diag_rtl = format!(
                "{}{}{}",
                &lines[y][x + 2..x + 3],
                &lines[y + 1][x + 1..x + 2],
                &lines[y + 2][x..x + 1]
            );

            if (diag_ltr == "MAS" || diag_ltr == "SAM") && (diag_rtl == "MAS" || diag_rtl == "SAM")
            {
                overall_sum += 1;
            }
        }
    }

    overall_sum
}

fn main() {
    const INPUT_FILE: &str = "inputs/input.txt";
    let contents = fs::read_to_string(INPUT_FILE).expect("Unable to read file");

    let width = contents.lines().next().unwrap().len();
    let height = contents.lines().count();

    let overall_sum = find_xmas_occurence(contents.as_str(), width, height);
    let crossmas_occurences = find_crossmas_occurences(contents.as_str(), width, height);

    println!("Overall sum: {overall_sum}\nCross-MAS Occurences: {crossmas_occurences}");
}
