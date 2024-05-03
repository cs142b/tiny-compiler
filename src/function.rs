use crate::basic_block::BasicBlock;

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub parameters: Vec<String>,
    pub basic_blocks: Vec<BasicBlock>,
}

impl Function {
    pub fn new(name: String) -> Self {
        Function {
            name,
            parameters: Vec::new(),
            basic_blocks: vec![BasicBlock::new(0)], // Block 0 for constants
        }
    }
}