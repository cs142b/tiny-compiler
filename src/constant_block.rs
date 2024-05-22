use std::collections::HashMap;
use crate::instruction::{Instruction, Operation};

#[derive(Debug)]
pub struct ConstantBlock {
    constant_table: HashMap<isize, Instruction>,
}

impl ConstantBlock {
    /// Creates and returns a new constant block
    pub fn new() -> Self {
        Self {
            constant_table: HashMap::<isize, Instruction>::new(),
        }
    }

    /// Adds a constant into the table
    pub fn add_constant(&mut self, constant: isize) {
        let operation = Operation::Const(constant);
        let line_number = constant * -1; // (self.constant_table.len() as isize) * -1 - 1; // len() returns usize
        let instruction = Instruction::new(line_number, operation);

        self.constant_table.insert(constant, instruction);
    }

    /// Gets a constant from the table and returns the line number
    pub fn get_constant(&mut self, constant: isize) -> isize {
        match self.constant_table.get(&constant) {
            Some(instruction) => instruction.get_line_number(),
            None => {
                self.add_constant(constant);
                self.get_constant(constant)
            }
        }
    }
    
    // used for testing purposes to display values
    pub fn display_table(&self) {
        for instruction in self.constant_table.values() {
            println!("{:?}", instruction);
        }
    }

}
