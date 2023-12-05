use std::{
    collections::HashSet,
    fs::File,
    io::{self, prelude::*, BufReader},
};

fn main() -> io::Result<()> {
    let file = File::open("../../input.txt")?;
    let points: u32 = BufReader::new(file)
        .lines()
        .map(|line| compute_card_points(&parse_card(&line.unwrap())))
        .sum();
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

fn compute_card_points(card: &Card) -> u32 {
    let winning_count = card
        .actual_numbers
        .iter()
        .filter(|number| card.winning_numbers.contains(number))
        .count() as u32;
    if winning_count == 0 {
        0
    } else {
        2_u32.pow(winning_count - 1)
    }
}

struct Card {
    winning_numbers: HashSet<u32>,
    actual_numbers: HashSet<u32>,
}
