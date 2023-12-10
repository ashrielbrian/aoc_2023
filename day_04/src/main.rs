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

fn part_one(tickets: &String) {
    let total_points: u32 = get_matching_wins(tickets)
        .iter()
        .map(|num_wins| match num_wins.cmp(&0) {
            Ordering::Less => 0,
            Ordering::Equal => 0,
            Ordering::Greater => 2_u32.pow(*num_wins as u32 - 1),
        })
        .fold(0, |acc, val| acc + val);

    println!("Part one: {}", total_points);
}

fn part_two(tickets: &String) {
    // think of each ticket as a bucket. each bucket can have multiple copies
    // of itself. e.g. if the i'th ticket has x matches, then all subsequent
    // (i + 1) to (i + x) tickets will get one additional copy and so on.
    // so this problem is just a matter of updating the buckets.
    let mut bucket_ticket_count: Vec<usize> = vec![1; tickets.lines().count()];
    let matching_wins_per_ticket = get_matching_wins(tickets);

    for i in 0..bucket_ticket_count.len() {
        for j in 1..matching_wins_per_ticket[i] + 1 {
            bucket_ticket_count[i + j] += bucket_ticket_count[i];
        }
    }

    let total_tickets = bucket_ticket_count
        .into_iter()
        .fold(0, |acc, val| acc + val);

    println!("Part two: {}", total_tickets);
}

fn get_matching_wins(tickets: &String) -> Vec<usize> {
    tickets
        .lines()
        .map(|line| {
            let (winning, ticket) = parse_ticket(line);
            let winning: HashSet<u32> = HashSet::from_iter(winning);

            // 2^(n-1)
            let num_wins = ticket
                .into_iter()
                .filter(|num| winning.contains(num))
                .count();

            num_wins
        })
        .collect()
}

fn main() {
    let tickets = fs::read_to_string("tickets.txt").expect("Expected to read tickets file.");
    part_one(&tickets);
    part_two(&tickets);
}
