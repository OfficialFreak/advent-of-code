use std::{fs, num::TryFromIntError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Wall,
    Box,
    Empty,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PartTwoTile {
    Wall,
    LeftBox,
    RightBox,
    Empty,
}

#[derive(Debug)]
enum Instruction {
    Up,
    Down,
    Left,
    Right,
}

impl Instruction {
    fn as_delta(&self) -> (i32, i32) {
        match self {
            Self::Up => (0, -1),
            Self::Down => (0, 1),
            Self::Left => (-1, 0),
            Self::Right => (1, 0),
        }
    }
}

fn display_map(map: &Vec<Tile>, width: &usize, robot_position: &(i32, i32)) {
    let mut buffer = String::new();
    for (i, tile) in map.iter().enumerate() {
        if i % width == 0 {
            buffer += "\n";
        }
        if ((i % width) as i32, (i / width) as i32) == *robot_position {
            buffer += "@";
        } else {
            buffer += match tile {
                Tile::Box => "O",
                Tile::Wall => "#",
                Tile::Empty => ".",
            }
        }
    }
    println!("{}", buffer);
}

fn p2_display_map(map: &Vec<PartTwoTile>, width: &usize, robot_position: &(i32, i32)) {
    let mut buffer = String::new();
    for (i, tile) in map.iter().enumerate() {
        if i % width == 0 {
            buffer += "\n";
        }
        if ((i % width) as i32, (i / width) as i32) == *robot_position {
            buffer += "@";
        } else {
            buffer += match tile {
                PartTwoTile::LeftBox => "[",
                PartTwoTile::RightBox => "]",
                PartTwoTile::Wall => "#",
                PartTwoTile::Empty => ".",
            }
        }
    }
    println!("{}", buffer);
}

fn is_valid_move(
    robot_position: &(i32, i32),
    delta: &(i32, i32),
    map: &Vec<Tile>,
    width: &usize,
) -> bool {
    let new_position = (robot_position.0 + delta.0, robot_position.1 + delta.1);
    let index: Result<usize, TryFromIntError> =
        (new_position.1 * *width as i32 + new_position.0).try_into();
    let object_at_position = match index {
        Ok(index) => map[index],
        Err(_) => Tile::Wall,
    };
    // Check if instruction is valid
    match object_at_position {
        Tile::Wall => false,
        Tile::Empty => true,
        Tile::Box => is_valid_move(&new_position, delta, map, width),
    }
}

fn p2_is_valid_move(
    robot_position: &(i32, i32),
    delta: &(i32, i32),
    map: &Vec<PartTwoTile>,
    width: &usize,
) -> bool {
    let new_position = (robot_position.0 + delta.0, robot_position.1 + delta.1);
    let index: Result<usize, TryFromIntError> =
        (new_position.1 * *width as i32 + new_position.0).try_into();
    let object_at_position = match index {
        Ok(index) => map[index],
        Err(_) => PartTwoTile::Wall,
    };
    // Check if instruction is valid
    match object_at_position {
        PartTwoTile::Wall => false,
        PartTwoTile::Empty => true,
        PartTwoTile::LeftBox if delta.0 == 0 => {
            p2_is_valid_move(&new_position, delta, map, width)
                && p2_is_valid_move(&(new_position.0 + 1, new_position.1), delta, map, width)
        }
        PartTwoTile::LeftBox if delta.0 == 1 => {
            p2_is_valid_move(&(new_position.0 + 1, new_position.1), delta, map, width)
        }
        PartTwoTile::LeftBox if delta.0 == -1 => p2_is_valid_move(&new_position, delta, map, width),
        PartTwoTile::RightBox if delta.0 == 0 => {
            p2_is_valid_move(&new_position, delta, map, width)
                && p2_is_valid_move(&(new_position.0 - 1, new_position.1), delta, map, width)
        }
        PartTwoTile::RightBox if delta.0 == -1 => {
            p2_is_valid_move(&(new_position.0 - 1, new_position.1), delta, map, width)
        }
        PartTwoTile::RightBox if delta.0 == 1 => p2_is_valid_move(&new_position, delta, map, width),
        _ => panic!("Something weird happened with the deltas"),
    }
}

fn move_object(
    robot_position: &(i32, i32),
    delta: &(i32, i32),
    map: &mut Vec<PartTwoTile>,
    width: &usize,
) {
    let goal_position = (robot_position.0 + delta.0, robot_position.1 + delta.1);
    let old_index: usize = (robot_position.1 * *width as i32 + robot_position.0)
        .try_into()
        .unwrap();
    let goal_index: usize = (goal_position.1 * *width as i32 + goal_position.0)
        .try_into()
        .unwrap();
    // Check if instruction is valid
    match map[goal_index] {
        PartTwoTile::Wall => panic!("Cannot move object into a wall"),
        PartTwoTile::Empty => (),
        PartTwoTile::LeftBox if delta.0 != 1 => {
            move_object(&goal_position, delta, map, width);
            move_object(&(goal_position.0 + 1, goal_position.1), delta, map, width);
        }
        PartTwoTile::LeftBox if delta.0 == 1 => {
            move_object(&(goal_position.0 + 1, goal_position.1), delta, map, width);
            move_object(&goal_position, delta, map, width);
        }
        PartTwoTile::RightBox if delta.0 != -1 => {
            move_object(&goal_position, delta, map, width);
            move_object(&(goal_position.0 - 1, goal_position.1), delta, map, width);
        }
        PartTwoTile::RightBox if delta.0 == -1 => {
            move_object(&(goal_position.0 - 1, goal_position.1), delta, map, width);
            move_object(&goal_position, delta, map, width);
        }
        _ => panic!("Unreachable"),
    }

    assert_eq!(map[goal_index], PartTwoTile::Empty);
    map[goal_index] = map[old_index];
    map[old_index] = PartTwoTile::Empty;
}

fn main() {
    const INPUT_FILE: &str = "inputs/input.txt";
    let contents = fs::read_to_string(INPUT_FILE).expect("Unable to read file");
    let mut split = contents.split("\n\n");
    let map_src = split.next().unwrap();
    let instruction_src = split.next().unwrap();

    let height = map_src.lines().count();
    let width = map_src.lines().next().unwrap().chars().count();

    let mut map: Vec<Tile> = Vec::with_capacity(width * height);
    let mut instructions: Vec<Instruction> = Vec::new();
    let mut robot_position: (i32, i32) = (0, 0);

    // Not using enumerate to skip carriage returns and newlines
    let mut i = 0;
    for char in map_src.chars() {
        match char {
            '#' => map.push(Tile::Wall),
            'O' => map.push(Tile::Box),
            '.' => map.push(Tile::Empty),
            '@' => {
                map.push(Tile::Empty);
                robot_position = (i % width as i32, i / width as i32)
            }
            _ => i -= 1,
        }
        i += 1;
    }
    for char_instructions in instruction_src.lines() {
        for instruction in char_instructions.chars() {
            match instruction {
                '^' => instructions.push(Instruction::Up),
                'v' => instructions.push(Instruction::Down),
                '<' => instructions.push(Instruction::Left),
                '>' => instructions.push(Instruction::Right),
                _ => (),
            }
        }
    }

    for instruction in &instructions {
        let delta = instruction.as_delta();
        if !is_valid_move(&robot_position, &delta, &map, &width) {
            continue;
        }
        let mut new_position = (robot_position.0 + delta.0, robot_position.1 + delta.1);
        let mut index: usize = (new_position.1 * width as i32 + new_position.0)
            .try_into()
            .unwrap();
        let moved_box = map[index] == Tile::Box;
        map[index] = Tile::Empty;
        robot_position = new_position.clone();

        if moved_box {
            new_position = (new_position.0 + delta.0, new_position.1 + delta.1);
            index = (new_position.1 * width as i32 + new_position.0)
                .try_into()
                .unwrap();
            while map[index] == Tile::Box {
                new_position = (new_position.0 + delta.0, new_position.1 + delta.1);
                index = (new_position.1 * width as i32 + new_position.0)
                    .try_into()
                    .unwrap();
            }

            map[index] = Tile::Box;
        }
    }

    // Get the sum of the GPS Coordinates of all boxes
    let mut sum = 0;
    for (i, tile) in map.iter().enumerate() {
        if *tile == Tile::Box {
            let x = i % width;
            let y = i / width;
            sum += y * 100 + x;
        }
    }

    display_map(&map, &width, &robot_position);
    println!("Sum: {sum}");

    // Part Two

    let mut map: Vec<PartTwoTile> = Vec::with_capacity(width * 2 * height);
    let mut robot_position: (i32, i32) = (0, 0);
    let width = width * 2;

    // Not using enumerate to skip carriage returns and newlines
    let mut i = 0;
    for char in map_src.chars() {
        match char {
            '#' => {
                map.push(PartTwoTile::Wall);
                map.push(PartTwoTile::Wall)
            }
            'O' => {
                map.push(PartTwoTile::LeftBox);
                map.push(PartTwoTile::RightBox)
            }
            '.' => {
                map.push(PartTwoTile::Empty);
                map.push(PartTwoTile::Empty)
            }
            '@' => {
                map.push(PartTwoTile::Empty);
                map.push(PartTwoTile::Empty);
                robot_position = (i % width as i32, i / width as i32)
            }
            _ => i -= 2,
        }
        i += 2;
    }

    for instruction in instructions {
        let delta = instruction.as_delta();
        if !p2_is_valid_move(&robot_position, &delta, &map, &width) {
            continue;
        }

        // Execute instruction
        move_object(&robot_position, &delta, &mut map, &width);
        robot_position = (robot_position.0 + delta.0, robot_position.1 + delta.1);
    }

    // Get the sum of the GPS Coordinates of all boxes
    let mut sum = 0;
    for (i, tile) in map.iter().enumerate() {
        if *tile == PartTwoTile::LeftBox {
            let x = i % width;
            let y = i / width;
            sum += y * 100 + x;
        }
    }

    p2_display_map(&map, &width, &robot_position);
    println!("Part two sum: {sum}");
}
