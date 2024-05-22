use crate::instruction::Instruction;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, Default)]
pub enum BasicBlockType {
    #[default]
    Entry, 
    Conditional, 
    FallThrough, 
    Branch,
    Join, 
    Exit 
}

#[derive(Debug, Default, Clone)]
pub struct BasicBlock {
    pub instructions: Vec<Instruction>,
    pub variable_table: HashMap<String, Option<isize>>, // (variable, line number)
    pub block_type: BasicBlockType,
}

impl BasicBlock {
    pub fn new(block_type: BasicBlockType) -> Self {
        Self {
            instructions: Vec::new(),
            variable_table: HashMap::new(),
            block_type,
        }
    }


    pub fn add_instruction(&mut self, instruction: Instruction) {
        self.instructions.push(instruction);
    }

    pub fn initalize_variable(&mut self, variable: &String) {
        self.variable_table.insert(variable.to_string(), None);
    }

    pub fn get_variable(&self, variable: &String) -> isize {
        match self.variable_table.get(variable) {
            Some(&instruction_number) => instruction_number.unwrap(),
            None => panic!("ERROR: get_variable() is only used when a known variable exists in the table."),
        }
    }

    pub fn assign_variable(&mut self, variable: &String, instruction_number: isize) {
        self.variable_table.insert(variable.to_string(), Some(instruction_number)); // will override the add
    }


    pub fn get_first_instruction_line_number(&self) -> isize {
        if let Some(instruction) = self.instructions.first() {
            instruction.get_line_number()
        } else {
            panic!("Basic block is empty")
        }
    }


    // pub fn propagate_variables(&self, next_block: &mut BasicBlock) {
    //     for (var, &line_num) in &self.variable_table {
    //         if let Some(line) = line_num {
    //             next_block.assign_variable(var, line);
    //         }
    //     }
    // }

    pub fn get_max_parents(&self) -> usize {
        match self.block_type {
            BasicBlockType::Entry => return 0,
            BasicBlockType::Conditional | BasicBlockType::Branch | BasicBlockType::FallThrough => return 1, 
            BasicBlockType::Join => return 2, 
            BasicBlockType::Exit =>  return 1
        }
    }

    pub fn get_max_children(&self) -> usize {
        match self.block_type {
            BasicBlockType::Entry => return 1,
            BasicBlockType::Branch | BasicBlockType::FallThrough => return 1, 
            BasicBlockType::Conditional => return 2, 
            BasicBlockType::Join => return 1, 
            BasicBlockType::Exit =>  return 0
        }
    }
}
