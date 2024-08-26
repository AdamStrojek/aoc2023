use std::fs;

fn main() {
    let mut result = 0;

    let content = fs::read_to_string("day09/input.txt")
        .expect("Could not read file");

    for line in content.lines() {
        let input: Vec<i64> = line.split_whitespace().filter_map(|x| x.parse().ok()).collect();
        result += extrapolate_next_value(&input);
    }

    println!("Result part 1 {}", result);

    result = 0;

    for line in content.lines() {
        let input: Vec<i64> = line.split_whitespace().filter_map(|x| x.parse().ok()).collect();
        result += extrapolate_previous_value(&input);
    }

    println!("Result part 2 {}", result);
}

fn generate_differences(sequence: &Vec<i64>) -> Vec<i64> {
    let mut differences = Vec::new();
    let mut it = sequence.iter().peekable();

    while let Some(i) = it.next() {
        if let Some(&j) = it.peek() {
           differences.push(j - i);
        }
    }
    differences
}

fn extrapolate_next_value(sequence: &Vec<i64>) -> i64 {
    let mut sequences = vec![sequence.clone()];
    while !sequences.last().unwrap().iter().all(|&x| x == 0) {
        let next_sequence = generate_differences(sequences.last().unwrap());
        sequences.push(next_sequence);
    }

    for i in (0..sequences.len() - 1).rev() {
        let next_value = sequences[i].last().unwrap() + sequences[i + 1].last().unwrap();
        sequences[i].push(next_value);
    }

    *sequences[0].last().unwrap()
}

fn extrapolate_previous_value(sequence: &Vec<i64>) -> i64 {
    let mut sequences = vec![sequence.clone()];
    while !sequences.last().unwrap().iter().all(|&x| x == 0) {
        let next_sequence = generate_differences(sequences.last().unwrap());
        sequences.push(next_sequence);
    }

    for i in (0..sequences.len() - 1).rev() {
        let prev_value = sequences[i].first().unwrap() - sequences[i + 1].first().unwrap();
        sequences[i].insert(0, prev_value);
    }

    *sequences[0].first().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_differences() {
        let input = vec![0, 3, 6, 9, 12, 15];
        assert_eq!(vec![3, 3, 3, 3, 3], generate_differences(&input));
    }

    #[test]
    fn test_extrapolate_next_value() {
        let input = vec![0, 3, 6, 9, 12, 15];
        assert_eq!(18, extrapolate_next_value(&input));
    }

    #[test]
    fn test_extrapolate_next_value_2() {
        let input = vec![10, 13, 16, 21, 30, 45];
        assert_eq!(68, extrapolate_next_value(&input));
    }

    #[test]
    fn test_extrapolate_prev_value() {
        let input = vec![0, 3, 6, 9, 12, 15];
        assert_eq!(-3, extrapolate_previous_value(&input));
    }

    #[test]
    fn test_extrapolate_prev_value_2() {
        let input = vec![10, 13, 16, 21, 30, 45];
        assert_eq!(5, extrapolate_previous_value(&input));
    }
}
