use std::fs;

fn parse_race(races: String) -> (Vec<usize>, Vec<usize>) {
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

fn main() {
    let races_text =
        fs::read_to_string("races.txt").expect("Should have been able to open the races file");
    let (time, distance) = parse_race(races_text);

    println!("{:?}", time);
    println!("{:?}", distance);

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

    println!("Num ways: {}", value);
}
