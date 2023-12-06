use regex::Regex;
use std::collections::HashMap;
use std::fs;

/* PART ONE: Determine which games would have been possible if the bag had been loaded
with only 12 red cubes, 13 green cubes, and 14 blue cubes. What is the sum of
 the IDs of those games? */
fn parse_cube_line(line: &str) -> (u32, Vec<HashMap<&str, u32>>) {
    let re = Regex::new(r"Game (?P<game>[0-9]+):").unwrap();
    let parsed_regex = re.captures(line).unwrap();

    let game: u32 = parsed_regex["game"].parse().unwrap();
    let start_game_index = line.find(':').unwrap();
    let cube_sets: Vec<HashMap<&str, u32>> = parse_games(&line[start_game_index + 1..]);

    (game, cube_sets)
}

fn parse_games(line: &str) -> Vec<HashMap<&str, u32>> {
    line.trim()
        .split(";")
        .filter(|segment| !segment.trim().is_empty())
        .map(|game_set| {
            let mut colour_counts: HashMap<&str, u32> = HashMap::new();

            for set in game_set.trim().split(",") {
                let parts: Vec<&str> = set.trim().split_whitespace().collect();
                if let [count, colour] = parts.as_slice() {
                    if let Ok(count) = count.parse::<u32>() {
                        colour_counts.insert(*colour, count);
                    }
                }
            }

            colour_counts
        })
        .collect()
}

fn part_one() {
    let mut requirements: HashMap<&str, u32> = HashMap::new();
    requirements.insert("red", 12);
    requirements.insert("green", 13);
    requirements.insert("blue", 14);

    let cubes_text =
        fs::read_to_string("cubes.txt").expect("Should have been able to read cubes text file");

    let game_sum = cubes_text
        .lines()
        .map(|line| parse_cube_line(line))
        .filter(|(_game, sets)| {
            for set in sets {
                let result = &requirements.keys().all(|key| {
                    requirements.get(key).unwrap() >= &set.get(key).unwrap_or(&(0 as u32))
                });
                if !*result {
                    return *result;
                }
            }

            true
        })
        .fold(0 as u32, |acc, val| acc + val.0);

    println!("Part one: {}", game_sum);
}

fn part_two() {
    let cubes_text =
        fs::read_to_string("cubes.txt").expect("Should have been able to read cubes text file");

    let game_power = cubes_text
        .lines()
        .map(|line| parse_cube_line(line))
        .map(|(_, sets)| {
            let mut red_max = &std::u32::MIN;
            let mut green_max = &std::u32::MIN;
            let mut blue_max = &std::u32::MIN;

            for set in &sets {
                if set.get("red").unwrap_or(&0) > red_max {
                    red_max = set.get("red").unwrap_or(&0);
                }
                if set.get("green").unwrap_or(&0) > green_max {
                    green_max = set.get("green").unwrap_or(&0);
                }
                if set.get("blue").unwrap_or(&0) > blue_max {
                    blue_max = set.get("blue").unwrap_or(&0);
                }
            }

            red_max * blue_max * green_max
        })
        .fold(0, |acc, val| acc + val);

    println!("Part two: {}", game_power);
}
fn main() {
    part_one();
    part_two();
}
