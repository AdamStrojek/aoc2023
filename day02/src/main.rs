use std::cmp::max;
use std::collections::HashMap;
use std::fs;

fn main() {
    let cubes = HashMap::from([("red".to_string(), 12u8), ("green".to_string(), 13u8), ("blue".to_string(), 14u8)]);

    solution1("example1.txt", &cubes);
    solution1("input.txt", &cubes);
    solution2("example2.txt");
    solution2("input.txt");
}

struct Game {
    game: u32,
    cubes: HashMap<String, u8>,
}

impl Game {
    pub fn from_line(line: &str) -> Self {
        let mut cubes: HashMap<String, u8> = HashMap::new();
        // "Game 1: ....."
        //       ^ Move cursor to the first digit
        // All lines starts with "Game "
        let mut it = line[5..].chars();

        // Collect all digits to generate game number
        let game: u32 = it.by_ref().take_while(|x| x.is_ascii_digit()).collect::<String>().parse().unwrap();

        // Remaining characters in line buffer:
        // " 1 red, 1 green, 1 blue; 2 red, 1 green, 6 blue; 2 green"
        // ":" is missing due to nature of take_while, but it is not important for us
        // We need to trim space

        let shown_cubes: String = it.by_ref().collect();

        // Extract highest number of cubes from each releave and save in hashmap
        for cubes_releaved in shown_cubes.trim().split("; ") {
            // "3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
            let mut splitted = cubes_releaved.split(", ");
            while let Some(single_cube) = splitted.next() {
                // "3 blue, 4 red"
                let mut splitted = single_cube.split(" ");
                let count: u8 = splitted.next().unwrap().parse().unwrap();
                let color = splitted.next().unwrap().to_string();
                cubes.entry(color)
                    .and_modify(|stored_count| *stored_count = max(*stored_count, count))
                    .or_insert(count);
            }
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
            println!("Is valid {}", game.game);
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
