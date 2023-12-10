use std::{
    fs::File,
    io::{self, prelude::*, BufReader},
};

fn main() -> io::Result<()> {
    let file = File::open("../../input.txt")?;
    let sum: i32 = BufReader::new(file)
        .lines()
        .map(|line| parse_history(&line.unwrap()))
        .map(|history| extrapolate_history(&history))
        .sum();
    println!("{}", sum);
    Ok(())
}

fn parse_history(s: &str) -> Vec<i32> {
    s.split_whitespace().map(|s| s.parse().unwrap()).collect()
}

fn extrapolate_history(history: &Vec<i32>) -> i32 {
    let mut histories = vec![history.clone()];
    while !histories.last().unwrap().iter().all(|&value| value == 0) {
        histories.push(get_differences(&histories.last().unwrap()));
    }

    histories.pop();
    let mut extrapolated_value = 0;
    while !histories.is_empty() {
        extrapolated_value = histories.pop().unwrap().iter().last().unwrap() + extrapolated_value;
    }

    extrapolated_value
}

fn get_differences(history: &Vec<i32>) -> Vec<i32> {
    history
        .windows(2)
        .map(|window| window[1] - window[0])
        .collect()
}
