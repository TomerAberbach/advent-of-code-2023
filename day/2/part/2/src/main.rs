use std::{
    fs::File,
    io::{self, prelude::*, BufReader},
};

fn main() -> io::Result<()> {
    let file = File::open("../../input.txt")?;
    let id_sum: u32 = BufReader::new(file)
        .lines()
        .map(|line| parse_game(&line.unwrap()))
        .map(|game| compute_power(&get_minimum_cube_set(&game)))
        .sum();
    println!("{}", id_sum);
    Ok(())
}

fn parse_game(line: &str) -> Game {
    let parts = line.split(": ").collect::<Vec<_>>();
    Game {
        cube_sets: parse_cube_sets(parts.last().unwrap()),
    }
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

fn get_minimum_cube_set(game: &Game) -> CubeSet {
    let mut min_red = 0;
    let mut min_green = 0;
    let mut min_blue = 0;
    for cube_set in game.cube_sets.iter() {
        min_red = min_red.max(cube_set.red);
        min_green = min_green.max(cube_set.green);
        min_blue = min_blue.max(cube_set.blue);
    }
    CubeSet {
        red: min_red,
        green: min_green,
        blue: min_blue,
    }
}

fn compute_power(cube_set: &CubeSet) -> u32 {
    cube_set.red as u32 * cube_set.green as u32 * cube_set.blue as u32
}

struct Game {
    cube_sets: Vec<CubeSet>,
}

struct CubeSet {
    red: u8,
    green: u8,
    blue: u8,
}
