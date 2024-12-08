use std::{collections::HashMap, fs};

#[derive(Debug, PartialEq)]
struct Position {
    x: usize,
    y: usize,
}

fn antinode_count_distance(
    antennas: &HashMap<char, Vec<Position>>,
    width: usize,
    height: usize,
) -> usize {
    let mut antinodes: Vec<bool> = vec![false; width * height];
    let mut antinode_count = 0;
    for (_, positions) in antennas {
        for pos1 in positions {
            for pos2 in positions {
                if pos1 == pos2 {
                    continue;
                }
                let delta_x: isize = pos2.x as isize - pos1.x as isize;
                let delta_y: isize = pos2.y as isize - pos1.y as isize;

                let new = (pos1.x as isize - delta_x, pos1.y as isize - delta_y);

                let position = (
                    if new.0 >= 0 {
                        Some(new.0 as usize)
                    } else {
                        None
                    },
                    if new.1 >= 0 {
                        Some(new.1 as usize)
                    } else {
                        None
                    },
                );

                if let (Some(x), Some(y)) = position {
                    let index = y * width + x;
                    if x < width && y < height && !antinodes[index] {
                        antinode_count += 1;
                        antinodes[index] = true;
                    }
                }
            }
        }
    }

    antinode_count
}

fn antinode_count(antennas: &HashMap<char, Vec<Position>>, width: usize, height: usize) -> usize {
    let mut antinodes: Vec<bool> = vec![false; width * height];
    let mut antinode_count = 0;
    for (_, positions) in antennas {
        for pos1 in positions {
            for pos2 in positions {
                if pos1 == pos2 {
                    continue;
                }

                let mut current_x = pos1.x;
                let mut current_y = pos1.y;
                let delta_x: isize = pos2.x as isize - pos1.x as isize;
                let delta_y: isize = pos2.y as isize - pos1.y as isize;
                let mut valid_position = true;
                if !antinodes[pos1.y * width + pos1.x] {
                    antinode_count += 1;
                    antinodes[pos1.y * width + pos1.x] = true;
                }

                while valid_position {
                    let new = (current_x as isize - delta_x, current_y as isize - delta_y);

                    let position = (
                        if new.0 >= 0 {
                            Some(new.0 as usize)
                        } else {
                            None
                        },
                        if new.1 >= 0 {
                            Some(new.1 as usize)
                        } else {
                            None
                        },
                    );

                    if let (Some(x), Some(y)) = position {
                        current_x = x;
                        current_y = y;
                        let index = y * width + x;
                        if x < width && y < height {
                            if !antinodes[index] {
                                antinode_count += 1;
                                antinodes[index] = true;
                            }
                        } else {
                            valid_position = false;
                        }
                    } else {
                        valid_position = false;
                    }
                }
            }
        }
    }

    antinode_count
}

fn main() {
    const INPUT_FILE: &str = "inputs/input.txt";
    let contents = fs::read_to_string(INPUT_FILE).expect("Unable to read file");
    let width = contents.lines().next().unwrap().chars().count();
    let height = contents.lines().count();

    let mut antennas: HashMap<char, Vec<Position>> = HashMap::new();

    for (y, line) in contents.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            match char {
                '.' => continue,
                character => antennas
                    .entry(character)
                    .or_insert(Vec::new())
                    .push(Position { x, y }),
            }
        }
    }

    let antinode_count_distance = antinode_count_distance(&antennas, width, height);
    let antinode_count = antinode_count(&antennas, width, height);

    println!("Antinode Count factoring in distance: {antinode_count_distance}\nAntinode Count without distance: {antinode_count}");
}
