use std::fs;

fn main() {
    solution1("example1.txt");
    solution1("input.txt");
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
    let mut symbols_pos: Vec<(i32, i32)> = Vec::new();

    let mut part_number = parse_data(filename, &mut numbers, &mut symbols_pos);

    for num in numbers.iter() {
        for (row, col) in symbols_pos.iter() {
            if num.is_symbol_nearby(row, col) {
                part_number += num.number;
            }
        }
    }

    println!("Solution 1: {}", part_number);
}

fn parse_data(filename: &str, numbers: &mut Vec<Number>, symbols_pos: &mut Vec<(i32, i32)>) -> i32 {
    let mut part_number = 0;
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
                symbols_pos.push((row as i32, col as i32));
            }
        }
    }
    part_number
}