use std::{
    collections::{HashSet, VecDeque},
    fs,
};

// pathfinding::directed::bfs could spare me the
// implementation, I want to learn a bit though,
// which is why I'll implement it myself
//
// Edit: This was the right descision as the second part
// requires you to modify BFS to count the paths (we know
// that no cycles can exist in our graph)

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
struct Position {
    x: u8,
    y: u8,
}

impl Position {
    fn as_index(&self, width: &u8) -> usize {
        self.y as usize * *width as usize + self.x as usize
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Node {
    position: Position,
    final_elevation: bool,
}

fn get_trail_scores(
    topographical_map: &Vec<Vec<Node>>,
    trailheads: &Vec<Position>,
    width: &u8,
    height: &u8,
) -> usize {
    let mut trail_count = 0;
    for trailhead in trailheads {
        let mut reachable_goals: HashSet<Node> = HashSet::new();
        let mut queue: VecDeque<Node> = VecDeque::new();
        let mut explored: Vec<bool> = vec![false; *width as usize * *height as usize];
        explored[trailhead.as_index(&width)] = true;
        queue.push_back(Node {
            position: trailhead.clone(),
            final_elevation: false,
        });

        loop {
            match queue.pop_front() {
                Some(v) => {
                    if v.final_elevation {
                        reachable_goals.insert(v.clone());
                    }
                    for w in &topographical_map[v.position.as_index(&width)] {
                        if explored[w.position.as_index(&width)] {
                            continue;
                        }

                        explored[w.position.as_index(&width)] = true;
                        queue.push_back(w.clone());
                    }
                }
                None => break,
            }
        }
        trail_count += reachable_goals.len();
    }

    trail_count
}

fn get_trail_ratings(
    topographical_map: &Vec<Vec<Node>>,
    trailheads: &Vec<Position>,
    width: &u8,
) -> usize {
    let mut trail_count = 0;
    for trailhead in trailheads {
        let mut queue: VecDeque<Node> = VecDeque::new();
        queue.push_back(Node {
            position: trailhead.clone(),
            final_elevation: false,
        });

        loop {
            match queue.pop_front() {
                Some(v) => {
                    if v.final_elevation {
                        trail_count += 1;
                    }
                    for w in &topographical_map[v.position.as_index(&width)] {
                        queue.push_back(w.clone());
                    }
                }
                None => break,
            }
        }
    }

    trail_count
}

fn main() {
    // Find the number of trail paths (-> paths starting at a trailhead that lead to an elevation of 9)
    const INPUT_FILE: &str = "inputs/input.txt";
    let contents = fs::read_to_string(INPUT_FILE).expect("Unable to read file");

    let mut trailheads: Vec<Position> = Vec::new();

    let height: u8 = contents.lines().count().try_into().unwrap();
    let width: u8 = contents
        .lines()
        .next()
        .unwrap()
        .chars()
        .count()
        .try_into()
        .unwrap();
    let mut topographical_map: Vec<Vec<Node>> =
        Vec::with_capacity(width as usize * height as usize);
    let lines: Vec<&str> = contents.lines().collect();

    // Iterate over the input and generate our graph
    const DIRECTIONS: [(i8, i8); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];
    for (y, line) in contents.lines().enumerate() {
        let y: u8 = y as u8;
        for (x, elevation) in line.chars().enumerate() {
            let x: u8 = x as u8;
            let current_position = Position { x, y };
            let elevation: u8 = elevation
                .to_digit(10)
                .expect("Invalid elevation found at {x} {y}") as u8;
            if elevation == 0 {
                trailheads.push(current_position.clone());
            }
            topographical_map.push(Vec::new());
            for direction in DIRECTIONS {
                let neighbor_pos = Position {
                    x: match x.checked_add_signed(direction.0) {
                        Some(new_x) if new_x < width => new_x,
                        _ => continue,
                    },
                    y: match y.checked_add_signed(direction.1) {
                        Some(new_y) if new_y < height => new_y,
                        _ => continue,
                    },
                };

                let neighboring_elevation: u8 = lines
                    .get(neighbor_pos.y as usize)
                    .unwrap()
                    .chars()
                    .nth(neighbor_pos.x as usize)
                    .unwrap()
                    .to_digit(10)
                    .expect("Invalid elevation found at {neighbor_pos.0} {neighbor_pos.1}")
                    as u8;

                if neighboring_elevation == elevation + 1 {
                    // Valid next move found
                    topographical_map.last_mut().unwrap().push(Node {
                        position: neighbor_pos,
                        final_elevation: neighboring_elevation == 9,
                    });
                }
            }
        }
    }

    let trail_scores = get_trail_scores(&topographical_map, &trailheads, &width, &height);
    let trail_ratings = get_trail_ratings(&topographical_map, &trailheads, &width);

    println!("Trail scores: {trail_scores}\nTrail ratings: {trail_ratings}");
}
