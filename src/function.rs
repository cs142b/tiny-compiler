use crate::basic_block::BasicBlock;
use petgraph::graph::{NodeIndex, DiGraph};

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub parameters: Vec<String>,
    pub basic_blocks: DiGraph::<BasicBlock, isize>,
}

impl Function {
    pub fn new(name: String) -> Self {
        Function {
            name,
            parameters: Vec::new(),
            basic_blocks: DiGraph::<BasicBlock, isize>::new(),
        }
    }
}
