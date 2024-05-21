use crate::instruction::Instruction;
use std::collections::HashMap;
use petgraph::graph::NodeIndex;

#[derive(Debug)]
pub struct BasicBlock {
    pub instructions: Vec<Instruction>,
    pub variable_table: HashMap<String, Option<isize>>, // (variable, line number)
    pub successors: Vec<NodeIndex>, // list of successor blocks
}

impl BasicBlock {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
            variable_table: HashMap::new(),
            successors: Vec::new(),
        }
    }

    pub fn add_instruction(&mut self, instruction: Instruction) {
        self.instructions.push(instruction);
    }

    pub fn add_variable(&mut self, variable: &String) {
        self.variable_table.insert(variable.to_string(), None);
    }

    pub fn get_variable(&self, variable: &String) -> isize {
        match self.variable_table.get(variable) {
            Some(&instruction_number) => instruction_number.unwrap(),
            None => panic!("ERROR: get_variable() is only used when a known variable exists in the table."),
        }
    }

    pub fn set_variable(&mut self, variable: &String, instruction_number: isize) {
        self.variable_table.insert(variable.to_string(), Some(instruction_number)); // will override the add
    }

    pub fn get_first_instruction_line_number(&self) -> isize {
        if let Some(instruction) = self.instructions.first() {
            instruction.get_line_number()
        } else {
            panic!("Basic block is empty")
        }
    }

    pub fn add_successor(&mut self, successor: NodeIndex) {
        self.successors.push(successor);
    }

    pub fn propagate_variables(&self, next_block: &mut BasicBlock) {
        for (var, &line_num) in &self.variable_table {
            if let Some(line) = line_num {
                next_block.set_variable(var, line);
            }
        }
    }
}
