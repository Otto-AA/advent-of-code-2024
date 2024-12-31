use anyhow::{Context, Result};

use crate::{
    instructions::{Combo, Instruction, Literal, Register},
    program::Program,
    state::State,
};

pub fn step(program: &Program, state: &mut State) -> Result<()> {
    let pc = state.pc;
    let instruction = program
        .get_instruction(pc)
        .context(format!("Invalid pc {}", pc))?;

    execute_instruction(state, instruction);

    Ok(())
}

fn execute_instruction(state: &mut State, instruction: &Instruction) {
    let mut jumped = false;
    match instruction {
        Instruction::Adv(operand) => {
            state.registers.a /= 2_u64.pow(parse_operand(operand, state).try_into().unwrap());
        }
        Instruction::Bxl(Literal(n)) => {
            state.registers.b ^= u64::from(*n);
        }
        Instruction::Bst(operand) => {
            state.registers.b = parse_operand(operand, state) % 8;
        }
        Instruction::Jnz(Literal(n)) => {
            if state.registers.a != 0 {
                state.pc = usize::from(*n);
                jumped = true;
            }
        }
        Instruction::Bxc(_) => {
            state.registers.b ^= state.registers.c;
        }
        Instruction::Out(operand) => {
            state
                .out
                .push((parse_operand(operand, state) % 8).try_into().unwrap());
        }
        Instruction::Bdv(operand) => {
            state.registers.b =
                state.registers.a / 2_u64.pow(parse_operand(operand, state).try_into().unwrap());
        }
        Instruction::Cdv(operand) => {
            state.registers.c =
                state.registers.a / 2_u64.pow(parse_operand(operand, state).try_into().unwrap());
        }
    }

    if !jumped {
        state.pc += 2;
    }
}

fn parse_operand(operand: &Combo, state: &State) -> u64 {
    match operand {
        Combo::Literal(n) => u64::from(*n),
        Combo::Register(Register::A) => state.registers.a,
        Combo::Register(Register::B) => state.registers.b,
        Combo::Register(Register::C) => state.registers.c,
    }
}
