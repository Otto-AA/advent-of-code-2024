use std::fs::read_to_string;

use anyhow::{Context, Result};

use crate::map::Point;

pub fn parse_points(path: &str) -> Result<Vec<Point>> {
    read_to_string(path)?
        .lines()
        .map(|line| {
            let nums: Vec<&str> = line.split(",").collect();
            let col = nums.get(0).context("Could not read x value")?.parse()?;
            let row = nums.get(1).context("Could not read x value")?.parse()?;

            Ok(Point { row, col })
        })
        .collect()
}
