use std::{
    char,
    fmt::Display,
    ops::{Index, IndexMut},
};

use crate::parser::Direction;

pub(crate) struct Warehouse {
    inner: Vec<Vec<Field>>,
    width: usize,
    height: usize,
}

impl Warehouse {
    pub(crate) fn new(rows: Vec<Vec<Field>>) -> Self {
        let height = rows.len();
        let width = rows.first().expect("Did not find a first row").len();

        Warehouse {
            height,
            width,
            inner: rows,
        }
    }

    pub(crate) fn scaled_up(original: &Warehouse) -> Self {
        let height = original.height;
        let width = original.width * 2;
        let rows = original
            .inner
            .iter()
            .map(|row| {
                row.iter()
                    .flat_map(|f| match f {
                        Field::Empty => [Field::Empty, Field::Empty],
                        Field::Wall => [Field::Wall, Field::Wall],
                        Field::Box => [
                            Field::WideBox(WideBoxSide::Left),
                            Field::WideBox(WideBoxSide::Right),
                        ],
                        Field::WideBox(_) => panic!("Cannot scale up a wide box"),
                        Field::Robot => [Field::Robot, Field::Empty],
                    })
                    .collect()
            })
            .collect();

        Warehouse {
            height,
            width,
            inner: rows,
        }
    }

    pub(crate) fn in_range(&self, point: &Point) -> bool {
        (0..self.width).contains(&point.col) && (0..self.height).contains(&point.row)
    }

    pub(crate) fn next_in_direction<'a>(
        &'a self,
        start: &'a Point,
        direction: &Direction,
    ) -> Option<(Point, &'a Field)> {
        self.iter_direction(start, direction).nth(1)
    }

    pub(crate) fn gps(&self) -> usize {
        self.iter_fields()
            .filter(|(_, f)| matches!(f, Field::Box | Field::WideBox(WideBoxSide::Left)))
            .map(|(p, _)| 100 * p.row + p.col)
            .sum()
    }

    pub(crate) fn iter_fields(&self) -> impl Iterator<Item = (Point, &'_ Field)> {
        iter_points_in_rectangle(
            &Point { row: 0, col: 0 },
            &Point {
                row: self.height - 1,
                col: self.width - 1,
            },
        )
        .map(|p| {
            let field = &self[&p];
            (p, field)
        })
    }

    pub(crate) fn iter_direction<'a>(
        &'a self,
        start: &'a Point,
        direction: &Direction,
    ) -> impl Iterator<Item = (Point, &'a Field)> {
        start
            .iter_direction(direction.clone())
            .take_while(|p| self.in_range(p))
            .map(|p| {
                let field = &self[&p];
                (p, field)
            })
    }
}

#[derive(Clone, Debug)]
pub(crate) struct Point {
    pub(crate) row: usize,
    pub(crate) col: usize,
}

impl Point {
    /// yield points in direction, including self as a first point
    /// stops when limits of usize are reached (0 or max value)
    pub(crate) fn iter_direction(&self, direction: Direction) -> PointIterator {
        PointIterator::in_direction(self.clone(), direction)
    }
}

pub(crate) fn iter_points_in_rectangle(
    top_left: &Point,
    bottom_right: &Point,
) -> impl Iterator<Item = Point> {
    let top_left = top_left.clone();
    let bottom_right = bottom_right.clone();
    (top_left.row..=bottom_right.row)
        .flat_map(move |row| (top_left.col..=bottom_right.col).map(move |col| Point { row, col }))
}

pub(crate) struct PointIterator {
    next: Option<Point>,
    offset_row: isize,
    offset_col: isize,
}

impl PointIterator {
    pub(crate) fn in_direction(start: Point, direction: Direction) -> Self {
        let (offset_row, offset_col): (isize, isize) = match direction {
            Direction::Left => (0, -1),
            Direction::Up => (-1, 0),
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
        };

        PointIterator {
            next: Some(start),
            offset_row,
            offset_col,
        }
    }
}

impl Iterator for PointIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let point = self.next.take()?;

        let new_row = point.row.checked_add_signed(self.offset_row);
        let new_col = point.col.checked_add_signed(self.offset_col);

        if let (Some(row), Some(col)) = (new_row, new_col) {
            self.next = Some(Point { row, col });
        } else {
            self.next = None;
        }

        Some(point)
    }
}

impl Index<&Point> for Warehouse {
    type Output = Field;

    fn index(&self, index: &Point) -> &Self::Output {
        &self.inner[index.row][index.col]
    }
}

impl IndexMut<&Point> for Warehouse {
    fn index_mut(&mut self, index: &Point) -> &mut Self::Output {
        &mut self.inner[index.row][index.col]
    }
}

impl Display for Warehouse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.inner {
            let line: Vec<String> = row.iter().map(|e| e.to_string()).collect();
            writeln!(f, "{}", line.join(""))?;
        }
        Ok(())
    }
}

#[derive(Clone)]
pub(crate) enum Field {
    Empty,
    Wall,
    Box,
    WideBox(WideBoxSide),
    Robot,
}

#[derive(Clone)]
pub(crate) enum WideBoxSide {
    Left,
    Right,
}

impl WideBoxSide {
    pub fn other_side(&self, position: &Point) -> (Self, Point) {
        match self {
            WideBoxSide::Left => (
                WideBoxSide::Right,
                Point {
                    row: position.row,
                    col: position.col + 1,
                },
            ),
            WideBoxSide::Right => (
                WideBoxSide::Left,
                Point {
                    row: position.row,
                    col: position.col - 1,
                },
            ),
        }
    }
}

impl Field {
    pub(crate) fn from(c: char) -> Self {
        match c {
            '.' => Field::Empty,
            '#' => Field::Wall,
            'O' => Field::Box,
            '@' => Field::Robot,
            _ => panic!("Invalid char"),
        }
    }
}

impl Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Field::Empty => write!(f, "."),
            Field::Wall => write!(f, "#"),
            Field::Box => write!(f, "O"),
            Field::Robot => write!(f, "@"),
            Field::WideBox(WideBoxSide::Left) => write!(f, "["),
            Field::WideBox(WideBoxSide::Right) => write!(f, "]"),
        }
    }
}
