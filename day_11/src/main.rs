use std::fs;

fn insert_space(universe: &mut Vec<Vec<i32>>) {
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

    for row in no_galaxy_row_indices {
        universe.insert(row, vec![0; universe[0].len()]);
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

    for col in no_galaxy_col_indices {
        for row in 0..universe.len() {
            universe[row].insert(col, 0);
        }
    }
}

fn main() {
    let universe =
        fs::read_to_string("small_input.txt").expect("Should be able to read input text.");

    let mut num_galaxies = 0;
    let mut parsed_universe: Vec<Vec<i32>> = universe
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

    insert_space(&mut parsed_universe);
}
