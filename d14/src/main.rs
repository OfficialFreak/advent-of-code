use std::fs;

use regex::Regex;

#[derive(Debug, Clone)]
struct Robot {
    position: (i32, i32),
    velocity: (i32, i32),
}

impl Robot {
    fn apply_velocity(&mut self, seconds: usize, width: &usize, height: &usize) {
        self.position = (
            (self.position.0 + (self.velocity.0 * seconds as i32)).rem_euclid(*width as i32),
            (self.position.1 + (self.velocity.1 * seconds as i32)).rem_euclid(*height as i32),
        )
    }
}

fn get_quadrant(final_position: &(i32, i32), width: &usize, height: &usize) -> Option<usize> {
    if final_position.0 < (width / 2) as i32 {
        if final_position.1 < (height / 2) as i32 {
            Some(0)
        } else if final_position.1 > (height / 2) as i32 {
            Some(1)
        } else {
            None
        }
    } else if final_position.0 > (width / 2) as i32 {
        if final_position.1 < (height / 2) as i32 {
            Some(2)
        } else if final_position.1 > (height / 2) as i32 {
            Some(3)
        } else {
            None
        }
    } else {
        None
    }
}

fn has_line_of_robots(robots: &Vec<Robot>) -> bool {
    // Heuristic based on the actual tree
    // (image contains border so I just check for that)
    let mut horizontal_lines: [usize; 103] = [0; 103];
    let mut vertical_lines: [usize; 101] = [0; 101];
    for robot in robots {
        vertical_lines[robot.position.0 as usize] += 1;
        horizontal_lines[robot.position.1 as usize] += 1;
    }

    let mut robot_lines = 0;
    for line in horizontal_lines {
        if line > 31 {
            robot_lines += 1;
        }
    }
    for line in vertical_lines {
        if line > 33 {
            robot_lines += 1;
        }
    }

    robot_lines == 4
}

fn get_safety_factor(robots: Vec<Robot>, width: &usize, height: &usize) -> usize {
    let mut quadrants: [usize; 4] = [0, 0, 0, 0];
    for mut robot in robots {
        // Simulate robot for 100s
        robot.apply_velocity(100, &width, &height);
        // Check which quadrant robot is in and add to count
        match get_quadrant(&robot.position, &width, &height) {
            Some(quadrant) => quadrants[quadrant] += 1,
            None => (),
        };
    }

    quadrants[0] * quadrants[1] * quadrants[2] * quadrants[3]
}

fn display_grid(robots: &Vec<Robot>, width: &usize, height: &usize) {
    let mut output_buffer = String::new();
    for y in 0..*width {
        for x in 0..*height {
            if robots
                .iter()
                .any(|robot| robot.position.0 == (x as i32) && robot.position.1 == (y as i32))
            {
                output_buffer += "█";
            } else {
                output_buffer += " ";
            }
        }
        output_buffer += "\n";
    }
    println!("{}", output_buffer);
}

fn main() {
    const INPUT_FILE: &str = "inputs/input.txt";
    const WIDTH: usize = 101;
    const HEIGHT: usize = 103;
    let contents = fs::read_to_string(INPUT_FILE).expect("Unable to read file");
    let num_robots = contents.lines().count();
    let mut robots = Vec::with_capacity(num_robots);
    let re = Regex::new(r"p=(-?\d+)\,(-?\d+) v=(-?\d+)\,(-?\d+)").unwrap();
    for capture in re.captures_iter(contents.as_str()) {
        let robot = Robot {
            position: (capture[1].parse().unwrap(), capture[2].parse().unwrap()),
            velocity: (capture[3].parse().unwrap(), capture[4].parse().unwrap()),
        };

        robots.push(robot);
    }
    let safety_factor = get_safety_factor(robots.clone(), &WIDTH, &HEIGHT);

    let mut i = 1;
    loop {
        for robot in &mut robots {
            robot.apply_velocity(1, &WIDTH, &HEIGHT);
        }

        // Original solution had rough heuristics to check for the tree
        // and didn't terminate when a tree was found but let me press
        // enter until i found the tree
        if has_line_of_robots(&robots) {
            display_grid(&robots, &WIDTH, &HEIGHT);
            println!("Seconds: {i}");
            break;
        }

        i += 1;
    }
    println!("Safety Factor: {safety_factor}");
}
