use crate::basic_block::{BasicBlock, BasicBlockType, VariableType};
use std::collections::HashMap;
use petgraph::{
    graph::{DiGraph, NodeIndex},
    Direction::{Incoming, Outgoing},
};

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub parameters: Vec<String>,

    pub bb_graph: DiGraph<BasicBlock, ()>,
    pub curr_node: NodeIndex<u32>,
    entry_node: NodeIndex<u32>
}

impl Function {
    // creates a graph and automatically adds in an entry block
    // returns itself
    pub fn new(name: String, parameters: Vec<String>) -> Self {
        let mut bb_g = DiGraph::<BasicBlock, ()>::new();
        let bb = BasicBlock::new(BasicBlockType::Entry);
        let entry_node = bb_g.add_node(bb);

        Self {
            name,
            parameters, 

            bb_graph: bb_g,
            curr_node: entry_node,
            entry_node
        }
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
    pub fn add_edge(&mut self, from_index: NodeIndex, to_index: NodeIndex) {
        self.bb_graph.add_edge(from_index, to_index, ());
    }

    // returns the current index of the graph
    pub fn get_current_index(&self) -> NodeIndex<u32> {
        self.curr_node
    }



    pub fn connect_to_conditional(&mut self, conditional_index: NodeIndex) {
        self.add_edge(self.curr_node, conditional_index);
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

        self.curr_node = child_node_index;
        self.add_edge(parent_node_index, child_node_index);

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


    /// add a join block to the current set of siblings at the bottom
    pub fn add_join_block(&mut self, left_parent_index: NodeIndex, right_parent_index: NodeIndex) -> NodeIndex {
        let mut join_block = BasicBlock::new(BasicBlockType::Join);

        if !self.can_add_child(left_parent_index.clone()) {
            panic!("Can no longer add any new children");
        }
        if !self.can_add_child(right_parent_index.clone()) {
            panic!("Can no longer add any new children");
        }


        // propogate variables in join block
        let left_parent_block = self.get_bb(&left_parent_index).unwrap();
        let right_parent_block = self.get_bb(&right_parent_index).unwrap();



        let mut join_variable_table = HashMap::<String, VariableType>::new();
        for (variable, left_value) in &left_parent_block.variable_table {
            if let Some(right_value) = &right_parent_block.variable_table.get(variable) {
                if left_value.get_not_phi_value() != (*right_value).get_not_phi_value() {
                   join_variable_table.insert(variable.to_string(), VariableType::Phi(left_value.get_not_phi_value(), right_value.get_not_phi_value()));
                } else {
                    join_variable_table.insert(variable.to_string(), VariableType::NotPhi(left_value.get_not_phi_value()));
                }
            }
        }

        join_block.variable_table = join_variable_table;

        let join_node = self.bb_graph.add_node(join_block);

        self.add_edge(left_parent_index, join_node);
        self.add_edge(right_parent_index, join_node);

        self.curr_node = join_node;

        self.curr_node
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
fn iter_len(x: &petgraph::graph::Neighbors<(), u32>) -> usize {
    let mut i: usize = 0;
    for _el in x.clone() {
        i += 1;
    }
    i
}

// used for testing purposes
fn in_iter(neighbor_iter: &petgraph::graph::Neighbors<(), u32>, needle: &NodeIndex<u32>) -> bool {
    let l = needle.clone();
    for neighbor in neighbor_iter.clone() {
        if neighbor == l {
            return true;
        }
    }
    false
}


#[cfg(test)]
mod basic_block_tests {

    use super::*;

    use petgraph::dot::{Dot, Config};

    #[test]
    fn conditional_test() {
        let parameters = vec![String::from("test")];
        let mut g = Function::new(String::from("test"), parameters); 

        let conditional = g.add_node_to_curr(BasicBlockType::Conditional);
        let left_parent = g.add_node_to_curr(BasicBlockType::FallThrough);
        let right_parent = g.add_node_to_index(conditional, BasicBlockType::Branch);

        g.add_join_block(left_parent, right_parent);

        println!("{:?}", Dot::with_config(&g.bb_graph, &[Config::EdgeNoLabel]));
    }

    #[test]
    fn while_test() {
        let parameters = vec![String::from("test")];
        let mut g = Function::new(String::from("test"), parameters); 

        let conditional = g.add_node_to_curr(BasicBlockType::Conditional);
        g.add_node_to_curr(BasicBlockType::FallThrough);
        g.connect_to_conditional(conditional);
        g.add_node_to_curr(BasicBlockType::Branch);
        println!("{:?}", Dot::with_config(&g.bb_graph, &[Config::EdgeNoLabel]));
    }
    
    #[test]
    fn join_variable_table() {

        let mut left_variable_table = HashMap::<String, VariableType>::new();
        left_variable_table.insert(String::from("test1"), VariableType::NotPhi(1));
        left_variable_table.insert(String::from("test2"), VariableType::NotPhi(2));

        let mut right_variable_table = HashMap::<String, VariableType>::new();
        right_variable_table.insert(String::from("test1"), VariableType::NotPhi(1));
        right_variable_table.insert(String::from("test2"), VariableType::NotPhi(3));

        let mut join_variable_table = HashMap::<String, VariableType>::new();

        for (variable, left_value) in &left_variable_table {
            if let Some(right_value) = &right_variable_table.get(variable) {
                if left_value.get_not_phi_value() != (*right_value).get_not_phi_value() {
                   join_variable_table.insert(variable.to_string(), VariableType::Phi(left_value.get_not_phi_value(), right_value.get_not_phi_value()));
                } else {
                    join_variable_table.insert(variable.to_string(), VariableType::NotPhi(left_value.get_not_phi_value()));
                }
            }
        }
        

        assert_eq!(format!("{:?}", join_variable_table), "{\"test1\": 1, \"test2\": 2 != 3}"); 


    }

}
