use crate::instruction::Instruction;

#[derive(Debug)]
pub struct BasicBlock {
    pub id: usize,
    pub instructions: Vec<Instruction>,
}

impl BasicBlock {
    pub fn new(id: usize) -> Self {
        BasicBlock {
            id,
            instructions: Vec::new(),
        }
    }
}