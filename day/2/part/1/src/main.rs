use std::{
    fs::File,
    io::{self, prelude::*, BufReader},
};

fn main() -> io::Result<()> {
    let file = File::open("../../input.txt")?;
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
        cube_sets: parse_cube_sets(parts.last().unwrap()),
    }
}

fn parse_id(string: &str) -> u32 {
    string["Game ".len()..].parse().unwrap()
}

fn parse_cube_sets(string: &str) -> Vec<CubeSet> {
    string
        .split("; ")
        .map(|string| parse_cube_set(string))
        .collect()
}

fn parse_cube_set(string: &str) -> CubeSet {
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

    CubeSet { red, green, blue }
}

fn is_game_possible(game: &Game) -> bool {
    game.cube_sets.iter().all(|cube_set| {
        cube_set.red <= BAG_CUBE_SET.red
            && cube_set.green <= BAG_CUBE_SET.green
            && cube_set.blue <= BAG_CUBE_SET.blue
    })
}

const BAG_CUBE_SET: CubeSet = CubeSet {
    red: 12,
    green: 13,
    blue: 14,
};

struct Game {
    id: u32,
    cube_sets: Vec<CubeSet>,
}

struct CubeSet {
    red: u8,
    green: u8,
    blue: u8,
}
