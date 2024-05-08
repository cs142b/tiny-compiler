use crate::instruction::Instruction;
use std::collections::HashMap;

#[derive(Debug)]
pub struct BasicBlock {
    pub id: usize,
    pub instructions: Vec<Instruction>,
    pub variable_table: HashMap<String, (usize, usize)>, // (basic block id, instruction index)
}

impl BasicBlock {
    pub fn new(id: usize) -> Self {
        BasicBlock {
            id,
            instructions: Vec::new(),
            variable_table: HashMap::new(),
        }
    }
}
