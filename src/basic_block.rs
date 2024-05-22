use crate::instruction::Instruction;
use std::collections::HashMap;
use std::fmt;

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

#[derive(Clone)]
pub enum VariableType {
    Phi(isize, isize),
    NotPhi(isize),
}

impl fmt::Debug for VariableType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            VariableType::Phi(value1, value2) => write!(f, "{} != {}", value1, value2),
            VariableType::NotPhi(value) => write!(f, "{}", value),
            _ => unreachable!("should never hit here in debug formatting for variable type"),
        }
    }

}

impl VariableType {
    fn is_phi(&self) -> bool {
        match self {
            VariableType::Phi(_, _) => true,
            VariableType::NotPhi(_) => false,
        }
    }

    fn get_phi_values(&self) -> (isize, isize) {
        if let VariableType::Phi(value1, value2) = self {
            return (*value1, *value2);
        } 

        panic!("this function should only be used if variabletype is known to be a phi");
    }

    fn get_not_phi_value(&self) -> isize {
        if let VariableType::NotPhi(value) = self {
            return *value;
        }
        
        panic!("this function should only be used if variabletype is known to be a not phi");
    }
}

#[derive(Debug, Default, Clone)]
pub struct BasicBlock {
    pub instructions: Vec<Instruction>,
    pub variable_table: HashMap<String, Option<VariableType>>, 
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

    pub fn get_variable(&self, variable: &String) -> VariableType {
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
