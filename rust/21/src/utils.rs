use anyhow::{Context, Result};

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn to_offset(self) -> Offset {
        match self {
            Direction::Up => Offset { row: -1, col: 0 },
            Direction::Right => Offset { row: 0, col: 1 },
            Direction::Down => Offset { row: 1, col: 0 },
            Direction::Left => Offset { row: 0, col: -1 },
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct Point {
    pub row: usize,
    pub col: usize,
}

#[derive(Debug)]
pub struct Offset {
    pub row: isize,
    pub col: isize,
}

pub struct Direction2D {
    pub horizontal: Option<Direction>,
    pub vertical: Option<Direction>,
}

impl Point {
    pub fn add(&mut self, offset: &Offset) -> Result<()> {
        self.row = self
            .row
            .checked_add_signed(offset.row)
            .context("Could not add offset to point")?;
        self.col = self
            .col
            .checked_add_signed(offset.col)
            .context("Could not add offset to point")?;
        Ok(())
    }

    pub fn distance(&self, other: &Point) -> Offset {
        let diff_col = isize::try_from(other.col).unwrap() - isize::try_from(self.col).unwrap();
        let diff_row = isize::try_from(other.row).unwrap() - isize::try_from(self.row).unwrap();
        Offset {
            row: diff_row,
            col: diff_col,
        }
    }

    pub fn direction_to(&self, other: &Point) -> Direction2D {
        let horizontal = match self.col.cmp(&other.col) {
            std::cmp::Ordering::Less => Some(Direction::Right),
            std::cmp::Ordering::Equal => None,
            std::cmp::Ordering::Greater => Some(Direction::Left),
        };
        let vertical = match self.row.cmp(&other.row) {
            std::cmp::Ordering::Less => Some(Direction::Down),
            std::cmp::Ordering::Equal => None,
            std::cmp::Ordering::Greater => Some(Direction::Up),
        };
        Direction2D {
            horizontal,
            vertical,
        }
    }
}
