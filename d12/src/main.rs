use std::{collections::VecDeque, fs};

#[derive(PartialEq, Eq, Debug)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl Direction {
    fn from_tuple(tup: &(isize, isize)) -> Direction {
        match tup {
            (0, -1) => Direction::UP,
            (0, 1) => Direction::DOWN,
            (-1, 0) => Direction::LEFT,
            (1, 0) => Direction::RIGHT,
            _ => panic!("Invalid tuple provided"),
        }
    }

    fn get_orthogonal(direction: &Direction) -> [(isize, isize); 2] {
        match direction {
            Direction::UP => [(-1, 0), (1, 0)],
            Direction::DOWN => [(-1, 0), (1, 0)],
            Direction::LEFT => [(0, -1), (0, 1)],
            Direction::RIGHT => [(0, -1), (0, 1)],
        }
    }
}

fn to_index(x: &usize, y: &usize, width: &usize) -> usize {
    y * width + x
}

fn main() {
    const INPUT_FILE: &str = "inputs/input.txt";
    // const INPUT_FILE: &str = "inputs/test.txt";
    let contents: Vec<Vec<char>> = fs::read_to_string(INPUT_FILE)
        .expect("Unable to read file")
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let height = contents.len();
    let width = contents[0].len();

    let mut explored: Vec<bool> = vec![false; width * height];
    let mut fencing_region_price = 0;
    let mut new_fencing_region_price = 0;

    for y in 0..height {
        for x in 0..width {
            if explored[to_index(&x, &y, &width)] {
                continue;
            }

            // Explore region
            const DIRECTIONS: [(isize, isize); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];
            let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
            let current_region = contents[x][y];
            let mut current_area = 0;
            let mut current_perimiter = 0;
            let mut current_sides = 0;
            queue.push_back((x, y));

            // TODO: Keep track of borders and only add to "sides" if new border doesn't extend any previous borders
            let mut explored_borders: Vec<(Direction, (isize, isize))> = Vec::new();

            while !queue.is_empty() {
                match queue.pop_front() {
                    Some((curr_x, curr_y)) => {
                        if explored[to_index(&curr_x, &curr_y, &width)] {
                            continue;
                        }
                        current_area += 1;
                        explored[to_index(&curr_x, &curr_y, &width)] = true;
                        current_perimiter += 4;
                        for direction in DIRECTIONS {
                            let wall_direction = Direction::from_tuple(&direction);
                            let neighbor_pos = (
                                curr_x.checked_add_signed(direction.0),
                                curr_y.checked_add_signed(direction.1),
                            );

                            let mut invalid_coordinates = false;
                            let x_negative = match neighbor_pos.0 {
                                Some(new_x) if new_x < width => false,
                                None => {
                                    invalid_coordinates = true;
                                    true
                                }
                                _ => {
                                    invalid_coordinates = true;
                                    false
                                }
                            };

                            let y_negative = match neighbor_pos.1 {
                                Some(new_y) if new_y < height => false,
                                None => {
                                    invalid_coordinates = true;
                                    true
                                }
                                _ => {
                                    invalid_coordinates = true;
                                    false
                                }
                            };

                            // Check if there's a wall there
                            // Either we are in bounds aka. valid coordiantes OR we're at the negative border OR the positive border
                            let wall = if !invalid_coordinates {
                                let neighbor_pos: (usize, usize) = (
                                    neighbor_pos.0.unwrap().try_into().unwrap(),
                                    neighbor_pos.1.unwrap().try_into().unwrap(),
                                );

                                if contents[neighbor_pos.0][neighbor_pos.1] != current_region {
                                    // Wall
                                    Some((neighbor_pos.0 as isize, neighbor_pos.1 as isize))
                                } else {
                                    None
                                }
                            } else {
                                // Wall (because of the region border)
                                let tmp_x = if x_negative {
                                    -1
                                } else {
                                    match neighbor_pos.0 {
                                        Some(pos_x) => pos_x as isize,
                                        None => width.try_into().unwrap(),
                                    }
                                };
                                let tmp_y = if y_negative {
                                    -1
                                } else {
                                    match neighbor_pos.1 {
                                        Some(pos_y) => pos_y as isize,
                                        None => height.try_into().unwrap(),
                                    }
                                };
                                Some((tmp_x, tmp_y))
                            };

                            match wall {
                                Some(wall) => {
                                    if !explored_borders.iter().any(|border| {
                                        border.0 == wall_direction && border.1 == wall
                                    }) {
                                        // We know there's a wall, so check for adjacent walls we might've
                                        // already explored
                                        let mut adjacent_walls = 0;
                                        for orth_wall_direction in
                                            Direction::get_orthogonal(&wall_direction)
                                        {
                                            adjacent_walls += explored_borders
                                                .iter()
                                                .filter(|border| {
                                                    border.0 == wall_direction
                                                        && border.1
                                                            == (
                                                                wall.0 + orth_wall_direction.0,
                                                                wall.1 + orth_wall_direction.1,
                                                            )
                                                })
                                                .count()
                                        }

                                        // If we didn't find an adjacent wall, add one to our sides
                                        current_sides += 1 - adjacent_walls;
                                        // Add our wall to explored_borders
                                        explored_borders.push((wall_direction, (wall.0, wall.1)));
                                    }
                                }
                                None => (),
                            }

                            if invalid_coordinates {
                                continue;
                            }

                            let neighbor_pos: (usize, usize) = (
                                neighbor_pos.0.unwrap().try_into().unwrap(),
                                neighbor_pos.1.unwrap().try_into().unwrap(),
                            );

                            if contents[neighbor_pos.0][neighbor_pos.1] == current_region {
                                current_perimiter -= 1;

                                if !explored[to_index(&neighbor_pos.0, &neighbor_pos.1, &width)] {
                                    queue.push_back(neighbor_pos);
                                }
                            }
                        }
                    }
                    None => break,
                };
            }

            fencing_region_price += current_area * current_perimiter;
            new_fencing_region_price += current_area * current_sides;
        }
    }

    println!("Fencing Region Price: {fencing_region_price}\nNew Fencing Region Price: {new_fencing_region_price}");
}
