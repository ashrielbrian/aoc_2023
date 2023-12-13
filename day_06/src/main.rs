use std::fs;

fn parse_race_part_one(races: &String) -> (Vec<usize>, Vec<usize>) {
    if let [time, distance] = races
        .lines()
        .map(|line| {
            if line.starts_with("Time:") {
                line[line.find(":").unwrap() + 1..]
                    .trim()
                    .split_whitespace()
                    .map(|char| char.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>()
            } else {
                line[line.find(":").unwrap() + 1..]
                    .trim()
                    .split_whitespace()
                    .map(|char| char.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>()
            }
        })
        .collect::<Vec<Vec<usize>>>()
        .as_slice()
    {
        (time.to_vec(), distance.to_vec())
    } else {
        (Vec::new(), Vec::new())
    }
}

fn parse_race_part_two(race: &String) -> (usize, usize) {
    if let [time, distance] = race
        .lines()
        .map(|line| {
            let combined_string: String = line[line.find(":").unwrap() + 1..]
                .chars()
                .filter(|c| !c.is_whitespace())
                .collect();

            combined_string.parse::<usize>().unwrap()
        })
        .collect::<Vec<usize>>()
        .as_slice()
    {
        (*time, *distance)
    } else {
        (0, 0)
    }
}

fn part_one(time: Vec<usize>, distance: Vec<usize>) {
    let value = time
        .into_iter()
        .zip(distance)
        .map(|(max_time, record_distance)| {
            let mut unique_wins = 0;
            for time_waiting in 1..(max_time) {
                let time_moving = max_time - time_waiting;

                if time_moving * time_waiting > record_distance {
                    unique_wins += 1;
                }
            }
            unique_wins
        })
        .fold(1, |acc, val| acc * val);

    println!("Part one: {}", value);
}

fn part_two(time: usize, record_distance: usize) {
    // the idea here is that if we plotted a graph where x-axis is time waiting, and y-axis
    // is the total distance that can be travelled, we'd get a symmetrical parabolic curve
    // of the form y = -ax. Since it's symmetrical, we simply need to find the time x where
    // it is the largest distance that is as yet LESS than record_distance. and we know the
    // the total number of ways, is simply the total_time minus (2 * x).
    let mut time_waiting: usize = 0;
    while (time - time_waiting) * (time_waiting) < record_distance {
        time_waiting += 1;
    }

    // (time + 1) because because the 0th time is itself a coordinate we need to account for.
    println!("Part two: {}", (time + 1) - (2 * time_waiting));
}

fn main() {
    let races_text =
        fs::read_to_string("races.txt").expect("Should have been able to open the races file");

    let (time, distance) = parse_race_part_one(&races_text);
    part_one(time, distance);

    let (time, distance) = parse_race_part_two(&races_text);
    part_two(time, distance);
}
