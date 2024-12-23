use anyhow::{Context, Result};
use map::{MemorySpace, Point};
use parse::parse_points;
use pathfinding::prelude::astar;

mod map;
mod parse;

fn main() -> Result<()> {
    let n = part_one("input.txt", 71, 71, 1024)?;
    println!("Steps: {n}");

    let point = part_two("input.txt", 71, 71)?;
    println!("Blocking point: {},{}", point.col, point.row);

    Ok(())
}

pub fn part_one(path: &str, width: usize, height: usize, time_ns: usize) -> Result<usize> {
    let mut memory_space = MemorySpace::new(width, height);

    parse_points(path)?
        .iter()
        .take(time_ns)
        .for_each(|p| memory_space.corrupt(p));

    println!("{memory_space}");

    let start = Point { row: 0, col: 0 };
    let end = Point {
        row: height - 1,
        col: width - 1,
    };

    let path =
        shortest_path(&memory_space, &start, &end).context("Could not find shortest path")?;

    Ok(path.len() - 1)
}

pub fn part_two(path: &str, width: usize, height: usize) -> Result<Point> {
    let mut memory_space = MemorySpace::new(width, height);
    let start = Point { row: 0, col: 0 };
    let end = Point {
        row: height - 1,
        col: width - 1,
    };

    parse_points(path)?
        .into_iter()
        .find(|p| {
            memory_space.corrupt(p);
            let path = shortest_path(&memory_space, &start, &end);
            path.is_none()
        })
        .context("Could not find a point that blocks everything")
}

pub fn shortest_path(memory_space: &MemorySpace, start: &Point, end: &Point) -> Option<Vec<Point>> {
    let result = astar(
        start,
        |p| successors(&memory_space, p),
        |p| heuristic(p, &end),
        |p| p == end,
    );
    let (path, _) = result?;
    Some(path)
}

pub fn successors(memory_space: &MemorySpace, point: &Point) -> Vec<(Point, usize)> {
    memory_space
        .neighbours(point)
        .into_iter()
        .map(|p| (p, 1))
        .collect()
}

pub fn heuristic(point: &Point, end: &Point) -> usize {
    point.row.abs_diff(end.row) + point.col.abs_diff(end.col)
}
