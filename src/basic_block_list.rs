use crate::basic_block::{BasicBlock, BasicBlockType};
use petgraph::{
    graph::{DiGraph, NodeIndex},
    Direction::{Incoming, Outgoing},
};

#[derive(Debug, Clone)]
pub struct BasicBlockList {
    pub bb_graph: DiGraph<BasicBlock, ()>,
    pub curr_node: NodeIndex<u32>,
    entry_node: NodeIndex<u32>
}

impl BasicBlockList {
    // creates a graph and automatically adds in an entry block
    // returns itself
    pub fn new() -> Self {
        let mut bb_g = DiGraph::<BasicBlock, ()>::new();
        let bb = BasicBlock::new(BasicBlockType::Entry);
        let entry_node = bb_g.add_node(bb);

        Self {
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
    pub fn get_bb(&self, node_index: NodeIndex) -> Option<&BasicBlock>{
        if node_index.index() < self.bb_graph.node_count() { 
            Some(&self.bb_graph[node_index])
        } else {
            None
        }
    }

    // accepts a node index and returns an option of an mutable reference to the basic block 
    // living at the index
    pub fn get_bb_mut(&mut self, node_index: NodeIndex) -> Option<&mut BasicBlock> {
        if node_index.index() < self.bb_graph.node_count() { 
            Some(&mut self.bb_graph[node_index])
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


    // keep but might delete later?
    // 1. takes in 2 node_indices
    /// returns the conditional block and adds a loop between the conditional the code block that are below it
  //   pub fn add_loop(&mut self, bb: BasicBlock) -> NodeIndex{
  //
  //       if self.get_curr_bb().get_block_type() != BasicBlockType::Conditional {
  //           panic!("Attempting to add a loop to a non conditional block")
  //       }
  //
  //       let conditional_block = self.add_node_to_curr_bb(bb); 
  //       let curr_code_block = self.curr_node; 
  //
  //       self.add_edge(curr_code_block, conditional_block);
  //
  //       conditional_block
  // 
  //   }

    // pub fn add_loop_from_curr(&mut self, cond_block: NodeIndex) {
    //     self.add_edge(self.curr_node, cond_block);
    // }
    
    pub fn connect_to_conditional(&mut self, conditional_index: NodeIndex) {
        self.add_edge(self.curr_node, conditional_index);
        self.curr_node = conditional_index;
    }

    pub fn add_node_to_index(&mut self, node_index: NodeIndex, bb_type: BasicBlockType) -> NodeIndex<u32> {
        let bb = BasicBlock::new(bb_type);
        let parent_node_index = node_index;

        if !self.can_add_child(parent_node_index.clone()) {
            panic!("Can no longer add any new children");
        }

        let child_node_index = self.bb_graph.add_node(bb);

        let child_node_bb_mut_ref = &mut self.bb_graph[child_node_index];
        child_node_bb_mut_ref.id = child_node_index;

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

    // keep but might delete later?
    // pub fn get_parents_join(&self) -> (NodeIndex, NodeIndex) {
    //     let curr_node = self.curr_node; 
    //     let incoming = self.bb_graph.neighbors_directed(curr_node, Incoming);
    //
    //     let mut left_parent: Option<NodeIndex> = None; 
    //     let mut right_parent: Option<NodeIndex> = None; 
    //
    //     for parent in incoming {
    //         if parent == self.get_prev_index().unwrap() {
    //             left_parent = Some(parent);
    //         } else {
    //             right_parent = Some(parent);
    //         }
    //     }
    //
    //     (left_parent.unwrap(), right_parent.unwrap())
    // }

    /// add a join block to the current set of siblings at the bottom
    pub fn add_join_block(&mut self, left_parent: NodeIndex, right_parent: NodeIndex) -> NodeIndex {
        let join_block = BasicBlock::new(BasicBlockType::Join);

        if !self.can_add_child(left_parent.clone()) {
            panic!("Can no longer add any new children");
        }
        if !self.can_add_child(right_parent.clone()) {
            panic!("Can no longer add any new children");
        }

        let join_node = self.bb_graph.add_node(join_block);

        self.add_edge(left_parent, join_node);
        self.add_edge(right_parent, join_node);

        self.curr_node = join_node;

        self.curr_node
    }



    // keep but might delete later?
    /// returns the node index of its sibling (only used in a child of a conditional)
    /// returns None or panics if the current node does not have a sibling
    // fn get_sibling_of_curr(&self) -> Option<NodeIndex<u32>> {
    //     // if self.curr_node == None {
    //     //     panic!("no curr node to find sibilng of ");
    //     // }
    //
    //     let curr_ni = self.curr_node;
    //
    //     let parent_ni = self.bb_graph.neighbors_directed(curr_ni, Incoming);
    //
    //     if iter_len(&parent_ni) >= 2 {
    //         panic!("too many parents");
    //     }
    //
    //     let parent_children = self
    //         .bb_graph
    //         .neighbors_directed(parent_ni.last().unwrap(), Outgoing);
    //
    //     if iter_len(&parent_children) != 2 {
    //         panic!("Not enough children");
    //     }
    //
    //     for child in parent_children {
    //         if child != self.curr_node {
    //             return Some(child);
    //         }
    //     }
    //
    //     None
    // }

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
        let mut g = BasicBlockList::new(); 

        let conditional = g.add_node_to_curr(BasicBlockType::Conditional);
        let left_parent = g.add_node_to_curr(BasicBlockType::FallThrough);
        let right_parent = g.add_node_to_index(conditional, BasicBlockType::Branch);

        g.add_join_block(left_parent, right_parent);

        println!("{:?}", Dot::with_config(&g.bb_graph, &[Config::EdgeNoLabel]));
    }

    #[test]
    fn while_test() {
        let mut g = BasicBlockList::new(); 

        let conditional = g.add_node_to_curr(BasicBlockType::Conditional);
        g.add_node_to_curr(BasicBlockType::FallThrough);
        g.connect_to_conditional(conditional);
        g.add_node_to_curr(BasicBlockType::Branch);
        
        println!("{:?}", Dot::with_config(&g.bb_graph, &[Config::EdgeNoLabel]));

    }

}
