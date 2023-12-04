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

    println!("Part one: {}", calibration_sum);
}

fn part_two() {
    use std::collections::HashMap;

    let mut string_to_digit: HashMap<&str, char> = HashMap::new();
    string_to_digit.insert("one", '1');
    string_to_digit.insert("two", '2');
    string_to_digit.insert("three", '3');
    string_to_digit.insert("four", '4');
    string_to_digit.insert("five", '5');
    string_to_digit.insert("six", '6');
    string_to_digit.insert("seven", '7');
    string_to_digit.insert("eight", '8');
    string_to_digit.insert("nine", '9');

    let calibration_values =
        fs::read_to_string("calibration.txt").expect("Should have been able to read file");

    let calibration_sum = calibration_values
        .lines()
        .map(|line| {
            let mut first: Option<char> = None;
            let mut prev: Option<char> = None;

            for (i, char) in line.chars().enumerate() {
                if char.is_digit(10) {
                    match first {
                        None => first = Some(char),
                        Some(_) => {}
                    }
                    prev = Some(char);
                } else {
                    for key in string_to_digit.keys() {
                        // ensure we don't slice past the end of the line.
                        // for each char, check if it's a key. since the only relevant letters
                        // are `o`, `t`, `f`, `s`, `e`, `n`, can optimise by ignoring all other chars.
                        if i + key.len() <= line.len() && &line[i..i + key.len()] == *key {
                            match first {
                                None => first = Some(string_to_digit.get(key).unwrap().clone()),
                                Some(_) => {}
                            }

                            prev = Some(string_to_digit.get(key).unwrap().clone());
                        }
                    }
                }
            }

            let parsed_digits = format!("{}{}", first.unwrap(), prev.unwrap())
                .parse::<i32>()
                .unwrap();

            parsed_digits
        })
        .fold(0, |acc, val| acc + val);

    println!("Part two: {}", calibration_sum);
}
fn main() {
    part_one();
    part_two();
}
