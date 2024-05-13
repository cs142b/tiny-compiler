use crate::instruction::Instruction;
use std::collections::HashMap;
use std::collections::LinkedList;


#[derive(Debug)]
pub struct BasicBlock {
    // pub instructions: Vec<Instruction>,
    pub instruction_list: LinkedList<Instruction>,
    pub variable_table: HashMap<String, isize>, // (variable, line number)
}

impl BasicBlock {
    pub fn new() -> Self {
        Self {
            // instructions: Vec::new(),
            instruction_list: LinkedList::new(),
            variable_table: HashMap::new(),
        }
    }

    pub fn add_instruction(&mut self, instruction: Instruction) {
        self.instruction_list.push_back(instruction);
    }

    pub fn add_variable(&mut self, variable: String, instruction_number: isize) {
        self.variable_table.insert(variable, instruction_number);
    }

    pub fn get_variable(&mut self, variable: &String) -> isize {
        match self.variable_table.get(variable) {
            Some(instruction_number) => *instruction_number,
            None => panic!("ERROR: get_variable() is only used when a known variable exists in the table."),
        }

    }
}
