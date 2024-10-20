use std::fs;
use std::io::Result;
use itertools::Itertools;


fn main() -> Result<()> {
    solution("example.txt", 1)?;
    solution("input.txt", 1)?;

    solution("example.txt", 9)?;
    solution("input.txt", 999999)?;

    Ok(())
}

fn solution(input: &str, expand: i64) -> Result<i64> {
    let mut galaxies = vec![];

    let mut columns_count = 0;
    let mut rows_count = 0;

    let mut cols = vec![];
    let mut rows = vec![];

    for (row, line) in fs::read_to_string(input)?.lines().enumerate() {
        columns_count = line.len();
        rows_count = row + 1;

        for (col, _) in line.char_indices().filter(|x| x.1 == '#') {
            galaxies.push((row as i64, col as i64));

            if !cols.contains(&col) {
                cols.push(col);
            }

            if !rows.contains(&row) {
                rows.push(row);
            }
        }
    }

    let empty_cols = (0..columns_count).filter(|x| !cols.contains(x));
    let empty_rows = (0..rows_count).filter(|x| !rows.contains(x));

    // dup empty columns
    for row in empty_rows.rev() {
        for galaxy in galaxies.iter_mut() {
            if galaxy.0 > row as i64 {
                galaxy.0 += expand;
            }
        }
    }

    for col in empty_cols.rev() {
        for galaxy in galaxies.iter_mut() {
            if galaxy.1 > col as i64 {
                galaxy.1 += expand;
            }
        }
    }

    let result: i64 = galaxies.iter().combinations(2).map(|x| {
        (x[0].0 - x[1].0).abs() + (x[0].1 - x[1].1).abs()
    }).sum();

    println!("Result for file {} with expand size {} is {}", input, expand+1, result);

    Ok(result)
}
