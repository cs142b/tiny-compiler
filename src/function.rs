use crate::basic_block::BasicBlock;
use petgraph::graph::{DiGraph, Node, NodeIndex};
use crate::instruction::{Instruction, Operation};
use std::collections::HashMap;

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

    pub fn get_graph(&mut self) -> &mut DiGraph::<BasicBlock, ()> {
        &mut self.basic_blocks
    }

    pub fn generate_phi_instructions(&mut self, join_block_index: NodeIndex) {
        // First pass: collect variable information from predecessors
        let predecessors: Vec<NodeIndex> = self.basic_blocks.neighbors_directed(join_block_index, petgraph::Direction::Incoming).collect();
        let mut variable_map: HashMap<String, Vec<isize>> = HashMap::new();

        for predecessor_index in &predecessors {
            let pred_block = self.basic_blocks.node_weight(*predecessor_index).unwrap();
            for (var, &line_num) in &pred_block.variable_table {
                if let Some(line) = line_num {
                    variable_map.entry(var.clone()).or_insert_with(Vec::new).push(line);
                }
            }
        }

        // Second pass: generate phi instructions for the join block
        let join_block = self.basic_blocks.node_weight_mut(join_block_index).unwrap();
        for (var, lines) in variable_map {
            if lines.len() > 1 {
                let phi_instruction = Instruction::create_instruction(join_block.instructions.len() as isize + 1, Operation::Phi(lines[0], lines[1])); // Adjust if there are more than 2 predecessors
                join_block.add_instruction(phi_instruction);
            }
        }
    }

    pub fn print_block_instructions(&self, block_index: NodeIndex) {
        if let Some(block) = self.basic_blocks.node_weight(block_index) {
            println!("Instructions in block {:?}:", block_index);
            for instruction in &block.instructions {
                println!("{:?}", instruction);
            }
        } else {
            println!("Block {:?} not found.", block_index);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_phi_instructions() {
        // Create a function
        let mut function = Function::new("test_function".to_string(), vec![]);

        // Add basic blocks
        let block1 = function.add_basic_block();
        let block2 = function.add_basic_block();
        let join_block = function.add_basic_block();

        // Add edges
        function.add_edge(block1, join_block);
        function.add_edge(block2, join_block);

        // Add variables to predecessors
        {
            let mut block1 = function.basic_blocks.node_weight_mut(block1).unwrap();
            block1.set_variable(&"x".to_string(), 1);
            block1.set_variable(&"y".to_string(), 2);

            let mut block2 = function.basic_blocks.node_weight_mut(block2).unwrap();
            block2.set_variable(&"x".to_string(), 3);
            block2.set_variable(&"y".to_string(), 4);
        }

        // Generate phi instructions for join block
        function.generate_phi_instructions(join_block);

        // Check the phi instructions in the join block
        let join_block = function.basic_blocks.node_weight(join_block).unwrap();
        let instructions = &join_block.instructions;

        assert_eq!(instructions.len(), 2);
        assert_eq!(format!("{:?}", instructions[0]), "1: phi (4) (2)");
        assert_eq!(format!("{:?}", instructions[1]), "2: phi (3) (1)");
    }

    #[test]
    fn test_generate_phi_instructions_single_predecessor() {
        // Create a function
        let mut function = Function::new("test_function".to_string(), vec![]);

        // Add basic blocks
        let block1 = function.add_basic_block();
        let join_block = function.add_basic_block();

        // Add edge
        function.add_edge(block1, join_block);

        // Add variables to predecessor
        {
            let mut block1 = function.basic_blocks.node_weight_mut(block1).unwrap();
            block1.set_variable(&"x".to_string(), 1);
        }

        // Generate phi instructions for join block
        function.generate_phi_instructions(join_block);

        // Check the phi instructions in the join block
        let join_block = function.basic_blocks.node_weight(join_block).unwrap();
        let instructions = &join_block.instructions;

        assert_eq!(instructions.len(), 0); // No phi instructions since there's only one predecessor
    }

    #[test]
    fn test_generate_phi_instructions_multiple_variables() {
        // Create a function
        let mut function = Function::new("test_function".to_string(), vec![]);

        // Add basic blocks
        let block1 = function.add_basic_block();
        let block2 = function.add_basic_block();
        let join_block = function.add_basic_block();

        // Add edges
        function.add_edge(block1, join_block);
        function.add_edge(block2, join_block);

        // Add variables to predecessors
        {
            let mut block1 = function.basic_blocks.node_weight_mut(block1).unwrap();
            block1.set_variable(&"x".to_string(), 1);
            block1.set_variable(&"y".to_string(), 2);
            block1.set_variable(&"z".to_string(), 3);

            let mut block2 = function.basic_blocks.node_weight_mut(block2).unwrap();
            block2.set_variable(&"x".to_string(), 4);
            block2.set_variable(&"y".to_string(), 5);
            block2.set_variable(&"z".to_string(), 6);
        }

        // Generate phi instructions for join block
        function.generate_phi_instructions(join_block);

        function.print_block_instructions(join_block);

        // Check the phi instructions in the join block
        let join_block = function.basic_blocks.node_weight(join_block).unwrap();
        let instructions = &join_block.instructions;

        assert_eq!(instructions.len(), 3);
        assert_eq!(format!("{:?}", instructions[0]), "1: phi (6) (3)");
        assert_eq!(format!("{:?}", instructions[1]), "2: phi (5) (2)");
        assert_eq!(format!("{:?}", instructions[2]), "3: phi (4) (1)");
    }
    
    #[test]
    fn test_generate_phi_instructions_with_repeated_assignments() {
        // Create a function
        let mut function = Function::new("test_function".to_string(), vec![]);

        // Add basic blocks
        let block1 = function.add_basic_block();
        let block2 = function.add_basic_block();
        let join_block = function.add_basic_block();

        // Add edges
        function.add_edge(block1, join_block);
        function.add_edge(block2, join_block);

        // Add variables to predecessors
        {
            let mut block1 = function.basic_blocks.node_weight_mut(block1).unwrap();
            // this already works because set_variable will always override :)
            block1.set_variable(&"x".to_string(), 1);
            block1.set_variable(&"y".to_string(), 2);
            block1.set_variable(&"y".to_string(), 3);

            let mut block2 = function.basic_blocks.node_weight_mut(block2).unwrap();
            block2.set_variable(&"x".to_string(), -1);
            block2.set_variable(&"y".to_string(), -3);
        }
    }
}
