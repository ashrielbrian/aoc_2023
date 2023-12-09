use std::{cmp::Ordering, collections::HashSet, fs};
fn parse_ticket(ticket: &str) -> (Vec<u32>, Vec<u32>) {
    let start_index = ticket.find(':').unwrap();
    parse_numbers(&ticket[start_index + 1..])
}

fn parse_numbers(numbers: &str) -> (Vec<u32>, Vec<u32>) {
    let parsed_winnings: Vec<Vec<u32>> = numbers
        .split('|')
        .map(|nums| {
            nums.trim()
                .split_whitespace()
                .filter(|s| !s.is_empty())
                .map(|digit_str| digit_str.trim().parse::<u32>().unwrap())
                .collect()
        })
        .collect();

    if let [winning, ticket] = parsed_winnings.as_slice() {
        (winning.clone(), ticket.clone())
    } else {
        (Vec::new(), Vec::new())
    }
}
fn main() {
    let tickets = fs::read_to_string("tickets.txt").expect("Expected to read tickets file.");

    let total_points: u32 = tickets
        .lines()
        .map(|line| {
            let (winning, ticket) = parse_ticket(line);
            let winning: HashSet<u32> = HashSet::from_iter(winning);

            // 2^(n-1)
            let num_wins = ticket
                .into_iter()
                .filter(|num| winning.contains(num))
                .count();
            let ticket_points = match num_wins.cmp(&0) {
                Ordering::Less => 0,
                Ordering::Equal => 0,
                Ordering::Greater => 2_u32.pow(num_wins as u32 - 1),
            };

            ticket_points
        })
        .fold(0, |acc, val| acc + val);

    println!("{}", total_points);
}
