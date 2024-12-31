use anyhow::{Context, Result};
use keypad::{
    directional_keypad, numerical_keypad,
    DirectionalButton::{self, Activate, Arrow},
    DirectionalKeypad, Keypad, NumericButton, NumericKeypad,
};
use std::{collections::HashMap, env, fs::read_to_string};

mod keypad;
mod utils;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let path = args.get(1).context("Please provide a path argument")?;
    let codes = parse_input(path)?;

    println!("Part one: {}", complexity_sum(&codes, 3, false));
    println!("Part two: {}", complexity_sum(&codes, 26, true));

    Ok(())
}

fn parse_input(path: &str) -> Result<Vec<String>> {
    Ok(read_to_string(path)?
        .lines()
        .map(|l| l.to_owned())
        .collect())
}

fn code_number(code: &str) -> Result<usize> {
    let digits: String = code.chars().take_while(|c| c.is_ascii_digit()).collect();
    Ok(digits.parse()?)
}

fn complexity_sum(codes: &[String], directional_keypads_count: usize, efficient: bool) -> usize {
    codes
        .iter()
        .map(|code| {
            let complexity = code_complexity(code, directional_keypads_count, efficient);
            let n = code_number(code).expect("Can parse code");
            complexity * n
        })
        .sum()
}

fn code_complexity(code: &str, directional_keypads_count: usize, efficient: bool) -> usize {
    if efficient {
        return dynamic_programming_solution(code, directional_keypads_count);
    }
    let mut numeric_keypad = numerical_keypad();
    let mut directional_keypads = vec![];
    for _ in 0..directional_keypads_count {
        directional_keypads.push(directional_keypad());
    }
    let mut directional_keypads: Vec<&mut Keypad<DirectionalButton>> =
        directional_keypads.iter_mut().collect();
    let targets = NumericButton::parse(code).expect("Could not parse code to numeric buttons");

    let steps = find_steps(&mut numeric_keypad, &mut directional_keypads, &targets);
    steps.len()
}

fn find_steps(
    numeric_keypad: &mut NumericKeypad,
    directional_keypads: &mut [&mut DirectionalKeypad],
    targets: &Vec<NumericButton>,
) -> Vec<DirectionalButton> {
    let mut steps = vec![];
    for target in targets {
        let mut results = vec![];
        for next_keypad_steps in numeric_keypad.paths_to(target) {
            let mut next_keypad_steps: Vec<DirectionalButton> = next_keypad_steps
                .into_iter()
                .map(DirectionalButton::Arrow)
                .collect();
            next_keypad_steps.push(DirectionalButton::Activate);
            let final_keyboard_steps =
                find_steps_recursive(directional_keypads, &next_keypad_steps);
            results.push(final_keyboard_steps);
        }
        results.sort_by_key(|a| a.len());
        let best_result = results
            .first()
            .expect("Find at least one result")
            .to_owned();
        steps.extend(best_result);
        numeric_keypad.move_to(target);
    }

    steps
}

fn find_steps_recursive(
    keypads: &mut [&mut DirectionalKeypad],
    targets: &Vec<DirectionalButton>,
) -> Vec<DirectionalButton> {
    // base case
    if keypads.len() == 1 {
        keypads[0].move_to(targets.last().unwrap());
        return targets.clone();
    }

    // recursively add neccessary steps on the next layer
    let mut steps = vec![];
    for target in targets {
        let mut results = vec![];
        for next_keypad_steps in keypads[0].paths_to(target) {
            let mut next_keypad_steps: Vec<DirectionalButton> = next_keypad_steps
                .into_iter()
                .map(DirectionalButton::Arrow)
                .collect();
            next_keypad_steps.push(DirectionalButton::Activate);
            let final_keyboard_steps = find_steps_recursive(&mut keypads[1..], &next_keypad_steps);
            results.push(final_keyboard_steps);
        }
        results.sort_by_key(|a| a.len());
        let best_result = results
            .first()
            .expect("Find at least one result")
            .to_owned();
        steps.extend(best_result);
        keypads[0].move_to(target);
    }
    // update current location to last performed step
    steps
}

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
struct SolutionKey {
    pub from: DirectionalButton,
    pub to: DirectionalButton,
}
// Costs for moving and activating
type Solutions = HashMap<SolutionKey, usize>;

fn dynamic_programming_solution(code: &str, directional_keypads_count: usize) -> usize {
    let solutions = precompute_solutions(directional_keypads_count);
    find_numeric_keypad_result(code, &solutions)
}

fn precompute_solutions(directional_keypads_count: usize) -> Solutions {
    let mut solutions: Option<Solutions> = None;
    let mut keypad = directional_keypad();

    for _ in 0..directional_keypads_count {
        let mut next_solutions: Solutions = HashMap::new();
        for from in DirectionalButton::all_buttons() {
            for to in DirectionalButton::all_buttons() {
                let solution_key = SolutionKey { from, to };
                keypad.move_to(&from);
                let mut min_costs = None;
                for path in keypad.paths_to(&to) {
                    // calculate costs
                    match solutions {
                        None => {
                            // user keypad only needs to press buttons without moving costs
                            min_costs = Some(1);
                        }
                        Some(ref previous_layer) => {
                            let mut total_costs = 0;
                            let mut current = Activate;
                            let mut steps: Vec<DirectionalButton> =
                                path.into_iter().map(Arrow).collect();
                            steps.push(Activate);
                            for target in steps {
                                total_costs += previous_layer[&SolutionKey {
                                    from: current,
                                    to: target,
                                }];
                                current = target;
                            }
                            min_costs = Some(total_costs.min(min_costs.unwrap_or(total_costs)));
                        }
                    };
                }
                next_solutions.insert(solution_key.clone(), min_costs.unwrap());
            }
        }
        solutions = Some(next_solutions);
    }

    solutions.unwrap()
}

fn find_numeric_keypad_result(code: &str, solutions: &Solutions) -> usize {
    let targets = NumericButton::parse(code).unwrap();
    let mut numeric_keypad = numerical_keypad();

    let mut total_sum = 0;

    for target in targets {
        let mut path_results = vec![];
        for path in numeric_keypad.paths_to(&target) {
            let mut sum = 0;
            let mut previous_position = DirectionalButton::Activate;
            let mut steps: Vec<DirectionalButton> = path.into_iter().map(Arrow).collect();
            steps.push(Activate);
            for target in steps.clone() {
                let solution_key = SolutionKey {
                    from: previous_position,
                    to: target,
                };
                let cost = solutions[&solution_key];
                previous_position = target;
                sum += cost;
            }
            path_results.push(sum);
        }
        let moving_costs = path_results
            .iter()
            .min()
            .expect("Finds a minimum number of presses");
        numeric_keypad.move_to(&target);
        total_sum += *moving_costs;
    }

    total_sum
}

#[cfg(test)]
mod tests {
    use keypad::{directional_keypad, numerical_keypad, NumericButton};
    use proptest::prelude::*;
    use utils::Direction;

    use super::*;

    #[test]
    pub fn find_steps_single_directional_keyboard() -> Result<()> {
        let mut keypad = directional_keypad();
        let targets = DirectionalButton::parse("<A")?;

        let steps = find_steps_recursive(&mut [&mut keypad], &targets);

        assert_eq!(targets, steps);
        assert_eq!(keypad.current(), targets.last().unwrap());
        Ok(())
    }

    #[test]
    pub fn find_steps_numerical_and_directional_keypad() -> Result<()> {
        let mut numerical_keypad = numerical_keypad();
        let mut directional_keypad = directional_keypad();
        let targets =
            NumericButton::parse("029A").expect("Could not parse code to numeric buttons");

        let steps = find_steps(
            &mut numerical_keypad,
            &mut [&mut directional_keypad],
            &targets,
        );

        let expected_steps = DirectionalButton::parse("<A^A>^^AvvvA")?;
        assert_eq!(expected_steps.len(), steps.len());
        assert_eq!(numerical_keypad.current(), targets.last().unwrap());
        Ok(())
    }

    #[test]
    pub fn find_steps_two_directional_keypads() -> Result<()> {
        let mut keypad_one = directional_keypad();
        let mut keypad_two = directional_keypad();
        let mut directional_keypads = vec![&mut keypad_one, &mut keypad_two];
        let targets = DirectionalButton::parse("<A^A>^^AvvvA")
            .expect("Could not parse code to directional buttons");

        let steps = find_steps_recursive(&mut directional_keypads, &targets);

        let expected_steps = DirectionalButton::parse("v<<A>>^A<A>AvA<^AA>A<vAAA>^A")?;
        assert_eq!(expected_steps.len(), steps.len());
        assert_eq!(keypad_one.current(), targets.last().unwrap());
        Ok(())
    }

    #[test]
    pub fn find_steps_numeric_and_three_directional_keypads() -> Result<()> {
        let mut numeric_keypad = numerical_keypad();
        let mut keypad_one = directional_keypad();
        let mut keypad_two = directional_keypad();
        let mut keypad_three = directional_keypad();
        let mut directional_keypads = vec![&mut keypad_one, &mut keypad_two, &mut keypad_three];
        let targets =
            NumericButton::parse("029A").expect("Could not parse code to numeric buttons");

        let steps = find_steps(&mut numeric_keypad, &mut directional_keypads, &targets);

        let expected_steps = DirectionalButton::parse(
            "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A",
        )?;
        assert_eq!(expected_steps.len(), steps.len());
        assert_eq!(numeric_keypad.current(), targets.last().unwrap());
        Ok(())
    }

    #[test]
    pub fn sample_cases() -> Result<()> {
        let samples = vec![
            (
                "029A",
                "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A",
            ),
            (
                "980A",
                "<v<A>>^AAAvA^A<vA<AA>>^AvAA<^A>A<v<A>A>^AAAvA<^A>A<vA>^A<A>A",
            ),
            (
                "179A",
                "<v<A>>^A<vA<A>>^AAvAA<^A>A<v<A>>^AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A",
            ),
            (
                "456A",
                "<v<A>>^AA<vA<A>>^AAvAA<^A>A<vA>^A<A>A<vA>^A<A>A<v<A>A>^AAvA<^A>A",
            ),
            (
                "379A",
                "<v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A",
            ),
        ];
        for (code, expected_buttons) in samples {
            assert_eq!(expected_buttons.len(), code_complexity(code, 3, false));
        }
        Ok(())
    }

    #[test]
    pub fn part_one_input() -> Result<()> {
        let codes = parse_input("input.txt")?;

        let result = complexity_sum(&codes, 3, false);

        assert_eq!(215374, result);
        Ok(())
    }

    #[test]
    pub fn precompute_solutions_one_layer() -> Result<()> {
        let solutions = precompute_solutions(1);

        // pressing a button on the user keypad costs 1
        assert_eq!(
            1,
            solutions[&SolutionKey {
                from: Activate,
                to: Activate
            }]
        );
        assert_eq!(
            1,
            solutions[&SolutionKey {
                from: Activate,
                to: Arrow(Direction::Left)
            }]
        );
        assert_eq!(
            1,
            solutions[&SolutionKey {
                from: Arrow(Direction::Left),
                to: Arrow(Direction::Right)
            }]
        );

        Ok(())
    }

    #[test]
    pub fn precompute_solutions_two_layers() -> Result<()> {
        let solutions = precompute_solutions(2);

        // moving a button indirectly via {n} button presses on the user keypad
        // and then submitting it
        assert_eq!(
            1,
            solutions[&SolutionKey {
                from: Activate,
                to: Activate
            }]
        );
        assert_eq!(
            4,
            solutions[&SolutionKey {
                from: Activate,
                to: Arrow(Direction::Left)
            }]
        );
        assert_eq!(
            3,
            solutions[&SolutionKey {
                from: Arrow(Direction::Left),
                to: Arrow(Direction::Right)
            }]
        );

        Ok(())
    }

    #[test]
    pub fn precompute_solutions_three_layers() -> Result<()> {
        let solutions = precompute_solutions(3);

        // moving a button indirectly via {n} button presses on the user keypad
        // and then submitting it
        assert_eq!(
            1,
            solutions[&SolutionKey {
                from: Activate,
                to: Activate
            }]
        );
        assert_eq!(
            10,
            solutions[&SolutionKey {
                from: Activate,
                to: Arrow(Direction::Left)
            }]
        );
        assert_eq!(
            5,
            solutions[&SolutionKey {
                from: Arrow(Direction::Left),
                to: Arrow(Direction::Right)
            }]
        );

        Ok(())
    }

    #[test]
    pub fn efficient_simple_button_press() {
        let code = "A";

        let presses = code_complexity(code, 3, true);

        assert_eq!(1, presses);
    }

    #[test]
    pub fn efficient_single_move() {
        let code = "0";

        let presses = code_complexity(code, 3, true);

        assert_eq!(18, presses);
    }

    #[test]
    pub fn efficient_multiple_moves() {
        let code = "0A";

        let presses = code_complexity(code, 3, true);
        let expected = code_complexity(code, 3, false);

        assert_eq!(expected, presses);
    }

    #[test]
    pub fn efficient_many_keypads() {
        let code = "0";

        let presses = code_complexity(code, 5, true);
        let expected = code_complexity(code, 5, false);

        assert_eq!(expected, presses);
    }

    proptest! {
        #[test]
        fn efficient_method_yields_same_result(n: u8, a: bool, keypads in 1usize..5) {
            let a = match a {
                true => "A",
                false => "",
            };
            let code = format!("{n}{a}");

            let baseline = code_complexity(&code, keypads, false);
            let efficient = code_complexity(&code, keypads, true);

            assert_eq!(baseline, efficient);
        }
    }
}
