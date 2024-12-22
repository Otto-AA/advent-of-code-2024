use anyhow::{Error, Ok, Result};
use instructions::Instruction;
use interpreter::step;
use parse::parse_input;
use program::Program;
use state::{RegisterState, State};
use std::{
    collections::{HashMap, HashSet},
    env,
};

mod instructions;
mod interpreter;
mod parse;
mod program;
mod state;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let part: u8 = args
        .get(1)
        .expect("Please specify which part to execute (1 or 2 or 3)")
        .parse()?;
    let file_path = args.get(2).expect("Please provide a file path");

    if part == 1 {
        let result = part_one(file_path)?;
        let result: Vec<String> = result.iter().map(|n| n.to_string()).collect();
        println!("Out: {}", result.join(","));
    } else if part == 2 {
        let a = part_two(file_path)?;
        println!("Register A: {}", a);
    } else {
        let a = args.get(3).expect("Please provide a register a").parse()?;
        let (program, _) = parse_input(file_path)?;
        let out = run_with_register_a(&program, a)?;
        println!("Result for {:b}: {:?}", a, out);
    }

    Ok(())
}

fn part_one(path: &str) -> Result<Vec<u8>> {
    let (program, mut state) = parse_input(path)?;

    while program.valid_pc(state.pc) {
        step(&program, &mut state)?;
    }

    Ok(state.out)
}

fn part_two(path: &str) -> Result<u64> {
    /*
    This particular program essentially has following loop:
    do:
        ...
        out ...
        a = a >> 3;
    while (a > 0)

    Thus, we can find a particular input by starting from the minimum.
    For instance, we have following property:
    - a=0b101 yields [7]
    - a=0b101000 yields [2, 7]
    - a=0b101000000 yields [4, 2, 7]
    ...
    So shifting a value 3 bits to the left does not modify the last entries
     */
    let (program, _) = parse_input(path)?;
    println!("Program: {}", program);
    let expected_out: Vec<u8> = program
        .iter_instructions()
        .map(|instr| <&Instruction as Into<(u8, u8)>>::into(instr))
        .fold(Vec::new(), |mut vec, (opcode, operand)| {
            vec.push(opcode);
            vec.push(operand);
            vec
        });
    let out_length: u32 = expected_out.len().try_into()?;

    // maps n to a set of numbers that produce the correct result
    // when considering the last n elements
    let mut possible_prefixes: HashMap<u32, HashSet<u64>> = HashMap::new();
    possible_prefixes.insert(0, HashSet::new());
    possible_prefixes.entry(0).and_modify(|s| {
        s.insert(0);
    });

    let mut solutions = Vec::new();

    for n in 0..out_length {
        let prefixes = possible_prefixes.get(&n).unwrap();
        let mut new_prefixes = HashSet::new();
        let index: usize = (out_length - n - 1).try_into()?;
        let expected_value = expected_out.get(index).unwrap();

        for prefix in prefixes {
            for i in 0..8 {
                let test_a = prefix + i;
                let out = run_with_register_a(&program, test_a)?;

                if expected_out == out {
                    solutions.push(test_a);
                }
                if expected_value == out.first().unwrap() {
                    new_prefixes.insert(test_a << 3);
                }
            }
        }
        possible_prefixes.insert(n + 1, new_prefixes);
    }

    if solutions.is_empty() {
        Err(Error::msg("Could not find a solution"))
    } else {
        Ok(*solutions.iter().min().unwrap())
    }
}

fn run_with_register_a(program: &Program, a: u64) -> Result<Vec<u8>> {
    let mut state = State {
        registers: RegisterState {
            a,
            ..Default::default()
        },
        ..Default::default()
    };

    while program.valid_pc(state.pc) {
        step(program, &mut state)?;
    }

    Ok(state.out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() -> Result<()> {
        let result = part_one("sample.txt")?;

        assert_eq!(vec![4, 6, 3, 5, 6, 3, 5, 2, 1, 0], result);

        Ok(())
    }

    #[test]
    fn input() -> Result<()> {
        let result = part_one("input.txt")?;

        assert_eq!(vec![7, 0, 7, 3, 4, 1, 3, 0, 1], result);

        Ok(())
    }
}
