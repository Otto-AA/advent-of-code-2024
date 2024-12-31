use std::fmt::Display;

use crate::instructions::Instruction;

pub struct Program {
    instructions: Vec<Instruction>,
}

impl Program {
    pub fn new(instructions: Vec<Instruction>) -> Self {
        Program { instructions }
    }

    pub fn get_instruction(&self, pc: usize) -> Option<&Instruction> {
        self.instructions.get(pc / 2)
    }

    pub fn iter_instructions(&self) -> impl Iterator<Item = &'_ Instruction> {
        self.instructions.iter()
    }

    pub fn valid_pc(&self, pc: usize) -> bool {
        pc / 2 < self.instructions.len()
    }
}

impl Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<")?;
        let s: Vec<String> = self
            .instructions
            .iter()
            .map(|instr| instr.to_string())
            .collect();
        write!(f, "{}", s.join(","))?;
        write!(f, ">")
    }
}
