use std::{
    fmt::{Debug, Display},
    fs, io,
};

use crate::warehouse::{Field, Warehouse};

pub(crate) fn parse_file(path: &str) -> Result<Input, io::Error> {
    let input = fs::read_to_string(path)?;
    let mut rows: Vec<Vec<Field>> = Vec::new();
    let mut moves: Vec<Direction> = Vec::new();

    let mut reading_warehouse = true;

    for line in input.lines() {
        if line.is_empty() {
            reading_warehouse = false;
        } else if reading_warehouse {
            let row = line.chars().map(Field::from).collect();
            rows.push(row);
        } else {
            moves.extend(line.chars().map(|c| match c {
                '<' => Direction::Left,
                '^' => Direction::Up,
                '>' => Direction::Right,
                'v' => Direction::Down,
                _ => panic!("Invalid move char"),
            }));
        }
    }

    let warehouse = Warehouse::new(rows);

    Ok(Input { warehouse, moves })
}

pub(crate) struct Input {
    pub(crate) warehouse: Warehouse,
    pub(crate) moves: Vec<Direction>,
}

#[derive(Clone)]
pub(crate) enum Direction {
    Left,
    Up,
    Right,
    Down,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Left => write!(f, "<"),
            Direction::Up => write!(f, "^"),
            Direction::Right => write!(f, ">"),
            Direction::Down => write!(f, "v"),
        }
    }
}

impl Debug for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Left => write!(f, "<"),
            Self::Up => write!(f, "^"),
            Self::Right => write!(f, ">"),
            Self::Down => write!(f, "v"),
        }
    }
}
