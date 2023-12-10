use std::{
    cmp::Ordering,
    fs::File,
    io::{self, prelude::*, BufReader},
};

fn main() -> io::Result<()> {
    let file = File::open("../../input.txt")?;
    let almanac = Almanac::parse(&file)?;

    let lowest_location = almanac
        .seeds
        .iter()
        .flat_map(|&(start, end)| {
            println!("({}, {})", start, end);
            (start..end).map(|seed| {
                almanac
                    .maps
                    .iter()
                    .fold(seed, |number, map| map.get(number).unwrap_or(number))
            })
        })
        .min()
        .unwrap();
    println!("{}", lowest_location);
    Ok(())
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<(u64, u64)>,
    maps: Vec<RangeMap>,
}

impl Almanac {
    fn parse(file: &File) -> io::Result<Almanac> {
        let mut almanac = Almanac {
            seeds: Vec::new(),
            maps: Vec::new(),
        };
        let mut current_map: Option<RangeMap> = None;
        for line in BufReader::new(file).lines() {
            let line = line?;

            if line.is_empty() {
                if let Some(map) = current_map {
                    almanac.maps.push(map);
                }
                current_map = None;
                continue;
            }

            if line.starts_with("seeds: ") {
                almanac.seeds.extend(
                    parse_numbers(&line["seeds: ".len()..])
                        .chunks_exact(2)
                        .map(|pair| (pair[0], (pair[0] + pair[1]))),
                );
                continue;
            }

            if line.ends_with("map:") {
                current_map = Some(RangeMap {
                    mappings: Vec::new(),
                });
                continue;
            }

            let numbers = parse_numbers(&line);
            let destination_range_start = numbers[0];
            let source_range_start = numbers[1];
            let range_length = numbers[2];
            if let Some(map) = current_map.as_mut() {
                map.mappings.push(RangeMapping {
                    source_range_start,
                    destination_range_start,
                    range_length,
                });
            }
        }
        if let Some(map) = current_map {
            almanac.maps.push(map);
        }

        for map in &mut almanac.maps {
            map.mappings
                .sort_by_key(|mapping| mapping.source_range_start)
        }

        Ok(almanac)
    }
}

fn parse_numbers(input: &str) -> Vec<u64> {
    input
        .split_whitespace()
        .map(|string| string.parse::<u64>().unwrap())
        .collect()
}

#[derive(Debug)]
struct RangeMap {
    mappings: Vec<RangeMapping>,
}

impl RangeMap {
    fn get(&self, key: u64) -> Option<u64> {
        self.mappings
            .binary_search_by(|mapping| {
                if key < mapping.source_range_start {
                    Ordering::Greater
                } else if key >= mapping.source_range_start + mapping.range_length {
                    Ordering::Less
                } else {
                    Ordering::Equal
                }
            })
            .map(|index| {
                self.mappings[index].destination_range_start
                    + (key - self.mappings[index].source_range_start)
            })
            .ok()
    }
}

#[derive(Debug)]
struct RangeMapping {
    source_range_start: u64,
    destination_range_start: u64,
    range_length: u64,
}
