use anyhow::{Context, Result};

use crate::race_map::{Field, Point, RaceMap};

pub fn parse_input(input: &str) -> Result<(RaceMap, Point, Point)> {
    let mut start: Option<Point> = None;
    let mut end: Option<Point> = None;
    let mut map: Vec<Vec<Field>> = Vec::new();

    for (row, line) in input.lines().enumerate() {
        let mut fields = Vec::new();
        for (col, c) in line.chars().enumerate() {
            let field = match c {
                '#' => Some(Field::Wall),
                '.' => Some(Field::Track),
                'S' => {
                    start = Some(Point { row, col });
                    Some(Field::Track)
                }
                'E' => {
                    end = Some(Point { row, col });
                    Some(Field::Track)
                }
                _ => None,
            };
            fields.push(field.context("Could not parse field")?);
        }
        map.push(fields);
    }

    Ok((
        RaceMap::new(map),
        start.context("Could not find start point")?,
        end.context("Could not find end point")?,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_sample_map() -> Result<()> {
        let sample_input = "\
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

        let (_, start, end) = parse_input(sample_input)?;

        assert_eq!(Point { row: 3, col: 1 }, start);
        assert_eq!(Point { row: 7, col: 5 }, end);

        Ok(())
    }
}
