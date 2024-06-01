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

    pub fn get_entry_node(&self) -> NodeIndex<u32> {
        self.entry_node
    }
    
    // returns an immutable reference to the current basic block
    pub fn get_curr_bb(&self) -> &BasicBlock{
        &self.bb_graph[self.curr_node]
    }

    // returns an mutable reference to the current basic block
    pub fn get_curr_bb_mut(&mut self) -> &mut BasicBlock {
        return &mut self.bb_graph[self.curr_node]; 
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

    // adds a basic block as a child to the node_to_add
    pub fn add_node_to_index(&mut self, node_to_add: NodeIndex, bb_child: BasicBlock) -> NodeIndex<u32> {
        let added_node_index = self.bb_graph.add_node(bb_child);
        let added_node_mut_block = self.get_bb_mut(added_node_index);
        added_node_mut_block.id = added_node_index; 
        self.add_edge(node_to_add, added_node_index);

        added_node_index
    }

    pub fn add_node_to_index_change_curr(&mut self, node_to_change: NodeIndex, bb: BasicBlock) -> NodeIndex {
        self.curr_node = self.add_node_to_index(node_to_change, bb);
        node_to_change
    }

    /// returns the conditional block and adds a loop between the conditional the code block that are below it
    pub fn add_loop(&mut self, bb: BasicBlock) -> NodeIndex{

        if self.get_curr_bb().get_block_type() != BasicBlockType::Conditional {
            panic!("Attempting to add a loop to a non conditional block")
        }

        let conditional_block = self.add_node_to_curr_bb(bb); 
        let curr_code_block = self.curr_node; 

        self.add_edge(curr_code_block, conditional_block);

        conditional_block
  
    }

    pub fn add_loop_from_curr(&mut self, cond_block: NodeIndex) {
        self.add_edge(self.curr_node, cond_block);
    }

    
    
    /// adds a child node to the current node and returns the current node index
    /// always used if you want to go straight down
    pub fn add_node_to_curr_bb(&mut self, bb: BasicBlock) -> NodeIndex<u32> {

        let parent_node = self.curr_node;

        if !self.can_add_child(parent_node.clone()) {
            panic!("Can no longer add any new children");
        }

        let new_child_node = self.bb_graph.add_node(bb);
        self.curr_node = new_child_node;

        self.add_edge(parent_node, new_child_node);

        parent_node
    }

    pub fn add_node_to_prev_bb(&mut self, bb:BasicBlock) -> NodeIndex {

        let parent_node = self.get_prev();
        if !self.validate_addition(parent_node) {
            panic!("Cannot make addition");
        }

        if !self.can_add_child(parent_node.unwrap().clone()) {
            panic!("Can no longer add any new children");
        }

        let added_node = self.bb_graph.add_node(bb);

        self.add_edge(parent_node.unwrap(), added_node);

        self.curr_node = added_node;

        parent_node.unwrap() 
    }
    
    pub fn add_node_to_curr(&mut self, bb_type: BasicBlockType) -> NodeIndex<u32> {
        let bb = BasicBlock::new(bb_type);

        let parent_node = self.curr_node;

        if !self.can_add_child(parent_node.clone()) {
            panic!("Can no longer add any new children");
        }

        let new_child_node = self.bb_graph.add_node(bb);

        let new_child_node_ref = &mut self.bb_graph[new_child_node];
        new_child_node_ref.id = new_child_node;

        self.curr_node = new_child_node;

        self.add_edge(parent_node, new_child_node);

        parent_node
    }

    /// adds a child node to the previous node and returns the previous node
    /// should only be used in a fall-through block (or left child)
    pub fn add_node_to_prev(&mut self, bb_type: BasicBlockType) -> NodeIndex<u32> {
        let bb = BasicBlock::new(bb_type);

        let parent_node = self.get_prev();
        if !self.validate_addition(parent_node) {
            panic!("Cannot make addition");
        }

        if !self.can_add_child(parent_node.unwrap().clone()) {
            panic!("Can no longer add any new children");
        }

        let added_node = self.bb_graph.add_node(bb);
        let added_node_ref = &mut self.bb_graph[added_node];
        added_node_ref.id = added_node;

        self.add_edge(parent_node.unwrap(), added_node);

        self.curr_node = added_node;

        parent_node.unwrap()
    }

    /// wrapper for [`add_node_to_curr`](#method.add_node_to_curr)
    /// returns the current node index
    pub fn add_fall_thru_block(&mut self, bb_type: BasicBlockType) -> NodeIndex<u32> {
        return self.add_node_to_curr(bb_type);
    }

    /// wrapper for [`add_node_to_prev`](#method.add_node_to_prev)
    /// returns the previous node index
    pub fn add_branch_block(&mut self, bb_type: BasicBlockType) -> NodeIndex<u32> {
        return self.add_node_to_prev(bb_type);
    }

    /// returns the parent of the current node
    pub fn get_prev(&self) -> Option<NodeIndex> {
        let mut parents_iter = self.bb_graph.neighbors_directed(self.curr_node, Incoming);
        parents_iter.nth(0)
    }

    pub fn get_parents_join(&self) -> (NodeIndex, NodeIndex) {
        // if self.bb_graph[self.curr_node].block_type != crate::basic_block_list::BasicBlockType::Join {
        //     panic!("type is not join");
        // }

        let curr_node = self.curr_node; 
        let incoming = self.bb_graph.neighbors_directed(curr_node, Incoming);

        let mut left_parent: Option<NodeIndex> = None; 
        let mut right_parent: Option<NodeIndex> = None; 

        for parent in incoming {
            if parent == self.get_prev().unwrap() {
                left_parent = Some(parent);
            } else {
                right_parent = Some(parent);
            }
        }

        (left_parent.unwrap(), right_parent.unwrap())
    }

    /// add a join block to the current set of siblings at the bottom
    pub fn add_join_block(&mut self, bb_type: BasicBlockType) -> ( NodeIndex<u32>, NodeIndex<u32>) {
        // return (self.curr_node, self.curr_node);
        let bb = BasicBlock::new(bb_type);
        let left_parent = self.get_sibling_of_curr();
        let right_parent = self.curr_node;

        if !self.can_add_child(left_parent.unwrap().clone()) {
            panic!("Can no longer add any new children");
        }
        if !self.can_add_child(right_parent.clone()) {
            panic!("Can no longer add any new children");
        }

        let added_node = self.bb_graph.add_node(bb);

        self.add_edge(left_parent.unwrap(), added_node);
        self.add_edge(right_parent, added_node);

        self.curr_node = added_node;
        (left_parent.unwrap(), right_parent)
    }



    /// returns the node index of its sibling (only used in a child of a conditional)
    /// returns None or panics if the current node does not have a sibling
    fn get_sibling_of_curr(&self) -> Option<NodeIndex<u32>> {
        // if self.curr_node == None {
        //     panic!("no curr node to find sibilng of ");
        // }

        let curr_ni = self.curr_node;

        let parent_ni = self.bb_graph.neighbors_directed(curr_ni, Incoming);

        if iter_len(&parent_ni) >= 2 {
            panic!("too many parents");
        }

        let parent_children = self
            .bb_graph
            .neighbors_directed(parent_ni.last().unwrap(), Outgoing);

        if iter_len(&parent_children) != 2 {
            panic!("Not enough children");
        }

        for child in parent_children {
            if child != self.curr_node {
                return Some(child);
            }
        }

        None
    }

    /// validates whether or not a child can be added to an instnace of a node index
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
    fn basic_test () {
        let mut g = BasicBlockList::new(); 
        g.add_fall_thru_block(BasicBlockType::FallThrough); 
        g.add_node_to_curr(BasicBlockType::Conditional); 
        let conditional_block = g.add_node_to_curr(BasicBlockType::FallThrough);

        g.add_branch_block(BasicBlockType::Branch);
        g.add_join_block(BasicBlockType::Join);

        g.add_fall_thru_block(BasicBlockType::FallThrough);
        g.add_node_to_curr(BasicBlockType::Conditional);
        let top_conditional = g.curr_node;

        g.add_fall_thru_block(BasicBlockType::FallThrough);
        g.add_node_to_curr(BasicBlockType::Conditional);
        g.add_fall_thru_block(BasicBlockType::FallThrough);
        g.add_branch_block(BasicBlockType::FallThrough); 
        let inner_join = g.add_join_block(BasicBlockType::Join);

        g.add_loop_from_curr(top_conditional);
        let bb = BasicBlock::new(BasicBlockType::Branch);

        g.add_node_to_index_change_curr(top_conditional, bb);


        // g.add_join_block(BasicBlockType::Join);






        // let bb = BasicBlock::new(BasicBlockType::Branch);

        // let br_ni = g.bb_graph.add_node(bb);

        // g.add_edge(conditional_block, br_ni); 

        println!("{:?}", Dot::with_config(&g.bb_graph, &[Config::EdgeNoLabel]));


        assert_eq!(g.bb_graph.node_count(), 2);



    }

    /*
    use petgraph::Direction::{Incoming, Outgoing};

    use crate::{
        basic_block::BasicBlock,
        basic_block_list::{in_iter, iter_len},
    };*/

    //use super::BasicBlockList;
    //use crate::basic_block::BasicBlockType;
    /*
    #[test]
    fn first_add_test() {
        let mut x = BasicBlockList::new();
        let ret_val = x.add_node_to_curr(BasicBlockType::Entry);

        assert_eq!(ret_val, None);
        assert_eq!(x.bb_graph.node_count(), 1);
    }
    #[test]
    fn second_add_test_curr() {
        let mut x = BasicBlockList::new();
        let ret_val = x.add_node_to_curr(BasicBlockType::Entry);

        assert_eq!(ret_val, None);

        let _ret_val_2 = x.add_node_to_curr(BasicBlockType::Entry);

        // assert_eq!()
        assert_eq!(x.bb_graph.node_count(), 2);
    }

    #[test]
    fn add_curr_twice_then_prev() {
        let mut x = BasicBlockList::new();
        let ret_val = x.add_node_to_curr(BasicBlockType::Conditional);

        let conditional_node = x.curr_node;

        println!("Edge count first addtion {:?}", x.bb_graph.edge_count());

        let mut cond_node_children = x
            .bb_graph
            .neighbors_directed(conditional_node.unwrap(), Outgoing);

        println!("Getting cond node children");
        for i in cond_node_children.clone() {
            println!("{:?}", i);
        }

        assert_eq!(ret_val, None);
        let ret_val_2 = x.add_node_to_curr(BasicBlockType::FallThrough);

        println!("Edge count second addtion {:?}", x.bb_graph.edge_count());
        let fall_thru_node = x.curr_node;

        cond_node_children = x
            .bb_graph
            .neighbors_directed(conditional_node.unwrap(), Outgoing);
        println!("Getting cond node children 2");
        for i in cond_node_children.clone() {
            println!("{:?}", i);
        }

        assert_eq!(ret_val_2, conditional_node);
        assert_ne!(ret_val_2, x.curr_node);

        let ret_val_3 = x.add_node_to_prev(BasicBlockType::Branch);

        println!(
            "Edge count third node addtion {:?}",
            x.bb_graph.edge_count()
        );
        cond_node_children = x
            .bb_graph
            .neighbors_directed(conditional_node.unwrap(), Outgoing);
        let len = iter_len(&cond_node_children);

        println!("Getting cond node children 3\n");
        for child in cond_node_children {
            println!("{:?}", child);
        }

        assert_eq!(len, 2);

        // assert_eq!(ret_val_3, conditional_node);
        // assert_ne!(ret_val_3)
    }
    #[test]
    fn add_curr_twice_then_prev_then_join() {
        let mut x = BasicBlockList::new();
        let ret_val = x.add_node_to_curr(BasicBlockType::Conditional);

        let conditional_node = x.curr_node;

        println!("Edge count first addtion {:?}", x.bb_graph.edge_count());

        let mut cond_node_children = x
            .bb_graph
            .neighbors_directed(conditional_node.unwrap(), Outgoing);

        println!("Getting cond node children");
        for i in cond_node_children.clone() {
            println!("{:?}", i);
        }

        assert_eq!(ret_val, None);
        let ret_val_2 = x.add_node_to_curr(BasicBlockType::FallThrough);

        println!("Edge count second addtion {:?}", x.bb_graph.edge_count());
        let fall_thru_node = x.curr_node;

        cond_node_children = x
            .bb_graph
            .neighbors_directed(conditional_node.unwrap(), Outgoing);
        println!("Getting cond node children 2");
        for i in cond_node_children.clone() {
            println!("\t{:?}", i);
        }

        assert_eq!(ret_val_2, conditional_node);
        assert_ne!(ret_val_2, x.curr_node);

        let ret_val_3 = x.add_node_to_prev(BasicBlockType::Branch);

        println!(
            "Edge count third node addtion {:?}",
            x.bb_graph.edge_count()
        );
        cond_node_children = x
            .bb_graph
            .neighbors_directed(conditional_node.unwrap(), Outgoing);
        let len = iter_len(&cond_node_children);

        println!("Getting cond node children 3\n");
        for child in cond_node_children.clone() {
            println!("\t{:?}", child);
        }

        assert_eq!(conditional_node, ret_val_3);
        assert_eq!(len, 2);

        assert!(in_iter(&cond_node_children, &x.curr_node.unwrap()));
        assert!(in_iter(
            &cond_node_children,
            &x.get_sibling_of_curr().unwrap()
        ));

        let else_node = x.curr_node;
        let return_add_join = x.add_join_block(BasicBlockType::Join);

        assert_eq!(return_add_join.0, fall_thru_node);
        assert_eq!(return_add_join.1, else_node);

        assert_ne!(x.curr_node, else_node);

        let join_parents = x
            .bb_graph
            .neighbors_directed(x.curr_node.unwrap(), Incoming);

        let join_parents_len = iter_len(&join_parents);

        assert_eq!(join_parents_len, 2);

        // let

        // assert_eq!(cond_node_children.nth(0).clone(), x.curr_node);
        // assert_eq!(cond_node_children.nth(1).clone(), fall_thru_node);

        // assert_eq!(ret_val_3, conditional_node);
        // assert_ne!(ret_val_3)
    }
     */
}
