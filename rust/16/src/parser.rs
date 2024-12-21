use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::{Context, Result};

use crate::{
    maze::{Field, Maze},
    utils::Point,
};

pub fn load_input(path: &str) -> Result<Input> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut map: Vec<Vec<Field>> = Vec::new();
    let mut start = None;
    let mut end = None;

    for (nth_row, line) in reader.lines().flatten().enumerate() {
        let mut row = Vec::new();
        for (nth_col, c) in line.chars().enumerate() {
            row.push(match c {
                '#' => Field::Wall,
                _ => Field::Space,
            });

            if c == 'S' {
                start = Some(Point {
                    row: nth_row,
                    col: nth_col,
                });
            } else if c == 'E' {
                end = Some(Point {
                    row: nth_row,
                    col: nth_col,
                });
            }
        }
        map.push(row);
    }

    Ok(Input {
        maze: Maze::new(map),
        start: start.context("Could not find start")?,
        end: end.context("Could not find end")?,
    })
}

pub struct Input {
    pub maze: Maze,
    pub start: Point,
    pub end: Point,
}
