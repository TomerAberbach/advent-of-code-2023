use std::{
    fs::File,
    io::{self, prelude::*, BufReader},
};

fn main() -> io::Result<()> {
    let file = File::open("./input.txt")?;
    let id_sum: u32 = BufReader::new(file)
        .lines()
        .map(|line| parse_game(&line.unwrap()))
        .filter(|game| is_game_possible(game))
        .map(|game| game.id)
        .sum();
    println!("{}", id_sum);
    Ok(())
}

fn parse_game(line: &str) -> Game {
    let parts = line.split(": ").collect::<Vec<_>>();
    Game {
        id: parse_id(parts.first().unwrap()),
        ball_sets: parse_ball_sets(parts.last().unwrap()),
    }
}

fn parse_id(string: &str) -> u32 {
    string["Game ".len()..].parse().unwrap()
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

fn is_game_possible(game: &Game) -> bool {
    game.ball_sets.iter().all(|ball_set| {
        ball_set.red <= BAG_BALL_SET.red
            && ball_set.green <= BAG_BALL_SET.green
            && ball_set.blue <= BAG_BALL_SET.blue
    })
}

const BAG_BALL_SET: BallSet = BallSet {
    red: 12,
    green: 13,
    blue: 14,
};

struct Game {
    id: u32,
    ball_sets: Vec<BallSet>,
}

struct BallSet {
    red: u8,
    green: u8,
    blue: u8,
}
