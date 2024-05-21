use crate::basic_block_list::BasicBlockList;
use crate::basic_block::{BasicBlock, BasicBlockType};
use petgraph::graph::NodeIndex;

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub parameters: Vec<String>,
    pub bb_list: BasicBlockList,
}

impl Function {
    pub fn new(name: String, parameters: Vec<String>) -> Self {
        let mut function = Self {
            name,
            parameters,
            bb_list: BasicBlockList::new(),
        };

        function.bb_list.add_node_to_curr(BasicBlockType::Entry);
        function
    }

    pub fn get_current_block(&mut self) -> &mut BasicBlock {
        &mut self.bb_list.bb_graph[self.bb_list.get_current_index()]
    }

    pub fn get_current_index(&mut self) -> NodeIndex {
        self.bb_list.get_current_index()
    }
    
    // wrapper function for add_node_to_curr
    pub fn add_fall_thru_block(&mut self, bb_type: BasicBlockType) -> NodeIndex {
        self.bb_list.add_node_to_curr(bb_type).unwrap()
    }
    
    // wrapper function for add_node_to_prev
    pub fn add_branch_block(&mut self, bb_type: BasicBlockType) -> NodeIndex {
        self.bb_list.add_node_to_prev(bb_type).unwrap()
    }

    // TODO: MERGE THIS WITH EXISTING CODE FOR IT TO WORK
    //
    // pub fn propagate_variables(&mut self, from_block_index: NodeIndex, to_block_index: NodeIndex) {
    //     // Collect variables from the source block
    //     let from_block = self.bb_graph.node_weight(from_block_index).unwrap();
    //     let variables: Vec<(String, isize)> = from_block
    //         .variable_table
    //         .iter()
    //         .filter_map(|(var, &line_num)| line_num.map(|line| (var.clone(), line)))
    //         .collect();
    //
    //     // Apply variables to the destination block
    //     let to_block = self.bb_graph.node_weight_mut(to_block_index).unwrap();
    //     for (var, line) in variables {
    //         to_block.set_variable(&var, line);
    //     }
    // }
    //
    // pub fn propagate_variables(&self) -> HashMap<String, Option<isize>>{
    //     self.variable_table.clone()
    // }
    //
    // pub fn propagate_variables_to_block(&self, next_block: &mut BasicBlock) {
    //     next_block.variable_table = self.propagate_variables();
    // }
    //
    // pub fn generate_phi_instructions(&mut self, join_block_index: NodeIndex) {
    //     // First pass: collect variable information from predecessors
    //     let predecessors: Vec<NodeIndex> = self.bb_graph.neighbors_directed(join_block_index, petgraph::Direction::Incoming).collect();
    //     let mut variable_map: HashMap<String, Vec<isize>> = HashMap::new();
    //
    //     for predecessor_index in &predecessors {
    //         let pred_block = self.bb_graph.node_weight(*predecessor_index).unwrap();
    //         for (var, &line_num) in &pred_block.variable_table {
    //             if let Some(line) = line_num {
    //                 variable_map.entry(var.clone()).or_insert_with(Vec::new).push(line);
    //             }
    //         }
    //     }
    //
    //     // Second pass: generate phi instructions for the join block
    //     let join_block = self.bb_graph.node_weight_mut(join_block_index).unwrap();
    //     for (var, lines) in variable_map {
    //         if lines.len() > 1 {
    //             let phi_instruction = Instruction::create_instruction(join_block.instructions.len() as isize + 1, Operation::Phi(lines[0], lines[1])); // Adjust if there are more than 2 predecessors
    //             join_block.add_instruction(phi_instruction);
    //         }
    //     }
    // }
}
