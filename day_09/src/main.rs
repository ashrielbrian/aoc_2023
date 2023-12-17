use std::fs;

fn extrapolate(nums: &Vec<i64>) -> i64 {
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

    differences.iter().fold(0, |value, difference| {
        value + difference[difference.len() - 1]
    }) + nums[nums.len() - 1]
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

    let total = oasis
        .iter()
        .map(|history| extrapolate(history))
        .fold(0, |acc, val| acc + val);
    println!("{:?}", total);
}
