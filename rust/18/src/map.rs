use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};

pub struct MemorySpace {
    map: Vec<Vec<MemoryState>>,
    width: usize,
    height: usize,
}

impl MemorySpace {
    pub fn new(width: usize, height: usize) -> Self {
        let mut map = Vec::new();
        for _ in 0..height {
            let mut row = Vec::new();
            for _ in 0..width {
                row.push(MemoryState::Normal);
            }
            map.push(row);
        }
        MemorySpace { map, width, height }
    }

    pub fn corrupt(&mut self, point: &Point) {
        self[point] = MemoryState::Corrupted;
    }

    pub fn in_range(&self, point: &Point) -> bool {
        point.row < self.height && point.col < self.width
    }

    pub fn neighbours(&self, point: &Point) -> Vec<Point> {
        point
            .neighbours()
            .into_iter()
            .filter(|p| self.in_range(p))
            .filter(|p| !matches!(self[p], MemoryState::Corrupted))
            .collect()
    }
}

impl Index<&Point> for MemorySpace {
    type Output = MemoryState;

    fn index(&self, index: &Point) -> &Self::Output {
        &self.map[index.row][index.col]
    }
}

impl IndexMut<&Point> for MemorySpace {
    fn index_mut(&mut self, index: &Point) -> &mut Self::Output {
        &mut self.map[index.row][index.col]
    }
}

impl Display for MemorySpace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.map {
            for state in row {
                write!(f, "{}", state)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum MemoryState {
    Normal,
    Corrupted,
}

impl Display for MemoryState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MemoryState::Normal => write!(f, "."),
            MemoryState::Corrupted => write!(f, "#"),
        }
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub struct Point {
    pub row: usize,
    pub col: usize,
}

impl Point {
    pub fn neighbours(&self) -> Vec<Point> {
        let offsets: Vec<(isize, isize)> = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];

        offsets
            .iter()
            .filter_map(|(offset_row, offset_col)| {
                Some(Point {
                    row: self.row.checked_add_signed(*offset_row)?,
                    col: self.col.checked_add_signed(*offset_col)?,
                })
            })
            .collect()
    }
}
