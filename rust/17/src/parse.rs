use std::fs::read_to_string;

use anyhow::{Context, Error, Result};
use regex::Regex;

use crate::{
    instructions::Instruction,
    program::Program,
    state::{RegisterState, State},
};

pub fn parse_input(path: &str) -> Result<(Program, State)> {
    let input = read_to_string(path)?;
    let mut instructions = Vec::new();

    let program = input
        .lines()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .next()
        .context("Could not find program in input file")?;
    let instr_regex = Regex::new(r"(\d),(\d)").unwrap();
    for capture in instr_regex.captures_iter(program) {
        let (_, [opcode, operand]) = capture.extract();
        let opcode = opcode.parse()?;
        let operand = operand.parse()?;
        let instruction = Instruction::parse(opcode, operand)?;
        instructions.push(instruction);
    }

    let [a, b, c] = input
        .lines()
        .take(3)
        .map(|line| {
            let n: u64 = line
                .split(": ")
                .nth(1)
                .context("Could not parse register")?
                .parse()?;
            Ok::<u64, Error>(n)
        })
        .collect::<Result<Vec<u64>>>()?
        .try_into()
        .expect("Could not parse registers");
    let state = State {
        pc: 0,
        out: Vec::new(),
        registers: RegisterState { a, b, c },
    };

    Ok((Program::new(instructions), state))
}
