use itertools::Itertools;
use std::cmp;
use std::fs;

fn insert_space(universe: &mut Vec<Vec<i32>>) -> (Vec<usize>, Vec<usize>) {
    /* This was initially used for part one to expand the matrix to insert additional
    rows and columns of space. But this has been superseded by a much simpler intuition
    - see the function shortest_path_with_space_expansion */
    let mut no_galaxy_row_indices: Vec<usize> = Vec::new();
    let mut no_galaxy_col_indices: Vec<usize> = Vec::new();

    // get row indexes to insert
    // need to account for the fact that we will be mutating the universe
    // (inserting new rows/cols) even as we are looping over the universe matrix.
    // so when we insert a new row, the index of the next row to insert would be increased by 1.
    // similarly for the cols.
    let mut num_rows_inserted = 0;
    for (idx, row) in universe.iter().enumerate() {
        if row.iter().all(|i| *i == 0) {
            no_galaxy_row_indices.push(idx + num_rows_inserted);
            num_rows_inserted += 1;
        }
    }

    for row in no_galaxy_row_indices.iter() {
        universe.insert(*row, vec![0; universe[0].len()]);
    }

    // get col indexes to insert
    let mut num_cols_inserted = 0;
    for col in 0..universe[0].len() {
        let mut only_zeroes = true;
        for row in 0..universe.len() {
            if universe[row][col] != 0 {
                only_zeroes = false;
                break;
            }
        }

        if only_zeroes {
            no_galaxy_col_indices.push(col + num_cols_inserted);
            num_cols_inserted += 1;
        }
    }

    for col in no_galaxy_col_indices.iter() {
        for row in 0..universe.len() {
            universe[row].insert(*col, 0);
        }
    }

    (no_galaxy_row_indices, no_galaxy_col_indices)
}

fn get_space(universe: &Vec<Vec<i32>>) -> (Vec<usize>, Vec<usize>) {
    /* Gets all row and column indices that have **no** spaces in them. */
    let mut no_galaxy_row_indices: Vec<usize> = Vec::new();
    let mut no_galaxy_col_indices: Vec<usize> = Vec::new();

    // get row indexes to insert
    for (idx, row) in universe.iter().enumerate() {
        if row.iter().all(|i| *i == 0) {
            no_galaxy_row_indices.push(idx);
        }
    }

    // get col indexes to insert
    for col in 0..universe[0].len() {
        let mut only_zeroes = true;
        for row in 0..universe.len() {
            if universe[row][col] != 0 {
                only_zeroes = false;
                break;
            }
        }

        if only_zeroes {
            no_galaxy_col_indices.push(col);
        }
    }
    (no_galaxy_row_indices, no_galaxy_col_indices)
}

fn shortest_path_with_space_expansion(
    universe: &Vec<Vec<i32>>,
    space_row_indices: &Vec<usize>,
    space_col_indices: &Vec<usize>,
    spaces_to_expand: usize,
) {
    // gets the coordinates of all galaxies (non-zero digits)
    let galaxies: Vec<(i32, usize, usize)> = universe
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(|(j, galaxy)| (*galaxy, i, j))
                .collect::<Vec<(i32, usize, usize)>>()
        })
        .flatten()
        .filter(|(galaxy, _, _)| *galaxy != 0)
        .collect();

    /* the intuition here is - for a given pair of galaxies, we want to identify
    all the row and columns that are going to be expanded. we can check if such a
    row/column index lies between the pair of galaxies. once we know a certain row/column
    index is relevant to the pair of galaxies, we expand it mathematically, i.e. the number of
    such row/cols indices between the pair of galaxies multiplied by the space to be expanded.
    take that and add it to the total number of steps between the pair. that is the shortest path.
    */
    let sum_of_shortest_paths = galaxies
        .iter()
        .combinations(2)
        .map(|pair| {
            // the intuition i figured here is that the number of space
            let mut num_empty_spaces_in_between = 0;

            num_empty_spaces_in_between += space_col_indices
                .iter()
                .filter(|col| {
                    *col < cmp::max(&pair[0].2, &pair[1].2)
                        && *col > cmp::min(&pair[0].2, &pair[1].2)
                })
                .count();

            num_empty_spaces_in_between += space_row_indices
                .iter()
                .filter(|row| {
                    *row < cmp::max(&pair[0].1, &pair[1].1)
                        && *row > cmp::min(&pair[0].1, &pair[1].1)
                })
                .count();

            let distance_between_pairs = pair[0].1.abs_diff(pair[1].1)
                + pair[0].2.abs_diff(pair[1].2)
                + (spaces_to_expand * num_empty_spaces_in_between);

            distance_between_pairs
        })
        .fold(0, |acc, val| acc + val);

    println!(
        "Shortest path: {}, with spaces {}",
        sum_of_shortest_paths, spaces_to_expand
    );
}

fn main() {
    let universe = fs::read_to_string("input.txt").expect("Should be able to read input text.");

    let mut num_galaxies = 0;
    let parsed_universe: Vec<Vec<i32>> = universe
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '#' => {
                        num_galaxies += 1;
                        num_galaxies
                    }
                    _ => 0,
                })
                .collect()
        })
        .collect();

    let (space_row_indices, space_col_indices) = get_space(&parsed_universe);

    // part one
    shortest_path_with_space_expansion(&parsed_universe, &space_row_indices, &space_col_indices, 1);

    // part two
    shortest_path_with_space_expansion(
        &parsed_universe,
        &space_row_indices,
        &space_col_indices,
        999999,
    );
}
