use std::fs;

fn main() {
    solution1("day06/example.txt");
    solution1("day06/input.txt");
    solution2("day06/example.txt");
    solution2("day06/input.txt");
}

fn calculate_once(time: u64, distance: u64) -> u64 {
    let mut ways_to_win = 0;
    for h in 1..time {  // We skip h = 0 as it will always result in 0 distance
        let travel_time = time - h;
        let total_distance = h * travel_time;
        if total_distance > distance {
            ways_to_win += 1;
        }
    }
    ways_to_win
}

fn calculate_time(time: &Vec<u64>, distance: &Vec<u64>) -> u64 {
    let mut result = 1;

    for (time, distance) in time.iter().zip(distance) {
        result *= calculate_once(*time, *distance);
    }

    result
}

fn solution1(filename: &str) {
    let content = fs::read_to_string(filename).unwrap();
    let mut lines = content.lines();

    let time: Vec<u64> = lines.next().unwrap().split_whitespace().skip(1).map(|x| x.parse().unwrap()).collect();
    let distance: Vec<u64> = lines.next().unwrap().split_whitespace().skip(1).map(|x| x.parse().unwrap()).collect();

    let result = calculate_time(&time, &distance);

    println!("Solution 1: {}", result);
}

fn solution2(filename: &str) {
    let content = fs::read_to_string(filename).unwrap();
    let mut lines = content.lines();

    let time: u64 = lines.next().unwrap().strip_prefix("Time: ").unwrap().replace(" ", "").parse().unwrap();
    let distance: u64 = lines.next().unwrap().strip_prefix("Distance: ").unwrap().replace(" ", "").parse().unwrap();

    let result = calculate_once(time, distance);

    println!("Solution 2: {}", result);
}

