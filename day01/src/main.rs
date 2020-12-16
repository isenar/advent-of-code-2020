use anyhow::{bail, Context, Error, Result};

use std::fs::File;
use std::io::BufRead;
use std::path::Path;

fn parse_input(input: impl AsRef<Path>) -> Result<Vec<u32>> {
    let file = File::open(input)?;
    let lines: Result<Vec<_>, _> = std::io::BufReader::new(file).lines().collect();
    let numbers: Result<Vec<_>, _> = lines
        .with_context(|| "Failed to get lines from the file")?
        .into_iter()
        .map(|line| line.parse::<u32>())
        .collect();

    numbers
        .map_err(Error::msg)
        .with_context(|| "Failed to parse String into u32")
}

fn find_pair(numbers: &[u32]) -> Result<u32> {
    for first in numbers {
        for second in numbers {
            if first + second == 2020 {
                return Ok(first * second);
            }
        }
    }

    bail!("Couldn't find a pair such that the sum of its elements == 2020")
}

fn find_three(numbers: &[u32]) -> Result<u32> {
    for first in numbers {
        for second in numbers {
            for third in numbers {
                if first + second + third == 2020 {
                    return Ok(first * second * third);
                }
            }
        }
    }

    bail!("Couldn't find 3 numbers such that their sum gives 2020")
}

fn main() -> Result<()> {
    let numbers = parse_input("day01/input.txt")?;
    let product_of_two = find_pair(&numbers)?;
    let product_of_three = find_three(&numbers)?;

    println!("PRODUCT OF 2: {}", product_of_two);
    println!("PRODUCT OF 3: {}", product_of_three);

    Ok(())
}
