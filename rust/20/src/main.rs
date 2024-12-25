use std::{collections::HashMap, env, fs::read_to_string};

use anyhow::{Context, Error, Result};
use cheat::{cheatcodes, steps_saved};
use parse_inputs::parse_input;
use race_map::{Point, RaceMap};

mod cheat;
mod parse_inputs;
mod race_map;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let path = args.get(1).context("Please provide an input file path")?;
    let min_saved: usize = args
        .get(2)
        .context("Please provide a minimum number of saved ns")?
        .parse()?;

    let track = parse_track(path)?;
    let track = map_to_index(&track);
    println!("Part one: {}", count_cheats(&track, 2, min_saved));
    println!("Part two: {}", count_cheats(&track, 20, min_saved));

    Ok(())
}

fn parse_track(path: &str) -> Result<Vec<Point>> {
    let input = read_to_string(path)?;
    let (map, start, end) = parse_input(&input)?;
    let track = find_track(&map, &start, &end)?;
    Ok(track)
}

fn map_to_index(track: &Vec<Point>) -> HashMap<&Point, usize> {
    track.iter().enumerate().map(|(n, p)| (p, n)).collect()
}

fn count_cheats(track: &HashMap<&Point, usize>, cheat_length: usize, min_saved: usize) -> usize {
    cheats_savings(track, cheat_length)
        .into_iter()
        .filter(|n| *n >= min_saved)
        .count()
}

fn cheats_savings(track: &HashMap<&Point, usize>, cheat_length: usize) -> Vec<usize> {
    let mut saved_counts = Vec::new();
    for (&point, _) in track {
        let cheats = cheatcodes(&track, point, cheat_length);
        let steps_saved: Vec<usize> = cheats
            .iter()
            .map(|c| steps_saved(&track, c))
            .flatten()
            .collect();

        saved_counts.extend(steps_saved);
    }

    saved_counts
}

fn find_track(map: &RaceMap, start: &Point, end: &Point) -> Result<Vec<Point>> {
    let mut track: Vec<Point> = vec![start.clone()];
    let mut prev: Option<Point> = None;
    let mut current: Point = start.clone();

    while current != end.clone() {
        let mut neighbours: Vec<Point> = map.neighbour_tracks(&current);
        if let Some(prev) = prev {
            neighbours.retain(|p| *p != prev);
        }
        if neighbours.len() != 1 {
            return Err(Error::msg(format!("Found {} neighbours", neighbours.len())));
        }
        prev = Some(current);
        current = neighbours.first().unwrap().to_owned();
        track.push(current.clone());
    }

    Ok(track)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_input() -> Result<()> {
        let track = parse_track("input.txt")?;
        let track = map_to_index(&track);

        let result = count_cheats(&track, 2, 100);

        assert_eq!(1499, result);
        Ok(())
    }

    #[test]
    fn part_two_sample() -> Result<()> {
        let track = parse_track("sample.txt")?;
        let track = map_to_index(&track);

        let result = count_cheats(&track, 20, 50);

        assert_eq!(285, result);
        Ok(())
    }
}
