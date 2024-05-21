use crate::basic_block::BasicBlock;
use petgraph::{
    data::{Build, DataMap},
    graph::{DiGraph, Graph, Node, NodeIndex},
    visit::IntoNeighborsDirected,
    Direction::{Incoming, Outgoing},
};

#[derive(Debug)]
pub struct BasicBlockList {
    pub bb_graph: DiGraph<BasicBlock, BasicBlock>,
    pub curr_node: Option<NodeIndex<u32>>,
}

impl BasicBlockList {
    pub fn new() -> Self {
        Self {
            bb_graph: DiGraph::<BasicBlock, BasicBlock>::new(),
            curr_node: None,
        }
    }

    pub fn new_with_graph() -> Self {
        let mut m = BasicBlockList::new();
        m.add_node_to_curr(&BasicBlock::new());

        m
    }

    pub fn is_empty(&self) -> bool {
        self.bb_graph.node_count() == 0
    }

    /// adds a child node to the current node and returns the current node index
    /// always used if you want to go straight down
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
            self.bb_graph
                .add_edge(parent_node.unwrap(), new_child_node, bb.clone());
        }

        parent_node
    }

    /// adds a child node to the previous node and returns the previous node 
    /// should only be used in a fall-through block (or left child)
    pub fn add_node_to_prev(&mut self, bb: &BasicBlock) -> Option<NodeIndex<u32>> {
        let parent_node = self.get_prev();
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

    /// wrapper for [`add_node_to_curr`](#method.add_node_to_curr)
    /// returns the current node index
    pub fn add_fall_thru_block(&mut self, bb: &BasicBlock) -> Option<NodeIndex<u32>> {
        return self.add_node_to_curr(bb);
    }

    /// wrapper for [`add_node_to_prev`](#method.add_node_to_prev)
    /// returns the previous node index
    pub fn add_branch_block(&mut self, bb: &BasicBlock) -> Option<NodeIndex<u32>> {
        return self.add_node_to_prev(bb);
    }

    /// returns the parent of the current node
    fn get_prev(&self) -> Option<NodeIndex> {
        let mut parents_iter = self
            .bb_graph
            .neighbors_directed(self.curr_node.unwrap(), Incoming);
        parents_iter.nth(0)
    }

    /// add a join block to the current set of siblings at the bottom
    pub fn add_join_block(
        &mut self,
        bb: &BasicBlock,
    ) -> (Option<NodeIndex<u32>>, Option<NodeIndex<u32>>) {
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

    /// returns the node index of its sibling (only used in a child of a conditional)
    /// returns None or panics if the current node does not have a sibling
    fn get_sibling_of_curr(&self) -> Option<NodeIndex<u32>> {
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
    fn validate_addition(&self, possible_parent_option: Option<NodeIndex>) -> bool {
        if possible_parent_option == None {
            return false;
        } else {
            return self.can_add_child(possible_parent_option.unwrap());
        }
    }
}

// used for testing purposes
fn iter_len(x: &petgraph::graph::Neighbors<BasicBlock, u32>) -> usize {
    let mut i: usize = 0;
    for _el in x.clone() {
        i += 1;
    }
    i
}

// used for testing purposes
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

#[cfg(test)]
mod basic_block_tests {

    use petgraph::Direction::{Incoming, Outgoing};

    use crate::{
        basic_block::{self, BasicBlock},
        basic_block_list::{in_iter, iter_len},
    };

    use super::BasicBlockList;
    use crate::basic_block::BasicBlockType;
    #[test]
    fn first_add_test() {
        let mut x = BasicBlockList::new();
        let j = BasicBlock::new();
        let ret_val = x.add_node_to_curr(&j);

        assert_eq!(ret_val, None);
        assert_eq!(x.bb_graph.node_count(), 1);
    }
    #[test]
    fn second_add_test_curr() {
        let mut x = BasicBlockList::new();
        let j = BasicBlock::new();
        let ret_val = x.add_node_to_curr(&j);

        assert_eq!(ret_val, None);

        let new_bb_2 = BasicBlock::new();
        let _ret_val_2 = x.add_node_to_curr(&new_bb_2);

        // assert_eq!()
        assert_eq!(x.bb_graph.node_count(), 2);
    }

    #[test]
    fn add_curr_twice_then_prev() {
        let mut x = BasicBlockList::new();
        let j = BasicBlock::new_with_type(BasicBlockType::Conditional);
        let ret_val = x.add_node_to_curr(&j);

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
        let g = BasicBlock::new_with_type(BasicBlockType::FallThrough);
        let ret_val_2 = x.add_node_to_curr(&j);

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

        let else_block = BasicBlock::new_with_type(BasicBlockType::Branch);
        let ret_val_3 = x.add_node_to_prev(&else_block);

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
        let j = BasicBlock::new_with_type(BasicBlockType::Conditional);
        let ret_val = x.add_node_to_curr(&j);

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
        let g = BasicBlock::new_with_type(BasicBlockType::FallThrough);
        let ret_val_2 = x.add_node_to_curr(&j);

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

        let else_block = BasicBlock::new_with_type(BasicBlockType::Branch);
        let ret_val_3 = x.add_node_to_prev(&else_block);

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

        let join_block = BasicBlock::new_with_type(BasicBlockType::Join);
        let else_node = x.curr_node;
        let return_add_join = x.add_join_block(&join_block);

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
}
