use std::cmp::PartialEq;
use std::fs;

use common::mdspan::MDSpan;

fn main() {
    solution("day10/input.txt");
}


#[derive(Debug, PartialEq)]
enum Dir {
    Start,
    Pipe {
        top: bool,
        bottom: bool,
        left: bool,
        right: bool,
    },
    Empty
}

impl Dir {
    fn from_char(c: char) -> Self {
        match c {
            'S' => Dir::Start,
            '|' => Dir::Pipe{top: true, bottom: true, left: false, right: false},
            '-' => Dir::Pipe { top: false, bottom: false, left: true, right: true },
            'L' => Dir::Pipe{top: true, bottom: false, left: false, right: true},
            'J' => Dir::Pipe { top: true, bottom: false, left: true, right: false },
            '7' => Dir::Pipe{top: false, bottom: true, left: true, right: false},
            'F' => Dir::Pipe { top: false, bottom: true, left: false, right: true },
            _ => Dir::Empty,
        }
    }
}

fn solution(filename: &str) {
    let content = fs::read_to_string(filename).expect("Failed to read file");
    let mut grid: Vec<Dir> = vec![];

    let (start_col, start_row, cols, rows) = process_grid(&content, &mut grid);

    let mut pos_col : i32 = start_col as i32;
    let mut pos_row : i32 = start_row as i32;
    let mut prev_pos_col = pos_col;
    let mut prev_pos_row = pos_row;

    let span = MDSpan::new(&grid, rows, cols);

    let mut len = 0;

    loop {
        let el = span.get(pos_row as usize, pos_col as usize).unwrap();

        match el {
            Dir::Start => {
                if len == 0 {
                    for (move_row, move_col) in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
                        match span.get((pos_row + move_row) as usize, (pos_col + move_col) as usize) {
                            Some(Dir::Start) => panic!("Only one start is possible!"),
                            Some(Dir::Empty) => continue,
                            Some(_) => {
                                pos_col += move_col;
                                pos_row += move_row;
                                len += 1;
                                break;
                            },
                            _ => panic!("None is not expected"),
                        };
                    }
                } else {
                    println!("Back to start!");
                    break;
                }
            },
            Dir::Pipe { top, bottom, left, right} => {
                if *top && prev_pos_row != pos_row - 1 {
                    prev_pos_col = pos_col;
                    prev_pos_row = pos_row;
                    pos_row -= 1;
                } else if *bottom && prev_pos_row != pos_row + 1 {
                    prev_pos_col = pos_col;
                    prev_pos_row = pos_row;
                    pos_row += 1;
                } else if *left && prev_pos_col != pos_col - 1 {
                    prev_pos_col = pos_col;
                    prev_pos_row = pos_row;
                    pos_col -= 1;
                } else if *right && prev_pos_col != pos_col + 1 {
                    prev_pos_col = pos_col;
                    prev_pos_row = pos_row;
                    pos_col += 1;
                } else {
                    panic!("Pipe without exit! {:?}", el);
                }

                len += 1;
            },
            _ => {
                panic!("Unexpected element at {}:{}", pos_col, pos_row);
            }
        }
    }

    println!("Total len: {}", len);
    println!("Mid point: {}", len/2);
}

/// Process the input content and initialize the grid.
/// Returns tuple that consists of:
/// 1. The column of the starting position (0-based)
/// 2. The row of the starting position (0-based)
/// 3. The number of columns in the grid
/// 4. The number of rows in the grid
fn process_grid(content: &str, grid: &mut Vec<Dir>) -> (usize, usize, usize, usize) {

    let cols = content.find('\n').unwrap();
    let mut start_col: Option<usize> = None;
    let mut start_row: Option<usize> = None;

    for ch in content.chars() {
        if ch == '\n' { continue; }
        let el = Dir::from_char(ch);

        if el == Dir::Start {
            let pos = grid.len();
            start_col = Some(pos%cols);
            start_row = Some(pos/cols);
        }
        grid.push(el);
    }

    println!("Columns: {}", cols);
    println!("Start: {}x{}", start_col.unwrap(), start_row.unwrap());

    (start_col.unwrap(), start_row.unwrap(), cols, grid.len()/cols)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_grid() {
        let content = "...\nS-7";
        let mut grid: Vec<Dir> = vec![];
        let (start_col, start_row, cols, rows) = process_grid(content, &mut grid);

        assert_eq!(grid, vec![Dir::Empty, Dir::Empty, Dir::Empty, Dir::Start, Dir::Pipe { top: false, bottom: false, left: true, right: true }, Dir::Pipe { top: false, bottom: true, left: true, right: false }]);
        assert_eq!((start_col, start_row), (0, 1));
        assert_eq!(cols, 3);
        assert_eq!(rows, 2);
    }
}
