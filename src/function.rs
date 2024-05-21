use crate::basic_block::BasicBlock;
use crate::instruction::{Instruction, Operation};
use petgraph::{
    data::{Build, DataMap},
    graph::{DiGraph, Graph, Node, NodeIndex},
    visit::IntoNeighborsDirected,
    Direction::{Incoming, Outgoing},
};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub parameters: Vec<String>,
    pub bb_graph: DiGraph<BasicBlock, ()>,
    pub curr_node: Option<NodeIndex<u32>>,
}

impl Function {
    pub fn new(name: String, parameters: Vec<String>) -> Self {
        let mut function = Self {
            name,
            parameters,
            bb_graph: DiGraph::<BasicBlock, ()>::new(),
            curr_node: None,
        };

        // function.add_basic_block(); // need to DO THIS after updating changes
        function
    }

    pub fn is_empty(&self) -> bool {
        self.bb_graph.node_count() == 0
    }

    /// adds node to curr node
    /// allows user access to new parent which is now the curr node
    pub fn add_node_to_curr(&mut self, bb: &BasicBlock) -> Option<NodeIndex<u32>> {
        let parent_node = self.curr_node;

        if parent_node != None {
            if !self.can_add_child(parent_node.unwrap().clone()) {
                panic!("Can no longer add any new children");
            }
        }

        let new_child_node = self.bb_graph.add_node(bb.clone());
        self.curr_node = Some(new_child_node);

        if parent_node != None {
            self.bb_graph.add_edge(parent_node.unwrap(), new_child_node, bb.clone());
        }

        parent_node
    }

    ///adds node to the previous node and allows user access to parent node to which it is adding upon return
    /// fails if validation fails
    pub fn add_node_to_prev(&mut self, bb: &BasicBlock) -> Option<NodeIndex<u32>> {
        // if (self.curr_node.neighbors_directed(n, d))
        // if (self.curr_node.neighbors_directed(a, dir))

        let parent_node = self.get_prev();
        // println!("parent node = {:?}", parent_node);
        if !self.validate_addition(parent_node) {
            panic!("Cannot make addition");
        }

        if !self.can_add_child(parent_node.unwrap().clone()) {
            panic!("Can no longer add any new children");
        }
        if parent_node == None {
            return parent_node;
        }

        let added_node = self.bb_graph.add_node(bb.clone());

        self.bb_graph
            .add_edge(parent_node.unwrap(), added_node, bb.clone());

        self.curr_node = Some(added_node);
        parent_node
    }

    ///wrapper for [`add_node_to_curr`](#method.add_node_to_curr), gives access to block which is now being added to
    pub fn add_fall_thru_block(&mut self, bb: &BasicBlock) -> Option<NodeIndex<u32>> {
        return self.add_node_to_curr(bb);
    }

    ///wrapper for [`add_node_to_prev`](#method.add_node_to_prev), gives access to block which is now being added to
    pub fn add_branch_block(&mut self, bb: &BasicBlock) -> Option<NodeIndex<u32>> {
        return self.add_node_to_prev(bb);
    }

    ///returns the parent of the current node
    fn get_prev(&self) -> Option<NodeIndex> {
        let mut parents_iter = self
            .bb_graph
            .neighbors_directed(self.curr_node.unwrap(), Incoming);
        parents_iter.nth(0)
    }

    ///add a join block to the current set of siblings at the bottom
    pub fn add_join_block(
        &mut self,
        bb: &BasicBlock,
    ) -> (Option<NodeIndex<u32>>, Option<NodeIndex<u32>>) {

        // let right_parent =
        let left_parent = self.get_sibling_of_curr();
        let right_parent = self.curr_node;

        if !self.can_add_child(left_parent.unwrap().clone()) {
            panic!("Can no longer add any new children");
        }
        if !self.can_add_child(right_parent.unwrap().clone()) {
            panic!("Can no longer add any new children");
        }

        let added_node = self.bb_graph.add_node(bb.clone());

        self.bb_graph
            .add_edge(left_parent.unwrap(), added_node, bb.clone());
        self.bb_graph
            .add_edge(right_parent.unwrap(), added_node, bb.clone());

        self.curr_node = Some(added_node);
        (left_parent, right_parent)
    }

    ///gets the sibling of the current node or returns none/panics if the current node does not have a current node
    fn get_sibling_of_curr(&self) -> Option<NodeIndex<u32>> {
        // to get sibiling of curr, must be in the right sibilng as left sibling may not have been created yet

        if self.curr_node == None {
            panic!("no curr node to find sibilng of ");
        }
        let curr_ni = self.curr_node.unwrap();

        let mut parent_ni = self.bb_graph.neighbors_directed(curr_ni, Incoming);

        let mut parent_children = self
            .bb_graph
            .neighbors_directed(parent_ni.nth(0).unwrap(), Outgoing);

        if iter_len(&mut parent_children) != 2 {
            panic!("Not enough children");
        }


        for child in parent_children {
            if child != self.curr_node.unwrap() {
                return Some(child);
            }
        }

        None
    }

    ///validates whether or not a child can be added to an instnace of a node index
    fn can_add_child(&self, possible_parent: petgraph::graph::NodeIndex) -> bool {
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
    fn validate_addition(&self, possible_parent_option: Option<NodeIndex>) -> bool {
        if possible_parent_option == None {
            return false;
        } else {
            return self.can_add_child(possible_parent_option.unwrap());
        }
    }

    fn iter_len(x: &petgraph::graph::Neighbors<BasicBlock, u32>) -> usize {
        let mut i: usize = 0;
        for _el in x.clone() {
            i += 1;
        }

        i
    }

    fn in_iter(
        neighbor_iter: &petgraph::graph::Neighbors<BasicBlock, u32>,
        needle: &NodeIndex<u32>,
    ) -> bool {
        let l = needle.clone();
        for neighbor in neighbor_iter.clone() {
            if neighbor == l {
                return true;
            }
        }

        false
    }

    // pub fn add_basic_block(&mut self) -> NodeIndex {
    //     let new_block = BasicBlock::new();
    //     self.bb_graph.add_node(new_block)
    // }
    //
    // pub fn add_edge(&mut self, from: NodeIndex, to: NodeIndex) {
    //     self.bb_graph.add_edge(from, to, ());
    // }
    //
    // pub fn get_graph(&mut self) -> &mut DiGraph::<BasicBlock, ()> {

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
