use std::{fmt::Display, ops::Index};

use crate::utils::Point;

pub struct Maze {
    inner: Vec<Vec<Field>>,
    height: usize,
    width: usize,
}

impl Maze {
    pub fn new(map: Vec<Vec<Field>>) -> Self {
        let height = map.len();
        let width = map.first().expect("Maze should not be empty").len();

        Maze {
            inner: map,
            height,
            width,
        }
    }

    pub fn in_range(&self, point: &Point) -> bool {
        point.row < self.height && point.col < self.width
    }

    pub fn is_walkable(&self, point: &Point) -> bool {
        self.in_range(point) && matches!(self[point], Field::Space)
    }
}

impl Display for Maze {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.inner {
            let x: Vec<String> = row.iter().map(|f| f.to_string()).collect();
            writeln!(f, "{}", x.join(""))?;
        }
        Ok(())
    }
}

impl Index<&Point> for Maze {
    type Output = Field;

    fn index(&self, index: &Point) -> &Self::Output {
        &self.inner[index.row][index.col]
    }
}

pub enum Field {
    Wall,
    Space,
}

impl Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Field::Wall => write!(f, "#"),
            Field::Space => write!(f, " "),
        }
    }
}
