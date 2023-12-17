use std::fs;

fn extrapolate(nums: &Vec<i64>, backward: bool) -> i64 {
    let mut differences: Vec<Vec<i64>> = Vec::new();

    let mut curr_diffs = nums
        .windows(2)
        .map(|val| val[1] - val[0])
        .collect::<Vec<i64>>();

    differences.push(curr_diffs.clone());

    while !curr_diffs.iter().all(|val| *val == 0) {
        curr_diffs = curr_diffs
            .windows(2)
            .map(|val| val[1] - val[0])
            .collect::<Vec<i64>>();

        differences.push(curr_diffs.clone());
    }

    if backward {
        let mut value = 0;
        for difference in differences.iter().rev() {
            value = -value + difference[0];
        }

        nums[0] - value
    } else {
        differences.iter().fold(0, |value, difference| {
            value + difference[difference.len() - 1]
        }) + nums[nums.len() - 1]
    }
}

fn part_one(oasis_history: &Vec<Vec<i64>>) {
    let total = oasis_history
        .iter()
        .map(|history| extrapolate(history, false))
        .fold(0, |acc, val| acc + val);
    println!("Part one: {}", total);
}

fn part_two(oasis_history: &Vec<Vec<i64>>) {
    let total = oasis_history
        .iter()
        .map(|history| extrapolate(history, true))
        .fold(0, |acc, val| acc + val);
    println!("Part two: {}", total);
}
fn main() {
    let oasis_contents =
        fs::read_to_string("oasis.txt").expect("Should have been able to open the file.");

    let oasis: Vec<Vec<i64>> = oasis_contents
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.trim().parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect();

    part_one(&oasis);
    part_two(&oasis);
}
