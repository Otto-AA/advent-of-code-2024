use std::{collections::HashMap, hash::Hash};

use anyhow::{Error, Result};

use crate::utils::{Direction, Point};

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub enum DirectionalButton {
    Arrow(Direction),
    Activate,
}

impl DirectionalButton {
    #[cfg(test)]
    pub fn parse(buttons: &str) -> Result<Vec<DirectionalButton>> {
        buttons
            .chars()
            .map(|c| match c {
                '^' => Ok(DirectionalButton::Arrow(Direction::Up)),
                '>' => Ok(DirectionalButton::Arrow(Direction::Right)),
                'v' => Ok(DirectionalButton::Arrow(Direction::Down)),
                '<' => Ok(DirectionalButton::Arrow(Direction::Left)),
                'A' => Ok(DirectionalButton::Activate),
                _ => Err(Error::msg("Invalid char for directional button")),
            })
            .collect()
    }

    pub fn all_buttons() -> impl Iterator<Item = DirectionalButton> {
        vec![
            DirectionalButton::Arrow(Direction::Up),
            DirectionalButton::Arrow(Direction::Right),
            DirectionalButton::Arrow(Direction::Down),
            DirectionalButton::Arrow(Direction::Left),
            DirectionalButton::Activate,
        ]
        .into_iter()
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct KeypadNumber(pub u8);

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum NumericButton {
    Number(KeypadNumber),
    Activate,
}

impl NumericButton {
    pub fn parse(buttons: &str) -> Result<Vec<NumericButton>> {
        buttons
            .chars()
            .map(|c| match c {
                '0'..='9' => Ok(NumericButton::Number(KeypadNumber(
                    u8::try_from(c.to_digit(10).unwrap()).unwrap(),
                ))),
                'A' => Ok(NumericButton::Activate),
                _ => Err(Error::msg(format!("Invalid char for numeric button: {c}"))),
            })
            .collect()
    }
}

pub type DirectionalKeypad = Keypad<DirectionalButton>;
pub type NumericKeypad = Keypad<NumericButton>;
pub struct Keypad<T: Hash + Eq + Clone> {
    point_to_button: HashMap<Point, T>,
    button_to_point: HashMap<T, Point>,
    current: Point,
}

impl<T: Hash + Eq + Clone> Keypad<T> {
    pub fn new(buttons: HashMap<Point, T>, start: Point) -> Self {
        let button_to_point: HashMap<T, Point> = buttons
            .iter()
            .map(|(p, b)| (b.clone(), p.clone()))
            .collect();

        Keypad {
            point_to_button: buttons,
            button_to_point,
            current: start,
        }
    }

    pub fn paths_to(&self, goal: &T) -> Vec<Vec<Direction>> {
        let goal = self.button_to_point.get(goal).unwrap();
        let direction = self.current.direction_to(goal);
        let offsets = self.current.distance(goal);
        let directions_vertical = vec![direction.vertical; offsets.row.unsigned_abs()];
        let directions_horizontal = vec![direction.horizontal; offsets.col.unsigned_abs()];

        let paths = vec![
            (directions_horizontal.clone(), directions_vertical.clone()),
            (directions_vertical, directions_horizontal),
        ];

        let possible_paths: Vec<Vec<Direction>> = paths
            .into_iter()
            .map(|(a, b)| a.into_iter().chain(b).flatten().collect())
            .filter(|path| self.is_possible_path(path))
            .collect();

        possible_paths
    }

    fn is_possible_path(&self, path: &Vec<Direction>) -> bool {
        let mut current = self.current.clone();
        for step in path {
            current.add(&step.to_offset()).unwrap();
            if !self.valid_point(&current) {
                return false;
            }
        }
        true
    }

    pub fn move_to(&mut self, target: &T) {
        self.current = self.button_to_point.get(target).unwrap().clone()
    }

    fn valid_point(&self, point: &Point) -> bool {
        self.point_to_button.contains_key(point)
    }

    #[cfg(test)]
    pub fn current(&self) -> &T {
        self.point_to_button.get(&self.current).unwrap()
    }
}

pub fn numerical_keypad() -> Keypad<NumericButton> {
    let buttons: HashMap<Point, NumericButton> = vec![
        (
            Point { row: 0, col: 0 },
            NumericButton::Number(KeypadNumber(7)),
        ),
        (
            Point { row: 0, col: 1 },
            NumericButton::Number(KeypadNumber(8)),
        ),
        (
            Point { row: 0, col: 2 },
            NumericButton::Number(KeypadNumber(9)),
        ),
        (
            Point { row: 1, col: 0 },
            NumericButton::Number(KeypadNumber(4)),
        ),
        (
            Point { row: 1, col: 1 },
            NumericButton::Number(KeypadNumber(5)),
        ),
        (
            Point { row: 1, col: 2 },
            NumericButton::Number(KeypadNumber(6)),
        ),
        (
            Point { row: 2, col: 0 },
            NumericButton::Number(KeypadNumber(1)),
        ),
        (
            Point { row: 2, col: 1 },
            NumericButton::Number(KeypadNumber(2)),
        ),
        (
            Point { row: 2, col: 2 },
            NumericButton::Number(KeypadNumber(3)),
        ),
        (
            Point { row: 3, col: 1 },
            NumericButton::Number(KeypadNumber(0)),
        ),
        (Point { row: 3, col: 2 }, NumericButton::Activate),
    ]
    .into_iter()
    .collect();
    let start = Point { row: 3, col: 2 };

    Keypad::new(buttons, start)
}
pub fn directional_keypad() -> Keypad<DirectionalButton> {
    let buttons: HashMap<Point, DirectionalButton> = vec![
        (
            Point { row: 0, col: 1 },
            DirectionalButton::Arrow(Direction::Up),
        ),
        (Point { row: 0, col: 2 }, DirectionalButton::Activate),
        (
            Point { row: 1, col: 0 },
            DirectionalButton::Arrow(Direction::Left),
        ),
        (
            Point { row: 1, col: 1 },
            DirectionalButton::Arrow(Direction::Down),
        ),
        (
            Point { row: 1, col: 2 },
            DirectionalButton::Arrow(Direction::Right),
        ),
    ]
    .into_iter()
    .collect();
    let start = Point { row: 0, col: 2 };

    Keypad::new(buttons, start)
}
