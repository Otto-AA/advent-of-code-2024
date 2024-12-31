use std::env;

use parser::parse_file;
use simulation::Simulation;
use warehouse::{Field, Warehouse};

pub mod parser;
pub mod simulation;
pub mod warehouse;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = args.get(1).expect("Please provide an input file path");
    part_one(path);
    part_two(path);
}

fn part_one(path: &str) -> usize {
    let input = parse_file(path).unwrap();
    let mut warehouse = input.warehouse;

    let robot_start = warehouse
        .iter_fields()
        .find(|(_, f)| matches!(f, Field::Robot))
        .expect("Warehouse contains a robot field")
        .0;

    println!("{}", warehouse);
    println!("{:?}", input.moves);

    assert!(matches!(warehouse[&robot_start], Field::Robot));

    let mut simulation = Simulation {
        warehouse: &mut warehouse,
        robot_position: robot_start,
    };

    for direction in input.moves {
        println!("Move {}", direction);
        simulation.move_robot(&direction);
        println!("{}", simulation.warehouse);
        println!("GPS: {}", simulation.warehouse.gps());
    }
    simulation.warehouse.gps()
}

fn part_two(path: &str) -> usize {
    let input = parse_file(path).unwrap();
    let mut warehouse = Warehouse::scaled_up(&input.warehouse);

    let robot_start = warehouse
        .iter_fields()
        .find(|(_, f)| matches!(f, Field::Robot))
        .expect("Warehouse contains a robot field")
        .0;

    println!("{}", warehouse);
    println!("{:?}", input.moves);

    assert!(matches!(warehouse[&robot_start], Field::Robot));

    let mut simulation = Simulation {
        warehouse: &mut warehouse,
        robot_position: robot_start,
    };

    for direction in input.moves {
        println!("Move {}", direction);
        simulation.move_robot(&direction);
        println!("{}", simulation.warehouse);
        println!("GPS: {}", simulation.warehouse.gps());
    }
    simulation.warehouse.gps()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_small() {
        assert_eq!(2028, part_one("small.txt"));
    }

    #[test]
    fn part_one_sample() {
        assert_eq!(10092, part_one("sample.txt"));
    }

    #[test]
    fn part_two_sample() {
        assert_eq!(9021, part_two("sample.txt"));
    }
}
