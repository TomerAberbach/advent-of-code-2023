use std::{
    fs::File,
    io::{self, prelude::*, BufReader},
};

fn main() -> io::Result<()> {
    let file = File::open("../../input.txt")?;
    let id_sum: u32 = BufReader::new(file)
        .lines()
        .map(|line| parse_game(&line.unwrap()))
        .map(|game| compute_power(&get_minimum_ball_set(&game)))
        .sum();
    println!("{}", id_sum);
    Ok(())
}

fn parse_game(line: &str) -> Game {
    let parts = line.split(": ").collect::<Vec<_>>();
    Game {
        ball_sets: parse_ball_sets(parts.last().unwrap()),
    }
}

fn parse_ball_sets(string: &str) -> Vec<BallSet> {
    string
        .split("; ")
        .map(|string| parse_ball_set(string))
        .collect()
}

fn parse_ball_set(string: &str) -> BallSet {
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;
    for ball_count_and_color in string.split(", ") {
        let parts = ball_count_and_color.split(" ").collect::<Vec<_>>();
        let ball_count = parts.first().unwrap().parse().unwrap();

        match parts.last().unwrap().as_ref() {
            "red" => red = ball_count,
            "green" => green = ball_count,
            "blue" => blue = ball_count,
            _ => {}
        }
    }

    BallSet { red, green, blue }
}

fn get_minimum_ball_set(game: &Game) -> BallSet {
    let mut min_red = 0;
    let mut min_green = 0;
    let mut min_blue = 0;
    for ball_set in game.ball_sets.iter() {
        min_red = min_red.max(ball_set.red);
        min_green = min_green.max(ball_set.green);
        min_blue = min_blue.max(ball_set.blue);
    }
    BallSet {
        red: min_red,
        green: min_green,
        blue: min_blue,
    }
}

fn compute_power(ball_set: &BallSet) -> u32 {
    ball_set.red as u32 * ball_set.green as u32 * ball_set.blue as u32
}

struct Game {
    ball_sets: Vec<BallSet>,
}

struct BallSet {
    red: u8,
    green: u8,
    blue: u8,
}
