use std::{
    fs::File,
    io::{self, prelude::*, BufReader},
};

fn main() -> io::Result<()> {
    let file = File::open("../../input.txt")?;
    let sum: u32 = BufReader::new(file)
        .lines()
        .map(|line| parse_line(&line.unwrap()))
        .sum();
    println!("{}", sum);
    Ok(())
}

fn parse_line(line: &String) -> u32 {
    let digits: Vec<_> = line.chars().flat_map(|c| c.to_digit(10)).collect();
    assert!(!digits.is_empty());
    digits.first().unwrap() * 10 + digits.last().unwrap()
}
