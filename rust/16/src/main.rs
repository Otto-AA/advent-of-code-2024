use std::{collections::HashSet, env};

use anyhow::Result;
use maze::Maze;
use parser::load_input;
use pathfinding::prelude::astar_bag;
use utils::{Direction, Point};

mod maze;
mod parser;
mod utils;

#[derive(Eq, PartialEq, Hash, Clone)]
struct Step {
    previous: Option<Point>,
    current: Point,
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let file_name = args.get(1).expect("Please provide an input filename");
    let input = load_input(file_name)?;
    let maze = input.maze;
    let start = input.start;
    let end = input.end;

    println!("From {:?} to {:?}", start, end);
    println!("{}", maze);

    let result = astar_bag(
        &Step {
            previous: None,
            current: start,
        },
        |p| successors(&maze, p),
        |_| 0,
        |p| p.current.row == end.row && p.current.col == end.col,
    );

    if let Some((paths, cost)) = result {
        println!("Found path with cost {}!", cost);

        let mut points = HashSet::new();
        for path in paths {
            for step in path {
                points.insert(step.current);
            }
        }

        let points_on_best_paths = points.len();
        println!(
            "There are {} points on multiple paths",
            points_on_best_paths
        );
    } else {
        println!("No path found :(");
    }

    Ok(())
}

fn successors(maze: &Maze, step: &Step) -> Vec<(Step, usize)> {
    let neighbours: Vec<Step> = step
        .current
        .neighbours()
        .into_iter()
        .filter(|other| maze.is_walkable(other))
        .map(|n| Step {
            previous: Some(step.current.clone()),
            current: n,
        })
        .collect();

    let mut previous_direction = None;
    if let Some(previous) = step.clone().previous {
        previous_direction = Some(
            previous
                .direction_to(&step.current)
                .expect("Could not compute direction between fields"),
        );
    }

    neighbours
        .into_iter()
        .map(|p| {
            (
                p.clone(),
                1 + calculate_turning_costs(
                    previous_direction.as_ref(),
                    &p.previous
                        .unwrap()
                        .direction_to(&p.current)
                        .expect("Could not compute direction between fields"),
                ),
            )
        })
        .collect()
}

fn calculate_turning_costs(
    current_direction: Option<&Direction>,
    next_direction: &Direction,
) -> usize {
    let current_direction = current_direction.unwrap_or(&Direction::Right);
    match (current_direction, next_direction) {
        (Direction::Up, Direction::Up) => 0,
        (Direction::Up, Direction::Right) => 1000,
        (Direction::Up, Direction::Down) => 2000,
        (Direction::Up, Direction::Left) => 1000,
        (Direction::Right, Direction::Up) => 1000,
        (Direction::Right, Direction::Right) => 0,
        (Direction::Right, Direction::Down) => 1000,
        (Direction::Right, Direction::Left) => 2000,
        (Direction::Down, Direction::Up) => 2000,
        (Direction::Down, Direction::Right) => 1000,
        (Direction::Down, Direction::Down) => 0,
        (Direction::Down, Direction::Left) => 1000,
        (Direction::Left, Direction::Up) => 1000,
        (Direction::Left, Direction::Right) => 2000,
        (Direction::Left, Direction::Down) => 1000,
        (Direction::Left, Direction::Left) => 0,
    }
}
