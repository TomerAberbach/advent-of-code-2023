use std::{
    fs::File,
    io::{self, prelude::*, BufReader},
};

fn main() -> io::Result<()> {
    let ordinals: Vec<&str> = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let file = File::open("../../input.txt")?;
    let sum: u32 = BufReader::new(file)
        .lines()
        .map(|line| parse_line(&line.unwrap(), &ordinals))
        .sum();
    println!("{}", sum);
    Ok(())
}

fn parse_line(line: &String, ordinals: &Vec<&str>) -> u32 {
    let first_digit = parse_first_digit(line, ordinals).unwrap();
    let last_digit = parse_last_digit(line, ordinals).unwrap();
    first_digit * 10 + last_digit
}

fn parse_first_digit(line: &String, ordinals: &Vec<&str>) -> Option<u32> {
    let mut current_line: &str = line;
    while !current_line.is_empty() {
        if let Some(index) = ordinals
            .iter()
            .position(|&ordinal| current_line.starts_with(ordinal))
        {
            return Some((index + 1).try_into().unwrap());
        }

        let first_char = current_line.chars().next().unwrap();
        if let Some(digit) = first_char.to_digit(10) {
            return Some(digit);
        }

        current_line = &current_line[first_char.len_utf8()..];
    }

    None
}

fn parse_last_digit(line: &String, ordinals: &Vec<&str>) -> Option<u32> {
    let mut current_line: &str = line;
    while !current_line.is_empty() {
        if let Some(index) = ordinals
            .iter()
            .position(|&ordinal| current_line.ends_with(ordinal))
        {
            return Some((index + 1).try_into().unwrap());
        }

        let last_char = current_line.chars().rev().next().unwrap();
        if let Some(digit) = last_char.to_digit(10) {
            return Some(digit);
        }

        current_line = &current_line[..current_line.len() - last_char.len_utf8()];
    }

    None
}
