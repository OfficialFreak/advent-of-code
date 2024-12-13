use regex::Regex;
use std::fs;

fn main() {
    const INPUT_FILE: &str = "inputs/input.txt";
    let contents = fs::read_to_string(INPUT_FILE).expect("Unable to read file");
    let re = Regex::new(
        r"Button A: X\+(\d*)\, Y\+(\d*)\nButton B: X\+(\d*)\, Y\+(\d*)\nPrize\: X=(\d*)\, Y=(\d*)",
    )
    .unwrap();

    let mut tokens_spent = 0.0;
    let mut tokens_spent_conversion_error = 0.0;
    for capture in re.captures_iter(contents.as_str()) {
        let ax: f64 = capture[1].parse().unwrap();
        let ay: f64 = capture[2].parse().unwrap();
        let bx: f64 = capture[3].parse().unwrap();
        let by: f64 = capture[4].parse().unwrap();
        let rx: f64 = capture[5].parse().unwrap();
        let ry: f64 = capture[6].parse().unwrap();

        let rx_conversion_error = capture[5].parse::<f64>().unwrap() + 10000000000000f64;
        let ry_conversion_error = capture[6].parse::<f64>().unwrap() + 10000000000000f64;

        // How many times we need to press a / b
        let a = (-bx * ry + by * rx) / (ax * by - bx * ay);
        let b = (ax * ry - ay * rx) / (ax * by - bx * ay);

        let a_conversion_error =
            (-bx * ry_conversion_error + by * rx_conversion_error) / (ax * by - bx * ay);
        let b_conversion_error =
            (ax * ry_conversion_error - ay * rx_conversion_error) / (ax * by - bx * ay);

        if a >= 0.0 && a.fract() == 0.0 && b >= 0.0 && b.fract() == 0.0 {
            // Cost of A: 3
            // Cost of B: 1
            tokens_spent += a * 3.0 + b;
        }
        if a_conversion_error >= 0.0
            && a_conversion_error.fract() == 0.0
            && b_conversion_error >= 0.0
            && b_conversion_error.fract() == 0.0
        {
            // Cost of A: 3
            // Cost of B: 1
            tokens_spent_conversion_error += a_conversion_error * 3.0 + b_conversion_error;
        }
    }
    println!("Tokens spent: {tokens_spent}\nConversion error tokens spent: {tokens_spent_conversion_error}");
}
