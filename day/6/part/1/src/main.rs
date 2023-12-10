use std::{
    fs::File,
    io::{self, prelude::*, BufReader},
};

fn main() -> io::Result<()> {
    let file = File::open("../../input.txt")?;
    let numbers: Vec<_> = BufReader::new(file)
        .lines()
        .take(2)
        .map(|line| parse_line(&line.unwrap()))
        .collect();
    let races: Vec<Race> = numbers[0]
        .iter()
        .zip(&numbers[1])
        .map(|(&time, &best_distance)| Race {
            time,
            best_distance,
        })
        .collect();

    let product: u32 = races.iter().map(compute_winning_race_count).product();
    println!("{}", product);

    Ok(())
}

fn parse_line(line: &String) -> Vec<u32> {
    let (_, numbers) = line.split_once(":").unwrap();
    numbers
        .trim()
        .split_whitespace()
        .map(|number| number.parse::<u32>().unwrap())
        .collect()
}

struct Race {
    time: u32,
    best_distance: u32,
}

fn compute_winning_race_count(race: &Race) -> u32 {
    (0..=race.time)
        .map(|time_holding_button| compute_distance(race.time, time_holding_button))
        .filter(|distance| distance > &race.best_distance)
        .count() as u32
}

fn compute_distance(race_time: u32, time_holding_button: u32) -> u32 {
    let remaining_time = race_time - time_holding_button;
    let speed = time_holding_button;
    remaining_time * speed
}
