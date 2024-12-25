use std::collections::HashMap;

use crate::race_map::Point;

pub fn cheatcodes(track: &HashMap<&Point, usize>, from: &Point, length: usize) -> Vec<Cheat> {
    from.points_within_step_distance(length)
        .into_iter()
        .filter(|p| p != from)
        .filter(|p| track.contains_key(p))
        .filter(|p| track.get(p).unwrap() > track.get(from).unwrap())
        .map(|p| Cheat {
            from: from.clone(),
            to: p,
        })
        .collect()
}

pub fn steps_saved(point_to_ns: &HashMap<&Point, usize>, cheat: &Cheat) -> Option<usize> {
    let find_pos = |x| {
        *point_to_ns
            .get(x)
            .expect("Can only calculate saved steps for cheats belonging to a track")
    };
    let from = find_pos(&cheat.from);
    let to = find_pos(&cheat.to);
    let cheating_steps = cheat.from.step_distance(&cheat.to);

    to.checked_sub(from)
        .and_then(|n| n.checked_sub(cheating_steps))
}

#[derive(PartialEq, Debug)]
pub struct Cheat {
    from: Point,
    to: Point,
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use super::*;

    fn simple_track() -> HashMap<Point, usize> {
        // X X
        //   X X
        //     X
        // X X X
        vec![
            Point { row: 0, col: 0 },
            Point { row: 0, col: 1 },
            Point { row: 1, col: 1 },
            Point { row: 1, col: 2 },
            Point { row: 2, col: 2 },
            Point { row: 3, col: 2 },
            Point { row: 3, col: 1 },
            Point { row: 3, col: 0 },
        ]
        .into_iter()
        .enumerate()
        .map(|(n, p)| (p, n))
        .collect()
    }

    #[test]
    fn calculates_steps_saved_short() -> Result<()> {
        let track = simple_track();
        let track = track.iter().map(|(p, u)| (p, *u)).collect();
        let cheat = Cheat {
            from: Point { row: 1, col: 1 },
            to: Point { row: 3, col: 1 },
        };

        let saved = steps_saved(&track, &cheat);

        assert_eq!(Some(2), saved);
        Ok(())
    }

    #[test]
    fn calculates_steps_saved_long() -> Result<()> {
        let track = simple_track();
        let track = track.iter().map(|(p, u)| (p, *u)).collect();
        let cheat = Cheat {
            from: Point { row: 1, col: 1 },
            to: Point { row: 3, col: 0 },
        };

        let saved = steps_saved(&track, &cheat);

        assert_eq!(Some(2), saved);
        Ok(())
    }

    #[test]
    fn cheatcodes_of_size_two() -> Result<()> {
        let track = simple_track();
        let track = track.iter().map(|(p, u)| (p, *u)).collect();
        let from = Point { row: 1, col: 1 };

        let cheats = cheatcodes(&track, &from, 2);

        assert!(cheats.contains(&Cheat {
            from: from.clone(),
            to: Point { row: 3, col: 1 }
        }));
        assert!(cheats.contains(&Cheat {
            from: from.clone(),
            to: Point { row: 1, col: 2 }
        }));
        assert!(cheats.contains(&Cheat {
            from: from.clone(),
            to: Point { row: 2, col: 2 }
        }));
        println!("{cheats:?}");
        assert_eq!(3, cheats.len());

        Ok(())
    }

    #[test]
    fn cheatcodes_of_larger_size() -> Result<()> {
        let track = simple_track();
        let track = track.iter().map(|(p, u)| (p, *u)).collect();
        let from = Point { row: 0, col: 1 };

        let cheats = cheatcodes(&track, &from, 3);

        let expected_points = vec![
            Point { row: 1, col: 1 },
            Point { row: 1, col: 2 },
            Point { row: 2, col: 2 },
            Point { row: 3, col: 1 },
        ];
        for p in &expected_points {
            assert!(cheats.contains(&Cheat {
                from: from.clone(),
                to: p.clone(),
            }));
        }
        assert_eq!(expected_points.len(), cheats.len());

        Ok(())
    }
}
