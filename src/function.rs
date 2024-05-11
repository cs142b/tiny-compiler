use crate::basic_block::BasicBlock;
use petgraph::graph::{DiGraph, Node, NodeIndex};

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub parameters: Vec<String>,
    pub basic_blocks: DiGraph::<BasicBlock, ()>,
}

impl Function {
    pub fn new(name: String, parameters: Vec<String>) -> Self {
        let mut function = Self {
            name,
            parameters,
            basic_blocks: DiGraph::<BasicBlock, ()>::new(),
        };

        function.add_basic_block();
        function
    }

    pub fn add_basic_block(&mut self) -> NodeIndex {
        let new_block = BasicBlock::new();
        self.basic_blocks.add_node(new_block)
    }

    pub fn add_edge(&mut self, from: NodeIndex, to: NodeIndex) {
        self.basic_blocks.add_edge(from, to, ());
    }
}
