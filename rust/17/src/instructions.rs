use std::fmt::Display;

use anyhow::{anyhow, Error, Result};

pub enum Instruction {
    ADV(Combo),
    BXL(Literal),
    BST(Combo),
    JNZ(Literal),
    BXC(Literal),
    OUT(Combo),
    BDV(Combo),
    CDV(Combo),
}

impl Instruction {
    pub fn parse(opcode: u8, operand: u8) -> Result<Instruction, Error> {
        match opcode {
            0 => Ok(Instruction::ADV(Combo::parse(operand)?)),
            1 => Ok(Instruction::BXL(Literal::parse(operand)?)),
            2 => Ok(Instruction::BST(Combo::parse(operand)?)),
            3 => Ok(Instruction::JNZ(Literal::parse(operand)?)),
            4 => Ok(Instruction::BXC(Literal::parse(operand)?)),
            5 => Ok(Instruction::OUT(Combo::parse(operand)?)),
            6 => Ok(Instruction::BDV(Combo::parse(operand)?)),
            7 => Ok(Instruction::CDV(Combo::parse(operand)?)),
            _ => Err(anyhow!("Invalid opcode {}", opcode)),
        }
    }
}

impl Into<(u8, u8)> for &Instruction {
    fn into(self) -> (u8, u8) {
        match self {
            Instruction::ADV(combo) => (0, combo.into()),
            Instruction::BXL(literal) => (1, literal.into()),
            Instruction::BST(combo) => (2, combo.into()),
            Instruction::JNZ(literal) => (3, literal.into()),
            Instruction::BXC(literal) => (4, literal.into()),
            Instruction::OUT(combo) => (5, combo.into()),
            Instruction::BDV(combo) => (6, combo.into()),
            Instruction::CDV(combo) => (7, combo.into()),
        }
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::ADV(combo) => write!(f, "adv({})", combo),
            Instruction::BXL(Literal(n)) => write!(f, "bxl({})", n),
            Instruction::BST(combo) => write!(f, "bst({})", combo),
            Instruction::JNZ(Literal(n)) => write!(f, "jnz({})", n),
            Instruction::BXC(Literal(_)) => write!(f, "bxc()"),
            Instruction::OUT(combo) => write!(f, "out({})", combo),
            Instruction::BDV(combo) => write!(f, "bdv({})", combo),
            Instruction::CDV(combo) => write!(f, "cdv({})", combo),
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

impl Into<u8> for &Literal {
    fn into(self) -> u8 {
        self.0
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

impl Into<u8> for &Combo {
    fn into(self) -> u8 {
        match self {
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
