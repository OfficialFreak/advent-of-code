use std::{fs, usize};

#[derive(Debug)]
struct File {
    id: Option<usize>,
    blocks: u32,
}

fn get_fragmented_checksum(diskmap: &Vec<char>) -> i128 {
    let mut disk: Vec<i32> = Vec::new();

    for i in (0..diskmap.len()).step_by(2) {
        let blocksize: u32 = diskmap[i].to_digit(10).unwrap();
        let free_space: u32 = if i == diskmap.len() - 1 {
            '0'
        } else {
            diskmap[i + 1]
        }
        .to_digit(10)
        .unwrap();
        for _ in 0..blocksize {
            disk.push(i as i32 / 2);
        }
        for _ in 0..free_space {
            disk.push(-1);
        }
    }

    let mut next_free_space = 0;
    'outer: for i in (0..disk.len()).rev() {
        if disk[i] == -1 {
            continue;
        }
        for j in next_free_space..disk.len() {
            if j > i {
                break 'outer;
            }
            if disk[j] == -1 {
                next_free_space = j;
                break;
            }
        }
        disk[next_free_space] = disk[i];
        disk[i] = -1;
    }

    let mut sum: i128 = 0;
    for (i, id) in disk.iter().enumerate() {
        if *id == -1 {
            break;
        }
        sum += (i as i128) * (*id as i128);
    }

    sum
}

fn get_unfragmented_checksum(diskmap: &Vec<char>) -> i128 {
    let mut disk: Vec<File> = Vec::new();

    // Initial disk population
    for i in (0..diskmap.len()).step_by(2) {
        let blocksize: u32 = diskmap[i].to_digit(10).unwrap();
        let free_space: u32 = if i == diskmap.len() - 1 {
            '0'
        } else {
            diskmap[i + 1]
        }
        .to_digit(10)
        .unwrap();

        disk.push(File {
            blocks: blocksize,
            id: Some(i / 2),
        });
        disk.push(File {
            blocks: free_space,
            id: None,
        });
    }

    'outer: for move_to_front in (0..disk.len()).rev() {
        if disk[move_to_front].id.is_none() {
            continue;
        }

        let mut next_free_space_idx = None;
        for (free_idx, potentially_free) in (&disk).iter().enumerate() {
            if potentially_free.id.is_some() {
                continue;
            }
            if move_to_front < free_idx {
                continue 'outer;
            }
            if let Some(file_id) = disk[move_to_front].id {
                if let Some(free_id) = disk[free_idx].id {
                    if free_id > file_id {
                        continue 'outer;
                    }
                }
                if disk[free_idx].blocks >= disk[move_to_front].blocks {
                    next_free_space_idx = Some(free_idx);
                    break;
                }
            }
        }

        if let Some(free_idx) = next_free_space_idx {
            let current_blocks = disk[move_to_front].blocks;
            let current_id = disk[move_to_front].id;

            if disk[free_idx].blocks == current_blocks {
                // If it fits exactly
                disk[free_idx].id = current_id;
                disk[move_to_front].id = None;
            } else {
                // If we need to split
                disk[free_idx].blocks -= current_blocks;
                disk[move_to_front].id = None;

                // Insert new used file before the free space
                disk.insert(
                    free_idx,
                    File {
                        id: current_id,
                        blocks: current_blocks,
                    },
                );
            }
        }
    }

    let mut idx = 0;
    let mut sum = 0;
    for file in &disk {
        for _ in 0..file.blocks {
            if let Some(file_id) = file.id {
                sum += (idx as i128) * (file_id as i128);
            }
            idx += 1;
        }
    }

    sum
}

fn main() {
    const INPUT_FILE: &str = "inputs/input.txt";
    let contents = fs::read_to_string(INPUT_FILE).expect("Unable to read file");

    let diskmap: Vec<char> = contents.trim().chars().collect();

    let fragmented_sum = get_fragmented_checksum(&diskmap);
    let unfragmented_sum = get_unfragmented_checksum(&diskmap);

    println!("Fragmented Sum: {fragmented_sum}\nUnfragmented Sum: {unfragmented_sum}");
}
