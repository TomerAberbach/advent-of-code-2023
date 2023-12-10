use std::{
    collections::HashMap,
    fs::File,
    io::{self, prelude::*, BufReader},
};

fn main() -> io::Result<()> {
    let file = File::open("../../input.txt")?;
    let mut lines = BufReader::new(file).lines();

    let directions = parse_directions(&lines.next().unwrap().unwrap());
    let network = Network::from_iter(lines.skip(1).map(|line| line.unwrap()));

    let mut current_node = "AAA";
    let mut steps = 0;
    for direction in directions.iter().cycle() {
        if current_node == "ZZZ" {
            break;
        }

        let (left, right) = network.nodes.get(current_node).unwrap();
        match direction {
            Direction::Left => current_node = left,
            Direction::Right => current_node = right,
        }
        steps += 1;
    }

    println!("{}", steps);

    Ok(())
}

fn parse_directions(s: &str) -> Vec<Direction> {
    s.chars().map(|c| c.try_into().unwrap()).collect()
}

enum Direction {
    Left,
    Right,
}

impl TryFrom<char> for Direction {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'L' => Ok(Direction::Left),
            'R' => Ok(Direction::Right),
            _ => Err(()),
        }
    }
}

struct Network {
    nodes: HashMap<String, (String, String)>,
}

impl FromIterator<String> for Network {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Network {
            nodes: iter
                .into_iter()
                .map(|s| {
                    let (key, value) = s.split_once(" = ").unwrap();
                    let (left, right) = value[1..value.len() - 1].split_once(", ").unwrap();
                    (key.to_string(), (left.to_string(), right.to_string()))
                })
                .collect(),
        }
    }
}
