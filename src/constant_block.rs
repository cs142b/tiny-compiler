use std::collections::HashMap;
use crate::instruction::{Instruction, Operation};

pub struct ConstantBlock {
    constant_table: HashMap<isize, Instruction>,
}

impl ConstantBlock {
    /// Creates and returns a new constant block
    pub fn new() -> Self {
        ConstantBlock {
            constant_table: HashMap::<isize, Instruction>::new(),
        }
    }

    /// Adds a constant into the table
    pub fn add_constant(&mut self, constant: isize) {
        let operation = Operation::Const(constant);
        let line_number = (constant_table.len() + 1) * -1;
        let instruction = Instruction::create_instruction(line_number, operation);

        self.constant_table.insert(constant, instruction);
    }

    /// Gets a constant from the table and returns the line number
    pub fn get_constant(&self, constant: isize) -> isize {
        match self.constant_table.get(isize) {
            Some(instruction) => instruction.line_number,
            None => {
                self.add_constant(constant);
                self.get_constant(constant);
            }
        }
    }
}
