use std::{
    collections::HashSet,
    fs::File,
    io::{self, prelude::*, BufReader},
};

fn main() -> io::Result<()> {
    let file = File::open("../../input.txt")?;

    let cards = BufReader::new(file)
        .lines()
        .map(|line| parse_card(&line.unwrap()))
        .collect::<Vec<_>>();
    let mut card_counts = cards.iter().map(|_| 1_u32).collect::<Vec<_>>();
    for (index, card) in cards.iter().enumerate() {
        let card_count = *card_counts.get(index).unwrap();
        let win_count = compute_win_count(&card);
        for delta in 1..=win_count {
            *card_counts.get_mut(index + delta as usize).unwrap() += card_count;
        }
    }

    let points: u32 = card_counts.iter().sum();
    println!("{}", points);
    Ok(())
}

fn parse_card(input: &str) -> Card {
    let (_, numbers_input) = input.split_once(": ").unwrap();
    let (winning_numbers_input, actual_numbers_input) = numbers_input.split_once(" | ").unwrap();
    Card {
        winning_numbers: parse_numbers(winning_numbers_input),
        actual_numbers: parse_numbers(actual_numbers_input),
    }
}

fn parse_numbers(input: &str) -> HashSet<u32> {
    input
        .split_whitespace()
        .map(|number| number.parse::<u32>().unwrap())
        .collect()
}

fn compute_win_count(card: &Card) -> u32 {
    card.actual_numbers
        .iter()
        .filter(|number| card.winning_numbers.contains(number))
        .count() as u32
}

#[derive(Clone)]
struct Card {
    winning_numbers: HashSet<u32>,
    actual_numbers: HashSet<u32>,
}
