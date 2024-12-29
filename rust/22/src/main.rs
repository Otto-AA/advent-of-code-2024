use std::{env, fs::read_to_string};

use anyhow::{Context, Result};
use monkey_market::efficient_optimum_sequence;
use secrets::iter_secrets;

mod monkey_market;
mod prices;
mod secrets;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let path = args.get(1).expect("Please provide an input path");
    let numbers = parse_input(path)?;

    println!("Part one: {}", part_one(&numbers));
    println!("Part two: {:?}", part_two(&numbers));

    Ok(())
}

fn parse_input(path: &str) -> Result<Vec<u64>> {
    read_to_string(path)?
        .lines()
        .map(|l| l.parse().context("Could not parse number"))
        .collect()
}

fn part_one(numbers: &Vec<u64>) -> u64 {
    numbers
        .iter()
        .map(|&n| iter_secrets(n).nth(1999).unwrap())
        .sum()
}

fn part_two(numbers: &Vec<u64>) -> ([i8; 4], u64) {
    efficient_optimum_sequence(numbers, 2000)
}
