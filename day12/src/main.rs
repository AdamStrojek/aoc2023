use std::fs;
use std::io::Result;
use itertools::{Itertools, repeat_n};

fn main() -> Result<()> {
    process_file("example.txt")?;
    process_file("input.txt")?;

    Ok(())
}

fn process_file(input: &str) -> Result<i32> {
    let binding = fs::read_to_string(input)?;
    let lines = binding.lines();

    let mut solutions = 0;

    for line in lines {
        if let Some((map, arr)) = line.split_once(" ") {
            let arr = arr.split(',').map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

            let count_missing = map.chars().filter(|&x| x == '?').count();

            // println!("{:?} missing {} for {:?}", map, count_missing, arr);

            let replacements = repeat_n(['.', '#'], count_missing).multi_cartesian_product();

            for mut rep in replacements {
                let solution = map.chars().map(|x| if x == '?' { rep.pop().unwrap() } else { x }).collect::<String>();

                if is_solution(&solution, &arr) {
                    solutions += 1;
                }
            }
        }
    }

    println!("Solutions for file {}: {}", input, solutions);

    Ok(solutions)
}

fn is_solution(map: &str, arr: &Vec<i32>) -> bool {
    let mut bad_len = 0;

    let mut it = map.chars().peekable();
    let mut it_arr = arr.iter();

    while let Some(ch) = it.next() {
        if ch == '#' {
            bad_len += 1;

            let peek_ch = it.peek();

            if peek_ch.is_none_or(|&x| x != '#' ) {
                let expected = it_arr.next();

                if !expected.is_some_and(|&x| x == bad_len) {
                    return false
                }

                bad_len = 0;
            }
        }
    }

    it_arr.next().is_none()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_solution() {
        assert!(is_solution(".", &vec![]));
        assert!(is_solution("#", &vec![1]));
        assert!(is_solution("##", &vec![2]));
        assert!(is_solution("#.##", &vec![1, 2]));
        assert!(is_solution(".#.##", &vec![1, 2]));
        assert!(is_solution(".#.##.", &vec![1, 2]));
        assert!(is_solution("#.##.", &vec![1, 2]));
        assert!(!is_solution(".#.##.", &vec![1, 3]));
        assert!(!is_solution(".###........", &vec![3, 2, 1]));
    }
}