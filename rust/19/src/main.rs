use std::{collections::HashMap, env, fs};

use anyhow::{Context, Result};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let path = args.get(1).expect("Please provide an input file path");

    let (towels, designs) = parse_input(path)?;
    let possible_designs = designs
        .iter()
        .filter(|design| is_composable(&towels, design))
        .count();

    println!("Possible designs: {possible_designs}");

    let all_possibilities: u64 =
        count_composability(&towels, designs.iter().map(|s| &s[..]).collect());

    println!("Sum: {all_possibilities}");

    Ok(())
}

fn parse_input(path: &str) -> Result<(Vec<String>, Vec<String>)> {
    let s = fs::read_to_string(path)?;

    let towels = s.lines().next().context("Could not read towels")?;
    let towels = towels.split(", ").map(|s| s.to_string()).collect();
    let designs = s.lines().skip(2).map(|s| s.to_string()).collect();

    Ok((towels, designs))
}

fn is_composable(towels: &[String], design: &str) -> bool {
    let mut known_composabilities = towels.iter().map(|s| (s.clone(), true)).collect();
    is_composable_rec(&mut known_composabilities, design)
}

fn is_composable_rec(known_composabilities: &mut HashMap<String, bool>, design: &str) -> bool {
    if design.is_empty() {
        return true;
    }
    if known_composabilities.contains_key(design) {
        return *known_composabilities.get(design).unwrap();
    }
    for i in 1..design.len() + 1 {
        let left = &design[..i];
        let right = &design[i..];
        let left_is_composable = *known_composabilities.get(left).unwrap_or(&false);
        if left_is_composable && is_composable_rec(known_composabilities, right) {
            known_composabilities.insert(design.to_string(), true);
            return true;
        }
    }

    known_composabilities.insert(design.to_string(), false);
    false
}

fn count_composability(towels: &Vec<String>, designs: Vec<&str>) -> u64 {
    let mut cache = HashMap::new();
    designs
        .iter()
        .map(|design| count_composability_rec(towels, &mut cache, design))
        .sum()
}

fn count_composability_rec(
    towels: &Vec<String>,
    cache: &mut HashMap<String, u64>,
    design: &str,
) -> u64 {
    if design.is_empty() {
        return 1;
    }
    if cache.contains_key(design) {
        return *cache.get(design).unwrap();
    }
    let mut solutions = 0;
    for towel in towels {
        if design.starts_with(towel) {
            solutions += count_composability_rec(towels, cache, &design[towel.len()..]);
        }
    }

    cache.insert(design.to_string(), solutions);
    solutions
}

#[cfg(test)]
mod tests {
    use crate::{count_composability, is_composable};

    #[test]
    fn test_is_composable() {
        let towels = ["abc".to_string(), "def".to_string()];
        let designs = [
            "abcdef".to_string(),
            "abcd".to_string(),
            "def".to_string(),
            "bbb".to_string(),
        ];

        let composable: Vec<bool> = designs.iter().map(|d| is_composable(&towels, d)).collect();

        assert_eq!(vec![true, false, true, false], composable);
    }

    #[test]
    fn test_count_composability_simple() {
        let towels = vec!["abc".to_string()];

        let count = count_composability(&towels, vec!["abc"]);

        assert_eq!(1, count);
    }

    #[test]
    fn test_count_composability_non_overlapping() {
        // ab-c-def, ab-cd-ef
        let towels = vec![
            "ab".to_string(),
            "c".to_string(),
            "cd".to_string(),
            "ef".to_string(),
            "def".to_string(),
        ];

        let count = count_composability(&towels, vec!["abcdef"]);

        assert_eq!(2, count);
    }

    #[test]
    fn test_count_composability_overlapping() {
        let towels = vec![
            "abc".to_string(),
            "ab".to_string(),
            "bc".to_string(),
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
        ];

        let count = count_composability(&towels, vec!["abc"]);

        assert_eq!(4, count);
    }
}
