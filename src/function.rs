use crate::{basic_block::{BasicBlock, BasicBlockType, VariableType}, instruction::Operation};
use std::collections::HashMap;
use petgraph::{
    graph::{DiGraph, NodeIndex},
    Direction::Incoming,
};

use crate::dot_viz::generate_dot_viz;

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub parameters: Vec<String>,
    pub is_void: bool,

    pub bb_graph: DiGraph<BasicBlock, BasicBlockType>,
    pub curr_node: NodeIndex<u32>,
    entry_node: NodeIndex<u32>
}

impl Function {
    // creates a graph and automatically adds in an entry block
    // returns itself
    pub fn new(name: String, is_void: bool) -> Self {
        let mut bb_g = DiGraph::<BasicBlock, BasicBlockType>::new();
        let bb = BasicBlock::new(BasicBlockType::Entry);
        let entry_node = bb_g.add_node(bb);

        Self {
            name,
            parameters: Vec::new(),
            is_void,

            bb_graph: bb_g,
            curr_node: entry_node,
            entry_node
        }
    }

    pub fn insert_new_parameter(&mut self, parameter_name: String) {
        self.parameters.push(parameter_name);
    }

    // returns the node index of the entry node
    pub fn get_entry_node(&self) -> NodeIndex<u32> {
        self.entry_node
    }

    // returns an immutable reference to the current basic block
    pub fn get_curr_bb(&self) -> &BasicBlock{
        &self.bb_graph[self.curr_node]
    }

    // returns an mutable reference to the current basic block
    pub fn get_curr_bb_mut(&mut self) -> &mut BasicBlock {
        &mut self.bb_graph[self.curr_node]
    }

    // accepts a node index and returns an option of an immutable reference to the basic block 
    // living at the index
    pub fn get_bb(&self, node_index: &NodeIndex) -> Option<&BasicBlock>{
        if node_index.index() < self.bb_graph.node_count() { 
            Some(&self.bb_graph[*node_index])
        } else {
            None
        }
    }

    // accepts a node index and returns an option of an mutable reference to the basic block 
    // living at the index
    pub fn get_bb_mut(&mut self, node_index: &NodeIndex) -> Option<&mut BasicBlock> {
        if node_index.index() < self.bb_graph.node_count() { 
            Some(&mut self.bb_graph[*node_index])
        } else {
            None
        }
    }

    // adds an edge FROM a node index TO a node index
    pub fn add_edge(&mut self, from_index: NodeIndex, to_index: NodeIndex, edge_type: BasicBlockType) {
        self.bb_graph.add_edge(from_index, to_index, edge_type);
    }

    // returns the current index of the graph
    pub fn get_current_index(&self) -> NodeIndex<u32> {
        self.curr_node
    }



    pub fn connect_to_conditional(&mut self, conditional_index: NodeIndex) {
        self.add_edge(self.curr_node, conditional_index, BasicBlockType::Conditional);
        self.curr_node = conditional_index;
    }

    pub fn add_node_to_index(&mut self, node_index: NodeIndex, bb_type: BasicBlockType) -> NodeIndex<u32> {
        let bb = BasicBlock::new(bb_type);

        let parent_node_index = node_index;
        let parent_block = self.get_bb(&parent_node_index).unwrap().clone();

        if !self.can_add_child(parent_node_index.clone()) {
            panic!("Can no longer add any new children");
        }

        let child_node_index = self.bb_graph.add_node(bb);

        let child_node_bb_mut_ref = self.get_bb_mut(&child_node_index).unwrap();
        child_node_bb_mut_ref.id = child_node_index;

        // variable propagation, clones the variable table
        child_node_bb_mut_ref.variable_table = parent_block.variable_table.clone();
        
        // dominator propagation, clones the dominator table
        child_node_bb_mut_ref.dominator_table = parent_block.dominator_table.clone();
        child_node_bb_mut_ref.dominator_table.dominated_by = parent_node_index;

        self.curr_node = child_node_index;
        self.add_edge(parent_node_index, child_node_index, bb_type);

        self.curr_node
    }

    // creates a new basic block and appends it to the current node
    // returns that new current node index
    pub fn add_node_to_curr(&mut self, bb_type: BasicBlockType) -> NodeIndex<u32> {
        self.add_node_to_index(self.curr_node, bb_type)
    }

    /// returns the parent of the current node
    pub fn get_prev_index(&self) -> Option<NodeIndex> {
        let mut parents_iter = self.bb_graph.neighbors_directed(self.curr_node, Incoming);
        parents_iter.nth(0)
    }

    pub fn get_prev_index_of_node(&self, node: NodeIndex) -> Option<NodeIndex> {
        let mut parents_iter = self.bb_graph.neighbors_directed(node, Incoming);
        parents_iter.nth(0)
    }

    /// add a join block to the current set of siblings at the bottom
    pub fn add_join_block(&mut self, left_parent_index: NodeIndex, right_parent_index: NodeIndex) -> (NodeIndex, Vec<(Operation, String)>) {
        let mut join_block = BasicBlock::new(BasicBlockType::Join);

        // variable propagation, clones the variable table
        let (new_table, phi_instructions) = self.join_variable_tables(left_parent_index, right_parent_index);
        join_block.variable_table = new_table;

        // dominator propagation, clones the dominator table
        join_block.dominator_table = self.get_bb(&self.get_prev_index_of_node(left_parent_index).unwrap()).unwrap().dominator_table.clone();
        join_block.dominator_table.dominated_by = self.get_prev_index_of_node(left_parent_index).unwrap();

        let join_node = self.bb_graph.add_node(join_block);

        self.add_edge(left_parent_index, join_node, BasicBlockType::Branch);
        self.add_edge(right_parent_index, join_node, BasicBlockType::FallThrough);

        self.curr_node = join_node;


        (self.curr_node, phi_instructions)
    }

    /// Join two blocks with one block as the target, propagating the variable table
    pub fn join_with_target(&mut self, source_index: NodeIndex, target_index: NodeIndex) -> Vec<(Operation, String)> {
        let (new_table, phi_instructions) = self.join_variable_tables(source_index, target_index);
        self.get_bb_mut(&target_index).unwrap().variable_table = new_table;
        phi_instructions
    }

    // Helper function to join variable tables
    fn join_variable_tables(&self, left_parent_index: NodeIndex, right_parent_index: NodeIndex) -> (HashMap<String, VariableType>, Vec<(Operation, String)>) {
        let left_parent_block = self.get_bb(&left_parent_index).unwrap();
        let right_parent_block = self.get_bb(&right_parent_index).unwrap();

        let mut join_variable_table = HashMap::<String, VariableType>::new();
        let mut phi_instructions = Vec::<(Operation, String)>::new();

        for (variable, left_value) in &left_parent_block.variable_table {
            let right_value = right_parent_block.variable_table.get(variable).unwrap();
            if left_value == right_value {
                join_variable_table.insert(variable.clone(), left_value.clone());
            } else {
                phi_instructions.push((Operation::Phi(left_value.get_value(), right_value.get_value()), variable.clone()));
            }
        }

        (join_variable_table, phi_instructions)
    }

    /// Get the single outgoing edge from a given block
    pub fn get_outgoing_edge(&self, block_index: NodeIndex) -> Option<NodeIndex> {
        let graph = &self.bb_graph;
        let mut neighbors = graph.neighbors_directed(block_index, petgraph::Direction::Outgoing);

        // Check if there is exactly one outgoing edge
        if let Some(target_index) = neighbors.next() {
            if neighbors.next().is_none() {
                return Some(target_index);
            } else {
                panic!("Block has more than one outgoing edge");
            }
        } else {
            None
        }
    }

    /// Get the single incoming edge from a given block
    pub fn get_incoming_edges(&self, block_index: NodeIndex) -> Vec<NodeIndex> {
        let graph = &self.bb_graph;
        graph
            .neighbors_directed(block_index, petgraph::Direction::Incoming)
            .collect()
    }




    /// validates whether or not a child can be added to an instnace of a node index
    fn can_add_child(&self, possible_parent: NodeIndex) -> bool {
        let parent_bb = self.bb_graph.node_weight(possible_parent);

        let max_children = parent_bb.unwrap().get_max_children();

        let curr_children = self.bb_graph.neighbors(possible_parent);

        let num_curr_children = iter_len(&curr_children);
        if num_curr_children < max_children {
            return true;
        }

        false
    }

    ///wrapper function for can_add_child()
    /// wrapper function for [`can_add_child`](#method.can_add_child)
    fn validate_addition(&self, possible_parent_option: Option<NodeIndex<u32>>) -> bool {
        if possible_parent_option == None {
            return false;
        }
        return self.can_add_child(possible_parent_option.unwrap());
    }


}

// used for testing purposes
fn iter_len(x: &petgraph::graph::Neighbors<BasicBlockType, u32>) -> usize {
    let mut i: usize = 0;
    for _el in x.clone() {
        i += 1;
    }
    i
}

// used for testing purposes
fn in_iter(neighbor_iter: &petgraph::graph::Neighbors<BasicBlockType, u32>, needle: &NodeIndex<u32>) -> bool {
    let l = needle.clone();
    for neighbor in neighbor_iter.clone() {
        if neighbor == l {
            return true;
        }
    }
    false
}


// #[cfg(test)]
// mod basic_block_tests {
//
//     use super::*;
//
//     use petgraph::dot::{Dot, Config};
//
//     #[test]
//     fn conditional_test() {
//         let parameters = vec![String::from("test")];
//         let mut g = Function::new(String::from("test"), parameters); 
//
//         let conditional = g.add_node_to_curr(BasicBlockType::Conditional);
//         let left_parent = g.add_node_to_curr(BasicBlockType::FallThrough);
//         let right_parent = g.add_node_to_index(conditional, BasicBlockType::Branch);
//
//         g.add_join_block(left_parent, right_parent);
//
//         println!("{:?}", generate_dot_viz(&g.bb_graph));
//
//
//     }
//
//     #[test]
//     fn while_test() {
//         let parameters = vec![String::from("test")];
//         let mut g = Function::new(String::from("test"), parameters); 
//
//         let conditional = g.add_node_to_curr(BasicBlockType::Conditional);
//         g.add_node_to_curr(BasicBlockType::FallThrough);
//         g.connect_to_conditional(conditional);
//         g.add_node_to_curr(BasicBlockType::Branch);
//         println!("{:?}", Dot::with_config(&g.bb_graph, &[Config::EdgeNoLabel]));
//     }
//
// }
