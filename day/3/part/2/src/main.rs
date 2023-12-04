use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, prelude::*, BufReader},
};

fn main() -> io::Result<()> {
    let file = File::open("../../input.txt")?;
    let schematic: Vec<Vec<char>> = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    let mut numbers: HashMap<(usize, usize), u32> = HashMap::new();
    let mut gears: HashSet<(usize, usize)> = HashSet::new();

    for (i, row) in schematic.iter().enumerate() {
        let mut js = Vec::new();
        let mut digits = Vec::new();
        for (j, c) in row.iter().enumerate() {
            if let Some(digit) = c.to_digit(10) {
                js.push(j);
                digits.push(digit);
                continue;
            }

            if !digits.is_empty() {
                let number = compute_number(&digits);
                for &j in js.iter() {
                    numbers.insert((i, j), number);
                }
            }

            js = Vec::new();
            digits = Vec::new();

            if *c == '*' {
                gears.insert((i, j));
            }
        }

        if !digits.is_empty() {
            let number = compute_number(&digits);
            for &j in js.iter() {
                numbers.insert((i, j), number);
            }
        }
    }

    let sum: u32 = gears
        .iter()
        .map(|(i, j)| get_adjacent_numbers(&numbers, *i, *j))
        .filter(|adjacent_numbers| adjacent_numbers.len() == 2)
        .map(|adjacent_numbers| adjacent_numbers.iter().product::<u32>())
        .sum();
    println!("{}", sum);

    Ok(())
}

fn compute_number(digits: &Vec<u32>) -> u32 {
    digits
        .iter()
        .rev()
        .enumerate()
        .map(|(index, digit)| 10_u32.pow(index.try_into().unwrap()) * digit)
        .sum()
}

fn get_adjacent_numbers(
    numbers: &HashMap<(usize, usize), u32>,
    i: usize,
    j: usize,
) -> HashSet<u32> {
    let mut coordinates = vec![(i + 1, j), (i, j + 1), (i + 1, j + 1)];
    if i > 0 {
        coordinates.push((i - 1, j));
        coordinates.push((i - 1, j + 1));
    }
    if j > 0 {
        coordinates.push((i, j - 1));
        coordinates.push((i + 1, j - 1));
    }
    if i > 0 && j > 0 {
        coordinates.push((i - 1, j - 1));
    }

    let mut adjacent_numbers = HashSet::new();
    for coordinate in coordinates {
        if let Some(&number) = numbers.get(&coordinate) {
            adjacent_numbers.insert(number);
        }
    }

    adjacent_numbers
}
