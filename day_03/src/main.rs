use std::cmp::{Ord, Ordering};
use std::collections::HashSet;
use std::fs;

static OFFSETS: &'static [(i32, i32)] = &[
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];
fn get_digit<'a>(
    row: i32,
    col: i32,
    movement: i32,
    matrix: &'a Vec<Vec<&str>>,
    visited: &mut HashSet<(i32, i32)>,
) -> String {
    if visited.contains(&(row, col)) || col < 0 || col >= matrix[row as usize].len() as i32 {
        return "".to_string();
    };

    let value = matrix[row as usize][col as usize];

    if value.is_empty() {
        return "".to_string();
    };

    if let Some(char) = value.chars().next() {
        if char.is_digit(10) {
            visited.insert((row, col));

            match movement.cmp(&0) {
                Ordering::Equal => {
                    let left = get_digit(row, col - 1, -1, matrix, visited);
                    let right = get_digit(row, col + 1, 1, matrix, visited);

                    return format!("{}{}{}", left, char, right);
                }
                Ordering::Less => {
                    let left = get_digit(row, col - 1, movement, matrix, visited);
                    return format!("{left}{char}");
                }
                Ordering::Greater => {
                    let right = get_digit(row, col + 1, movement, matrix, visited);
                    return format!("{char}{right}");
                }
            };
        }
    }

    return "".to_string();
}

fn part_one(matrix: &Vec<Vec<&str>>) {
    let mut visited = HashSet::new();
    let mut total_parts = 0;

    for row in 0..matrix.len() {
        for col in 0..matrix[row].len() {
            if let Some(char) = &matrix[row][col].chars().next() {
                if !char.is_digit(10) && *char != '.' {
                    // use off-sets to find the numbers
                    total_parts += OFFSETS
                        .iter()
                        .map(|(row_offset, col_offset)| {
                            let movement = match (*row_offset, *col_offset) {
                                (0, -1) => -1,
                                (0, 1) => 1,
                                _ => 0,
                            };

                            get_digit(
                                row as i32 + row_offset,
                                col as i32 + col_offset,
                                movement,
                                &matrix,
                                &mut visited,
                            )
                        })
                        .filter(|digit_str| digit_str.len() > 0)
                        .map(|digit_str| digit_str.parse::<i32>().unwrap())
                        .fold(0, |acc, val| acc + val);
                }
            }
        }
    }
    println!("Part one: {}", total_parts)
}

fn part_two(matrix: &Vec<Vec<&str>>) {
    let mut visited = HashSet::new();
    let mut total_gears = 0;

    for row in 0..matrix.len() {
        for col in 0..matrix[row].len() {
            if let Some(char) = &matrix[row][col].chars().next() {
                if !char.is_digit(10) && *char == '*' {
                    // use off-sets to find the numbers
                    let adjacent_numbers: Vec<i32> = OFFSETS
                        .iter()
                        .map(|(row_offset, col_offset)| {
                            let movement = match (*row_offset, *col_offset) {
                                (0, -1) => -1,
                                (0, 1) => 1,
                                _ => 0,
                            };

                            get_digit(
                                row as i32 + row_offset,
                                col as i32 + col_offset,
                                movement,
                                &matrix,
                                &mut visited,
                            )
                        })
                        .filter(|digit_str| digit_str.len() > 0)
                        .map(|digit_str| digit_str.parse::<i32>().unwrap())
                        .collect();

                    // magic number exactly 2, as given by the problem definition. gear ratio when exactly
                    // two adjacent numbers beside a `*`
                    if adjacent_numbers.len() == 2 {
                        if let [first, second] = adjacent_numbers.as_slice() {
                            total_gears += first * second;
                        }
                    }
                }
            }
        }
    }
    println!("Part two: {}", total_gears)
}

fn main() {
    let schematics_text =
        fs::read_to_string("schematics.txt").expect("Should be able to read schematics text file.");

    let matrix: Vec<Vec<&str>> = schematics_text
        .lines()
        .map(|line| line.split("").collect())
        .collect();

    println!(
        "Matrix Input -> Rows: {}, Cols: {}",
        matrix.len(),
        matrix[0].len()
    );
    part_one(&matrix);
    part_two(&matrix);
}
