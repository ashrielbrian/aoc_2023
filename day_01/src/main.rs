use std::fs;

fn part_one() {
    let calibration_values =
        fs::read_to_string("calibration.txt").expect("Should have been able to read file");

    let calibration_sum = calibration_values
        .lines()
        .map(|line| {
            let mut first: Option<char> = None;
            let mut prev: Option<char> = None;
            for char in line.chars() {
                if char.is_digit(10) {
                    match first {
                        None => first = Some(char),
                        Some(_) => {}
                    }

                    prev = Some(char);
                }
            }

            let parsed_digits = format!("{}{}", first.unwrap(), prev.unwrap())
                .parse::<i32>()
                .unwrap();

            parsed_digits
        })
        .fold(0, |acc, val| acc + val);

    println!("{}", calibration_sum);
}

fn main() {
    part_one()
}
