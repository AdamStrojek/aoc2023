use std::fs;

fn main() {
    solution1("example1.txt");
    solution1("input.txt");
    solution2("example2.txt");
    solution2("input.txt");
}

#[derive(Debug)]
struct Number {
    pos_min_col: i32,
    pos_max_col: i32,
    pos_row: i32,
    number: i32,
}

impl Number {
    fn new(pos_row: i32, pos_min_col: i32, pos_max_col: i32,  number: i32) -> Self {
        Number {
            pos_min_col,
            pos_max_col,
            pos_row,
            number,
        }
    }

    fn is_symbol_nearby(&self, row: &i32, col: &i32) -> bool {
        return ((self.pos_row-1)..=(self.pos_row+1)).contains(row) && ((self.pos_min_col-1)..=(self.pos_max_col+1)).contains(col);
    }
}

fn solution1(filename: &str) {
    println!("Solving for file {}", filename);

    let mut numbers: Vec<Number> = Vec::new();
    let mut symbols_pos: Vec<(i32, i32, char)> = Vec::new();

    let mut part_number: i32 = 0;

    parse_data(filename, &mut numbers, &mut symbols_pos);

    for num in numbers.iter() {
        for (row, col, _) in symbols_pos.iter() {
            if num.is_symbol_nearby(row, col) {
                part_number += num.number;
            }
        }
    }

    println!("Solution 1: {}", part_number);
}

fn solution2(filename: &str) {
    println!("Solving for file {}", filename);

    let mut numbers: Vec<Number> = Vec::new();
    let mut symbols_pos: Vec<(i32, i32, char)> = Vec::new();

    let mut part_number: i32 = 0;

    parse_data(filename, &mut numbers, &mut symbols_pos);

    for (row, col, ch) in symbols_pos.iter() {
        if *ch != '*' {
            // Can safely skip, we are interested only in gear ratios
            continue;
        }
        // Collect all adjacent numbers
        let mut nearby: Vec<&Number> = Vec::new();
        for num in numbers.iter() {
            if num.is_symbol_nearby(row, col) {
                nearby.push(&num);
            }
        }

        // Check what symbol was nearby
        if nearby.len() == 2 {
            // If was * and 2 items are nearby calculate gear ratio
            part_number += nearby.iter().fold(1, |acc, x| acc * x.number);
        }
    }

    println!("Solution 2: {}", part_number);
}

fn parse_data(filename: &str, numbers: &mut Vec<Number>, symbols_pos: &mut Vec<(i32, i32, char)>) {
    for (row, line) in fs::read_to_string(filename).expect("Could not read file").lines().enumerate() {
        let mut it = line.chars().enumerate().peekable();
        let mut buf = String::new();

        while let Some((col, ch)) = it.next() {
            if ch.is_ascii_digit() {
                buf.push(ch);

                let peek: Option<&(usize, char)> = it.peek();
                if peek.is_some_and(|(_, peek_ch)| !peek_ch.is_ascii_digit()) || peek.is_none() {
                    // In case next char is not a digit or line ended we need to close buffer and parse it
                    numbers.push(Number::new(row as i32, (col + 1 - buf.len()) as i32, col as i32, buf.parse().unwrap()));
                    buf.clear();
                }
            } else if ch != '.' {
                symbols_pos.push((row as i32, col as i32, ch));
            }
        }
    }
}