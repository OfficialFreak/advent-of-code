use std::fs;

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
enum Direction {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

impl Direction {
    fn to_delta(&self) -> (isize, isize) {
        match self {
            Self::Up => (0, -1),
            Self::Right => (1, 0),
            Self::Down => (0, 1),
            Self::Left => (-1, 0),
        }
    }

    fn turned_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn turn_right(&mut self) {
        *self = self.turned_right();
    }
}

#[derive(Debug, Clone, Copy)]
struct Guard {
    position: (usize, usize),
    direction: Direction,
}

fn in_bounds(x: usize, y: usize, width: usize, height: usize) -> bool {
    x < width && y < height
}

#[derive(Clone, Debug)]
struct Tile {
    visited: [bool; 4],
    wall: bool,
}

#[derive(PartialEq)]
enum Result {
    LoopDetected,
    ExitedGrid(usize),
}

fn run_simulation(mut guard: Guard, mut grid: Vec<Tile>, width: usize, height: usize) -> Result {
    let mut visited_count = 1;

    loop {
        // Step
        let direction_delta = guard.direction.to_delta();
        let new_position = (
            (guard.position.0 as isize) + direction_delta.0,
            (guard.position.1 as isize) + direction_delta.1,
        );

        let x: usize;
        let y: usize;
        match new_position.0.try_into() {
            Ok(val) => x = val,
            Err(_) => break,
        }
        match new_position.1.try_into() {
            Ok(val) => y = val,
            Err(_) => break,
        }

        if !in_bounds(x, y, width, height) {
            break;
        };

        if grid[y * width + x].wall {
            guard.direction.turn_right();
        } else {
            guard.position = (x, y);
            // Detect Loop
            if grid[guard.position.1 * width + guard.position.0].visited[guard.direction as usize] {
                return Result::LoopDetected;
            }
            if !grid[y * width + x].visited.contains(&true) {
                visited_count += 1;
                grid[y * width + x].visited[guard.direction as usize] = true;
            }
        }
    }
    Result::ExitedGrid(visited_count)
}

fn main() {
    const INPUT_FILE: &str = "inputs/input.txt";
    let contents = fs::read_to_string(INPUT_FILE).expect("Unable to read file");
    let height = contents.lines().count();
    let width = contents.lines().next().unwrap().chars().count();

    let mut grid: Vec<Tile> = vec![
        Tile {
            visited: [false; 4],
            wall: false
        };
        width * height
    ];
    let mut guard = Guard {
        position: (0, 0),
        direction: Direction::Up,
    };

    for (y, line) in contents.lines().enumerate() {
        // Search for the guards initial position
        // Search for all of the walls
        for (x, char) in line.chars().enumerate() {
            match char {
                '#' => {
                    // Wall found
                    grid[y * width + x].wall = true;
                }
                '^' => {
                    // Guard position found
                    grid[y * width + x].visited[0] = true;
                    guard.position = (x, y);
                    guard.direction = Direction::Up
                }
                '>' => {
                    // Guard position found
                    grid[y * width + x].visited[1] = true;
                    guard.position = (x, y);
                    guard.direction = Direction::Right
                }
                'v' => {
                    // Guard position found
                    grid[y * width + x].visited[2] = true;
                    guard.position = (x, y);
                    guard.direction = Direction::Down
                }
                '<' => {
                    // Guard position found
                    grid[y * width + x].visited[3] = true;
                    guard.position = (x, y);
                    guard.direction = Direction::Left
                }
                _ => (),
            }
        }
    }

    let result = run_simulation(guard.clone(), grid.clone(), width.clone(), height.clone());

    match result {
        Result::ExitedGrid(count) => println!("Visited count: {count}"),
        Result::LoopDetected => println!("Loop Detected"),
    };

    // Loop over the entire grid and if there's not a wall or the starting
    // position of the guard, place a wall and simulate and check if there's a loop

    let mut stuck_count = 0;
    for i in 0..width * height - 1 {
        if !grid[i].wall && i != guard.position.1 * width + guard.position.0 {
            let mut new_grid = grid.clone();
            new_grid[i].wall = true;
            if run_simulation(guard.clone(), new_grid, width.clone(), height.clone())
                == Result::LoopDetected
            {
                stuck_count += 1;
            }
        }
    }
    println!("Found {stuck_count} new wall positions for the guard to get stuck in.");
}
