use std::{
    fs::File,
    io::{self, prelude::*, BufReader},
};

fn main() -> io::Result<()> {
    let file = File::open("../../input.txt")?;
    let schematic: Vec<Vec<char>> = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    let mut sum = 0;
    for (i, row) in schematic.iter().enumerate() {
        let mut digits: Vec<u32> = Vec::new();
        let mut is_adjacent_to_symbol = false;
        for (j, c) in row.iter().enumerate() {
            if let Some(digit) = c.to_digit(10) {
                is_adjacent_to_symbol =
                    is_adjacent_to_symbol || check_adjacent_to_symbol(&schematic, i, j);
                digits.push(digit);
            } else {
                if is_adjacent_to_symbol {
                    sum += compute_number(&digits);
                }
                digits = Vec::new();
                is_adjacent_to_symbol = false;
            }
        }

        if is_adjacent_to_symbol {
            sum += compute_number(&digits);
        }
    }

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

fn check_adjacent_to_symbol(schematic: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
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

    coordinates.iter().any(|(i, j)| {
        schematic
            .get(*i)
            .and_then(|row| row.get(*j))
            .filter(|c| is_symbol(c))
            .is_some()
    })
}

fn is_symbol(c: &char) -> bool {
    !c.is_digit(10) && *c != '.'
}
