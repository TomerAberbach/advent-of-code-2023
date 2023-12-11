use std::{
    collections::HashSet,
    fs::File,
    io::{self, prelude::*, BufReader},
};

fn main() -> io::Result<()> {
    let file = File::open("../../input.txt")?;
    let lines = BufReader::new(file).lines().map(|line| line.unwrap());
    let grid = Grid::from_iter(lines);

    let starting_position = grid.starting_position();
    let mut current_positions: Vec<(usize, usize, u32)> =
        vec![(starting_position.0, starting_position.1, 0)];
    let mut visited_positions: HashSet<(usize, usize)> = HashSet::new();
    visited_positions.insert(starting_position);
    let mut longest_length = 0;

    while !current_positions.is_empty() {
        let mut new_positions = Vec::new();
        for (row_index, column_index, length) in current_positions {
            longest_length = longest_length.max(length);
            for position in grid.get_connected_positions((row_index, column_index)) {
                if visited_positions.contains(&position) {
                    continue;
                }

                visited_positions.insert(position);
                new_positions.push((position.0, position.1, length + 1));
            }
        }

        current_positions = new_positions;
    }

    println!("{}", longest_length);

    Ok(())
}

struct Grid {
    tiles: Vec<Vec<Tile>>,
}

impl FromIterator<String> for Grid {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Grid {
            tiles: iter
                .into_iter()
                .map(|s| s.chars().map(|c| c.try_into().unwrap()).collect())
                .collect(),
        }
    }
}

impl Grid {
    fn starting_position(&self) -> (usize, usize) {
        self.tiles
            .iter()
            .map(|row| {
                row.iter()
                    .enumerate()
                    .find(|(_, tile)| **tile == Tile::StartingPosition)
                    .map(|(column_index, _)| column_index)
            })
            .enumerate()
            .flat_map(|(row_index, column_index)| {
                column_index.map(|column_index| (row_index, column_index))
            })
            .nth(0)
            .unwrap()
    }

    fn get(&self, row_index: usize, column_index: usize) -> &Tile {
        self.tiles
            .get(row_index)
            .unwrap()
            .get(column_index)
            .unwrap()
    }

    fn get_connected_positions(
        &self,
        (row_index, column_index): (usize, usize),
    ) -> Vec<(usize, usize)> {
        let tile = self.get(row_index, column_index);
        let mut connected_positions = Vec::new();

        if row_index > 0
            && tile.has_north_opening()
            && self.get(row_index - 1, column_index).has_south_opening()
        {
            connected_positions.push((row_index - 1, column_index));
        }
        if row_index < self.tiles.len() - 1
            && tile.has_south_opening()
            && self.get(row_index + 1, column_index).has_north_opening()
        {
            connected_positions.push((row_index + 1, column_index))
        }
        if column_index > 0
            && tile.has_west_opening()
            && self.get(row_index, column_index - 1).has_east_opening()
        {
            connected_positions.push((row_index, column_index - 1));
        }
        if row_index < self.tiles.first().unwrap().len() - 1
            && tile.has_east_opening()
            && self.get(row_index, column_index + 1).has_west_opening()
        {
            connected_positions.push((row_index, column_index + 1))
        }

        connected_positions
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Tile {
    VerticalPipe,
    HorizontalPipe,
    NorthToEastPipe,
    NorthToWestPipe,
    SouthToEastPipe,
    SouthToWestPipe,
    Ground,
    StartingPosition,
}

impl TryFrom<char> for Tile {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '|' => Ok(Tile::VerticalPipe),
            '-' => Ok(Tile::HorizontalPipe),
            'L' => Ok(Tile::NorthToEastPipe),
            'J' => Ok(Tile::NorthToWestPipe),
            'F' => Ok(Tile::SouthToEastPipe),
            '7' => Ok(Tile::SouthToWestPipe),
            '.' => Ok(Tile::Ground),
            'S' => Ok(Tile::StartingPosition),
            _ => Err(()),
        }
    }
}

impl Tile {
    fn has_north_opening(&self) -> bool {
        match self {
            Tile::VerticalPipe => true,
            Tile::HorizontalPipe => false,
            Tile::NorthToEastPipe => true,
            Tile::NorthToWestPipe => true,
            Tile::SouthToEastPipe => false,
            Tile::SouthToWestPipe => false,
            Tile::Ground => false,
            Tile::StartingPosition => true,
        }
    }

    fn has_south_opening(&self) -> bool {
        match self {
            Tile::VerticalPipe => true,
            Tile::HorizontalPipe => false,
            Tile::NorthToEastPipe => false,
            Tile::NorthToWestPipe => false,
            Tile::SouthToEastPipe => true,
            Tile::SouthToWestPipe => true,
            Tile::Ground => false,
            Tile::StartingPosition => true,
        }
    }

    fn has_east_opening(&self) -> bool {
        match self {
            Tile::VerticalPipe => false,
            Tile::HorizontalPipe => true,
            Tile::NorthToEastPipe => true,
            Tile::NorthToWestPipe => false,
            Tile::SouthToEastPipe => true,
            Tile::SouthToWestPipe => false,
            Tile::Ground => false,
            Tile::StartingPosition => true,
        }
    }

    fn has_west_opening(&self) -> bool {
        match self {
            Tile::VerticalPipe => false,
            Tile::HorizontalPipe => true,
            Tile::NorthToEastPipe => false,
            Tile::NorthToWestPipe => true,
            Tile::SouthToEastPipe => false,
            Tile::SouthToWestPipe => true,
            Tile::Ground => false,
            Tile::StartingPosition => true,
        }
    }
}
