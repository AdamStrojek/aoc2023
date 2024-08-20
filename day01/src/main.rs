use std::fs;

fn main() {
    calibrate_part1("example1.txt");
    calibrate_part1("input.txt");
    calibrate_part2("example2.txt");
    calibrate_part2("input.txt");
    // calibrate_part2("edge_case.txt");
}

fn calibrate_part1(filename: &str) -> i64 {
    let mut calibration_sum: i64 = 0;

    println!("Calibrating file {}", filename);

    for line in fs::read_to_string(filename).expect("Could not read file").lines() {
        let first: char = line.chars().find(|x| x.is_ascii_digit()).unwrap();
        let last: char = line.chars().rfind(|x| x.is_ascii_digit()).unwrap();
        let number: i8 = format!("{}{}", first, last).parse().unwrap();
        calibration_sum += number as i64;
    }

    println!("Calibration1 result: {}", calibration_sum);

    calibration_sum
}

fn lineslice<'a>(line: &'a str, slices: &[&'a str]) -> Vec<(u8, u8)> {
    let slices = slices.iter().enumerate();
    let mut res: Vec<(u8, u8)> = Vec::new();

    for (i, num_str) in slices {
        for (pos, _) in line.match_indices(num_str) {
            res.push((pos as u8, (i % 10) as u8));
        }
    }

    res.sort_by_key(|x| x.0);
    res
}

fn calibrate_part2(filename: &str) -> i64 {
    let nums = vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut calibration_sum: i64 = 0;

    println!("Calibrating file {}", filename);

    for line in fs::read_to_string(filename).expect("Could not read file").lines() {
        let pos = lineslice(line, &nums);

        let number: i64 = format!("{}{}", pos.first().unwrap().1, pos.last().unwrap().1).parse().unwrap();
        calibration_sum += number;
    }

    println!("Calibration2 result: {}", calibration_sum);

    calibration_sum
}
