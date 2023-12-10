use std::{collections::HashMap, fs};

fn parse_numbers(line: &str) -> Vec<usize> {
    line.trim()
        .split_whitespace()
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|digit_str| digit_str.parse::<usize>().unwrap())
        .collect()
}

fn parse_content(content: String) -> (Vec<usize>, HashMap<String, Vec<Vec<usize>>>) {
    let mut seeds: Vec<usize> = Vec::new();
    let mut mapper: HashMap<String, Vec<Vec<usize>>> = HashMap::new();
    let mut current_map_str = "".to_string();
    for line in content.lines() {
        if line.starts_with("seeds:") {
            let start_index = line.find(":").unwrap() + 1;
            seeds = parse_numbers(&line[start_index..]);
        } else if line.ends_with("map:") {
            current_map_str = line.to_string();
            mapper.insert(current_map_str.clone(), Vec::new());
        } else {
            if !line.is_empty() {
                let vec = mapper.get_mut(&current_map_str).unwrap();
                vec.push(parse_numbers(&line))
            }
        }
    }

    (seeds, mapper)
}
fn main() {
    let map_contents =
        fs::read_to_string("seed_maps.txt").expect("Should have been able to open seed_maps file");

    let map_relationships = [
        "seed-to-soil map:",
        "soil-to-fertilizer map:",
        "fertilizer-to-water map:",
        "water-to-light map:",
        "light-to-temperature map:",
        "temperature-to-humidity map:",
        "humidity-to-location map:",
    ];

    let (seeds, mapper) = parse_content(map_contents);

    let min_location = seeds
        .iter()
        .map(|seed| {
            let mut base = *seed;
            let mut future_base = None;

            for map_str in map_relationships {
                let vecs = mapper.get(map_str).unwrap();

                for vec in vecs {
                    if let [dest, source, range] = vec.as_slice() {
                        if base >= *source && base < source + range {
                            future_base = Some(dest + (base - source));
                            break;
                        }
                    }
                }

                if let Some(new_base) = future_base {
                    base = new_base;
                }
            }

            base
        })
        .min()
        .unwrap();
    println!("Min location: {}", min_location);
}
