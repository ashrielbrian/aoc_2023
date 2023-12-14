use std::{char, cmp::Ordering, collections::HashMap, fs};

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Hand {
    HighCard([Card; 5]),
    OnePair([Card; 5]),
    TwoPair([Card; 5]),
    ThreeOfAKind([Card; 5]),
    FullHouse([Card; 5]),
    FourOfAKind([Card; 5]),
    FiveOfAKind([Card; 5]),
}

fn string_to_card_array(s: &str, has_joker: bool) -> [Card; 5] {
    let vecs: Result<[Card; 5], _> = s
        .chars()
        .filter_map(|c| match c {
            '2' => Some(Card::Two),
            '3' => Some(Card::Three),
            '4' => Some(Card::Four),
            '5' => Some(Card::Five),
            '6' => Some(Card::Six),
            '7' => Some(Card::Seven),
            '8' => Some(Card::Eight),
            '9' => Some(Card::Nine),
            'T' => Some(Card::Ten),
            'J' => {
                if has_joker {
                    Some(Card::Joker)
                } else {
                    Some(Card::Jack)
                }
            }
            'Q' => Some(Card::Queen),
            'K' => Some(Card::King),
            'A' => Some(Card::Ace),
            _ => None,
        })
        .collect::<Vec<Card>>()
        .try_into();

    vecs.unwrap()
}

fn parse_hand(hand: &str, has_joker: bool) -> Hand {
    let mut char_counts = hand.chars().fold(HashMap::new(), |mut mapper, c| {
        if has_joker && c == 'J' {
            // use lowercase j to indicate joker
            *mapper.entry('j').or_insert(0) += 1;
        } else {
            *mapper.entry(c).or_insert(0) += 1;
        }
        mapper
    });

    // give all joker cards to the highest occuring card. if it is the highest occuring card,
    // give it to the second highest one.
    if has_joker {
        match char_counts.get(&'j').cmp(&Some(&0)) {
            Ordering::Greater => {
                let mut max_value: &mut i32 = &mut i32::MIN;
                let joker_value = char_counts.get(&'j').unwrap().clone();

                // special case, if all jokers, we don't want to do anything.
                if joker_value != 5 {
                    for (key, value) in char_counts.iter_mut() {
                        if *key != 'j' && value > max_value {
                            max_value = value;
                        }
                    }

                    *max_value += joker_value;

                    // set the joker counts to 0
                    char_counts.entry('j').and_modify(|v| *v = 0);
                }
            }
            _ => {}
        }
    }

    let mut sorted_values: Vec<&i32> = char_counts.values().collect();
    sorted_values.sort_by(|a, b| b.cmp(a));

    let card_array = string_to_card_array(hand, has_joker);

    let i = 0;
    match sorted_values[i] {
        5 => Hand::FiveOfAKind(card_array),
        4 => Hand::FourOfAKind(card_array),
        3 => match sorted_values[i + 1] {
            2 => Hand::FullHouse(card_array),
            _ => Hand::ThreeOfAKind(card_array),
        },
        2 => match sorted_values[i + 1] {
            2 => Hand::TwoPair(card_array),
            _ => Hand::OnePair(card_array),
        },
        _ => Hand::HighCard(card_array),
    }
}

fn run(hands_text: &String, has_joker: bool) {
    let mut hands: Vec<(usize, Hand)> = Vec::new();
    let mut bids: Vec<i32> = Vec::new();

    // enumerate to keep track of the original index so we can index into each hand's
    // respective bids vector.
    for (i, line) in hands_text.lines().enumerate() {
        let parsed_values = line.split_whitespace().collect::<Vec<&str>>();

        let hand = parse_hand(parsed_values[0], has_joker);
        let bid = parsed_values[1].parse::<i32>().unwrap();

        hands.push((i, hand));
        bids.push(bid)
    }

    // weakest hand first (lowest rank), strongest hand last (highest rank)
    hands.sort_by(|a, b| a.1.cmp(&b.1));

    let total_winnings = hands
        .iter()
        // enumerate here to get the rank of each hand (0'th index, see next comment)
        .enumerate()
        // + 1 to the rank, because the 0th  index is rank 1
        .fold(0, |acc, val| acc + bids[val.1 .0] * (val.0 + 1) as i32);

    println!(
        "With joker: {}. Total Winnings: {}",
        has_joker, total_winnings
    );
}
fn main() {
    let hands_text =
        fs::read_to_string("hands.txt").expect("Should have been able to load hands text file");

    // part one
    run(&hands_text, false);

    // part two
    run(&hands_text, true);
}

// MOVE ALL THESE INTO A TEST
// assert!(
//     Hand::FourOfAKind(string_to_card_array("88788"))
//         < Hand::FourOfAKind(string_to_card_array("99929"))
// );
// assert!(
//     Hand::TwoPair(string_to_card_array("KK677")) > Hand::TwoPair(string_to_card_array("KTJJT"))
// );
// assert!(
//     Hand::ThreeOfAKind(string_to_card_array("T55J5"))
//         < Hand::ThreeOfAKind(string_to_card_array("QQQJA"))
// );
