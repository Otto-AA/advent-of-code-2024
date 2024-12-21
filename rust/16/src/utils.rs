#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct Point {
    pub row: usize,
    pub col: usize,
}

impl Point {
    pub fn neighbours(&self) -> Vec<Point> {
        let mut neighbours = vec![
            Point {
                row: self.row + 1,
                col: self.col,
            },
            Point {
                row: self.row,
                col: self.col + 1,
            },
        ];
        if self.row > 0 {
            neighbours.push(Point {
                row: self.row - 1,
                col: self.col,
            });
        }
        if self.col > 0 {
            neighbours.push(Point {
                row: self.row,
                col: self.col - 1,
            });
        }
        neighbours
    }

    pub fn direction_to(&self, to: &Point) -> Option<Direction> {
        if self.row < to.row && self.col == to.col {
            return Some(Direction::Down);
        }
        if self.row > to.row && self.col == to.col {
            return Some(Direction::Up);
        }
        if self.row == to.row && self.col < to.col {
            return Some(Direction::Right);
        }
        if self.row == to.row && self.col > to.col {
            return Some(Direction::Left);
        }

        None
    }
}

pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}
