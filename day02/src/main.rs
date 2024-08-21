use std::cmp::max;
use std::collections::HashMap;
use std::fs;

fn main() {
    let cubes = HashMap::from([("red", 12u8), ("green", 13u8), ("blue", 14u8)]);

    solution1("example1.txt", &cubes);
    solution1("input.txt", &cubes);
}

fn solution1(filename: &str, max_cubes: &HashMap<&str, u8>) {
    println!("Calibrating file {}", filename);

    let mut result = 0;

    for line in fs::read_to_string(filename).expect("Could not read file").lines() {
        // "Game 1: ....."
        //       ^ Move cursor to the first digit
        // All lines starts with "Game "
        let mut it = line[5..].chars();

        // Collect all digits to generate game number
        let game: u8 = it.by_ref().take_while(|x| x.is_ascii_digit()).collect::<String>().parse().unwrap();

        // Remaining characters in line buffer:
        // " 1 red, 1 green, 1 blue; 2 red, 1 green, 6 blue; 2 green"
        // ":" is missing due to nature of take_while, but it is not important for us
        // We need to trim space

        let shown_cubes: String = it.by_ref().collect();

        // Extract highest number of cubes from each releave and save in hashmap
        let mut cubes: HashMap<&str, u8> = HashMap::new();
        for cubes_releaved in shown_cubes.trim().split("; ") {
            // "3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
            let mut splitted = cubes_releaved.split(", ");
            while let Some(single_cube) = splitted.next() {
                // "3 blue, 4 red"
                let mut splitted = single_cube.split(" ");
                let count: u8 = splitted.next().unwrap().parse().unwrap();
                let color = splitted.next().unwrap();
                cubes.entry(color)
                    .and_modify(|stored_count| *stored_count = max(*stored_count, count))
                    .or_insert(count);
            }
        }

        // Check is this game is valid
        let mut valid_game = true;

        for (color, count) in max_cubes {
            if let Some(stored_count) = cubes.get(color) {
                valid_game &= stored_count <= count;
            }
        }

        println!("{} is valid {}", game, valid_game);
        if valid_game {
            result += game as u32;
        }
    }

    println!("Solution 1: {}", result);
}
