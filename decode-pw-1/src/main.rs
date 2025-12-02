use std::fs;
use std::io::Result;

use cdll::CircularList;

#[derive(Debug)]
enum Direction {
    L,
    R,
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    rotations: u32,
}

fn decode(instruction: &str) -> Instruction {
    let (direction_str, rotations_str) = instruction.split_at(1);
    let direction = match direction_str {
        "L" => Direction::L,
        "R" => Direction::R,
        _ => {
            panic!("Error in parsing the direction!");
        }
    };
    let rotations = rotations_str.parse::<u32>().unwrap();

    Instruction {
        direction,
        rotations,
    }
}

fn read_input(path: &str) -> Result<Vec<String>> {
    let mut result = Vec::new();

    for line in fs::read_to_string(path)?.lines() {
        result.push(line.to_string());
    }

    Ok(result)
}

fn part_1() {
    let mut rotor: CircularList<u32> = CircularList::default();

    for i in 0..100 {
        rotor.add(i);
    }

    let mut cursor = rotor.cursor_mut().unwrap();

    for _ in 0..50 {
        cursor.move_next();
    }

    let instructions = read_input("input.txt");
    let mut trigger_count = 0;

    for raw_instruction in instructions.unwrap() {
        let instruction = decode(raw_instruction.as_ref());
        let rotations = instruction.rotations;
        match instruction.direction {
            Direction::L => {
                for _ in 1..(rotations + 1) {
                    cursor.move_prev();
                }
                if *cursor.value() == 0 {
                    trigger_count += 1;
                }
            }
            Direction::R => {
                for _ in 1..(rotations + 1) {
                    cursor.move_next();
                }
                if *cursor.value() == 0 {
                    trigger_count += 1;
                }
            }
        };
    }

    println!("Password is: {}", trigger_count);
}

fn part_2() {
    let mut rotor: CircularList<u32> = CircularList::default();

    for i in 0..100 {
        rotor.add(i);
    }

    let mut cursor = rotor.cursor_mut().unwrap();

    for _ in 0..50 {
        cursor.move_next();
    }

    let instructions = read_input("input.txt");
    let mut trigger_count = 0;

    for raw_instruction in instructions.unwrap() {
        let instruction = decode(raw_instruction.as_ref());
        let rotations = instruction.rotations;
        match instruction.direction {
            Direction::L => {
                for _ in 1..(rotations + 1) {
                    cursor.move_prev();
                    if *cursor.value() == 0 {
                        trigger_count += 1;
                    }
                }
            }
            Direction::R => {
                for _ in 1..(rotations + 1) {
                    cursor.move_next();
                    if *cursor.value() == 0 {
                        trigger_count += 1;
                    }
                }
            }
        };
    }

    println!("Password is: {}", trigger_count);
}

fn main() {
    println!("=== Answer to Part 1 ===");
    part_1();
    println!("=== Answer to Part 2 ===");
    part_2();
}
