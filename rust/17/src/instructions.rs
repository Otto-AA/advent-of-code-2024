use std::fmt::Display;

use anyhow::{anyhow, Error, Result};

pub enum Instruction {
    Adv(Combo),
    Bxl(Literal),
    Bst(Combo),
    Jnz(Literal),
    Bxc(Literal),
    Out(Combo),
    Bdv(Combo),
    Cdv(Combo),
}

impl Instruction {
    pub fn parse(opcode: u8, operand: u8) -> Result<Instruction, Error> {
        match opcode {
            0 => Ok(Instruction::Adv(Combo::parse(operand)?)),
            1 => Ok(Instruction::Bxl(Literal::parse(operand)?)),
            2 => Ok(Instruction::Bst(Combo::parse(operand)?)),
            3 => Ok(Instruction::Jnz(Literal::parse(operand)?)),
            4 => Ok(Instruction::Bxc(Literal::parse(operand)?)),
            5 => Ok(Instruction::Out(Combo::parse(operand)?)),
            6 => Ok(Instruction::Bdv(Combo::parse(operand)?)),
            7 => Ok(Instruction::Cdv(Combo::parse(operand)?)),
            _ => Err(anyhow!("Invalid opcode {}", opcode)),
        }
    }
}

impl From<&Instruction> for (u8, u8) {
    fn from(val: &Instruction) -> Self {
        match val {
            Instruction::Adv(combo) => (0, combo.into()),
            Instruction::Bxl(literal) => (1, literal.into()),
            Instruction::Bst(combo) => (2, combo.into()),
            Instruction::Jnz(literal) => (3, literal.into()),
            Instruction::Bxc(literal) => (4, literal.into()),
            Instruction::Out(combo) => (5, combo.into()),
            Instruction::Bdv(combo) => (6, combo.into()),
            Instruction::Cdv(combo) => (7, combo.into()),
        }
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Adv(combo) => write!(f, "adv({})", combo),
            Instruction::Bxl(Literal(n)) => write!(f, "bxl({})", n),
            Instruction::Bst(combo) => write!(f, "bst({})", combo),
            Instruction::Jnz(Literal(n)) => write!(f, "jnz({})", n),
            Instruction::Bxc(Literal(_)) => write!(f, "bxc()"),
            Instruction::Out(combo) => write!(f, "out({})", combo),
            Instruction::Bdv(combo) => write!(f, "bdv({})", combo),
            Instruction::Cdv(combo) => write!(f, "cdv({})", combo),
        }
    }
}

pub struct Literal(pub u8);

impl Literal {
    pub fn parse(operand: u8) -> Result<Self, Error> {
        if operand >= 8 {
            Err(anyhow!("Invalid literal {}", operand))
        } else {
            Ok(Self(operand))
        }
    }
}

impl From<&Literal> for u8 {
    fn from(val: &Literal) -> Self {
        val.0
    }
}

pub enum Combo {
    Literal(u8),
    Register(Register),
}

impl Combo {
    pub fn parse(operand: u8) -> Result<Self, Error> {
        match operand {
            0..=3 => Ok(Self::Literal(operand)),
            4 => Ok(Self::Register(Register::A)),
            5 => Ok(Self::Register(Register::B)),
            6 => Ok(Self::Register(Register::C)),
            _ => Err(anyhow!("Invalid combo operand {}", operand)),
        }
    }
}

impl From<&Combo> for u8 {
    fn from(val: &Combo) -> Self {
        match val {
            Combo::Literal(n) => *n,
            Combo::Register(Register::A) => 4,
            Combo::Register(Register::B) => 5,
            Combo::Register(Register::C) => 6,
        }
    }
}

impl Display for Combo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Combo::Literal(n) => write!(f, "{}", n),
            Combo::Register(register) => write!(f, "{}", register),
        }
    }
}

pub enum Register {
    A,
    B,
    C,
}

impl Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Register::A => write!(f, "A"),
            Register::B => write!(f, "B"),
            Register::C => write!(f, "C"),
        }
    }
}
