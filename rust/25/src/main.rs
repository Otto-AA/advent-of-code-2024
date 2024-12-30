use std::env;

use anyhow::Result;
use parser::{parse_input, Input};

mod parser;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let path = args.get(1).expect("Please provide an input file path");

    let input = parse_input(path)?;

    println!("Part one: {}", part_one(&input));

    Ok(())
}

fn part_one(input: &Input) -> usize {
    let mut sum = 0;
    for key in &input.keys {
        for lock in &input.locks {
            if matches(key, lock) {
                sum += 1;
            }
        }
    }
    sum
}

fn matches(key: &[u8; 5], lock: &[u8; 5]) -> bool {
    (0..5).all(|i| key[i] + lock[i] <= 5)
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use crate::{parser::parse_input, part_one};

    #[test]
    fn part_one_sample() -> Result<()> {
        let input = parse_input("sample.txt")?;

        let result = part_one(&input);

        assert_eq!(3, result);
        Ok(())
    }
}
