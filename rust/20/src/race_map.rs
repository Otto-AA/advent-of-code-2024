use std::{
    collections::HashSet,
    ops::{Index, IndexMut},
};

pub struct RaceMap {
    map: Vec<Vec<Field>>,
    width: usize,
    height: usize,
}

impl RaceMap {
    pub fn new(map: Vec<Vec<Field>>) -> Self {
        let height = map.len();
        let width = map.first().expect("Race map has at least one row").len();

        RaceMap { map, width, height }
    }

    pub fn neighbour_tracks(&self, point: &Point) -> Vec<Point> {
        point
            .neighbours()
            .into_iter()
            .filter(|p| self.in_range(p))
            .filter(|p| matches!(self[p], Field::Track))
            .collect()
    }

    pub fn in_range(&self, point: &Point) -> bool {
        point.row < self.height && point.col < self.width
    }
}

impl Index<&Point> for RaceMap {
    type Output = Field;

    fn index(&self, index: &Point) -> &Self::Output {
        &self.map[index.row][index.col]
    }
}

impl IndexMut<&Point> for RaceMap {
    fn index_mut(&mut self, index: &Point) -> &mut Self::Output {
        &mut self.map[index.row][index.col]
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Point {
    pub row: usize,
    pub col: usize,
}

impl Point {
    pub fn neighbours(&self) -> Vec<Point> {
        [(1, 0), (0, 1), (-1, 0), (0, -1)]
            .iter()
            .filter_map(|(offset_row, offset_col)| {
                Some(Point {
                    row: self.row.checked_add_signed(*offset_row)?,
                    col: self.col.checked_add_signed(*offset_col)?,
                })
            })
            .collect()
    }

    pub fn step_distance(&self, other: &Point) -> usize {
        self.row.abs_diff(other.row) + self.col.abs_diff(other.col)
    }

    pub fn points_within_step_distance(&self, distance: usize) -> HashSet<Point> {
        let mut points: HashSet<Point> = HashSet::new();
        // we use this as a starting point (and later remove it)
        points.insert(self.clone());
        for _ in 0..distance {
            let mut neighbours = HashSet::new();
            for p in &points {
                neighbours.extend(p.neighbours());
            }
            // remove non-walls
            neighbours = neighbours
                .into_iter()
                // .filter(|w| !track.contains_key(w))
                .collect();
            points.extend(neighbours);
        }
        points
    }
}

pub enum Field {
    Track,
    Wall,
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use super::*;

    #[test]
    fn map_neighbour_tracks() -> Result<()> {
        let map = RaceMap::new(vec![
            vec![Field::Wall, Field::Track, Field::Track],
            vec![Field::Track, Field::Track, Field::Track],
        ]);

        let neighbours = map.neighbour_tracks(&Point { row: 0, col: 1 });

        assert!(neighbours.contains(&Point { row: 0, col: 2 }));
        assert!(neighbours.contains(&Point { row: 1, col: 1 }));
        assert_eq!(2, neighbours.len());
        Ok(())
    }
}
