use std::cmp::max;
use std::collections::HashMap;
use std::fs;

use common::scanner::DirtyScanner;

fn main() {
    let cubes = HashMap::from([("red".to_string(), 12u8), ("green".to_string(), 13u8), ("blue".to_string(), 14u8)]);

    solution1("day02/example1.txt", &cubes);
    solution1("day02/input.txt", &cubes);
    solution2("day02/example2.txt");
    solution2("day02/input.txt");
}

struct Game {
    game: u32,
    cubes: HashMap<String, u8>,
}

impl Game {
    pub fn from_line(line: &str) -> Self {
        let mut s = DirtyScanner::new(line);

        let mut cubes: HashMap<String, u8> = HashMap::new();
        s.next_word(); // Skip "Game"

        // Collect all digits to generate game number
        let game: u32 = s.next_number().unwrap();

        // Remaining characters in line buffer:
        // ": 1 red, 1 green, 1 blue; 2 red, 1 green, 6 blue; 2 green"

        while let Some(count) = s.next_number::<u8>() {
            let color = s.next_word().unwrap();

            cubes.entry(color.to_string())
                .and_modify(|stored_count| *stored_count = max(*stored_count, count))
                .or_insert(count);
        }

        Game {
            game,
            cubes
        }
    }

    pub fn is_valid(&self, max_cubes: &HashMap<String, u8>) -> bool {
        for (color, count) in max_cubes {
            if let Some(stored_count) = self.cubes.get(color) {
                if *stored_count > *count {
                    return false
                }
            }
        }

        true
    }
}

fn solution1(filename: &str,  max_cubes: &HashMap<String, u8>) {
    println!("Solving for file {}", filename);

    let mut result = 0;

    for line in fs::read_to_string(filename).expect("Could not read file").lines() {
        let game = Game::from_line(line);

        // Check is this game is valid
        if game.is_valid(&max_cubes) {
            result += game.game;
        }
    }

    println!("Solution 1: {}", result);
}

fn solution2(filename: &str) {
    println!("Solving for file {}", filename);

    let mut result = 0;

    for line in fs::read_to_string(filename).expect("Could not read file").lines() {
        let game = Game::from_line(line);

        // Check is this game is valid
        let mut score = 1;

        for count in game.cubes.values() {
            score *= *count as u32;
        }

        result += score;
    }

    println!("Solution 2: {}", result);
}
