
use std::collections::{self, HashSet, VecDeque, HashMap};


// use rustc_index::{Idx, IndexVec};

use petgraph::{csr::NodeIdentifiers, graph::{DiGraph, NodeIndex}, Direction::{Incoming, Outgoing}, Graph};

use crate::{basic_block::{BasicBlock, BasicBlockType}, basic_block_list::BasicBlockList};
#[derive(Debug)]
pub struct DominatorTree  {
    bb_vec: Vec<BasicBlock>,
}


impl DominatorTree  {
    pub fn new() -> Self {
        Self {
            bb_vec: Vec::new()
        }
    }

}

fn get_dominator_set(bbl: &BasicBlockList, possible_dominator: &BasicBlock) -> HashSet<NodeIndex> {
    let mut bbl_g = bbl.bb_graph.clone(); 

    let bbl_g_nodes = bbl_g.node_indices();

    let bbl_g_node_set:HashSet<NodeIndex> = bbl_g_nodes.clone().collect();

    bbl_g.remove_node(possible_dominator.id);

    let non_dominated = traverse_graph(&bbl_g, bbl.get_entry_node()); 

    //nodes that were once reachable to nodes that are no longer reachable 

    //dominated set = diff (bbl_g_node_set, non_dominated)

    find_difference(&bbl_g_node_set, &non_dominated)
    
}

fn is_dominator(bbl: &BasicBlockList, possible_dominator: &BasicBlock, possibly_dominated: &BasicBlock, start_point: &BasicBlock) -> Result<bool, &'static str> {
    
    if possible_dominator == start_point || possibly_dominated == possible_dominator || start_point == possibly_dominated {
        return Err("Attempting to start at variable to removed or variable to be checked for (logical error)");
    }
    match (bbl.get_bb(possible_dominator.id), bbl.get_bb(possibly_dominated.id)) {
        (Some(_), Some(_)) => 'check_ok:  {
            break 'check_ok
        }, 
        _ =>  return Err("Attempting to access a basic block that is not in the graph")
    }; 

    let mut bbl = bbl.clone(); 

    bbl.bb_graph.remove_node(possible_dominator.id);

    let non_dom_set = traverse_cfg(&bbl, start_point); 

    // if non dominator set contains the basic block then basicBlock is not dominated, thus the !

    return Ok(!non_dom_set.contains(possibly_dominated));
}

fn traverse_cfg(bbl: &BasicBlockList, start_point: &BasicBlock) -> Vec<BasicBlock> {

    let start_index = start_point.id; 

    let node_index_set =  traverse_graph(&bbl.bb_graph, start_index);

    let mut ret_vec:Vec<BasicBlock> = Vec::new();

    for node in node_index_set {
        ret_vec.push(bbl.get_bb(node).unwrap().clone())
    }

    ret_vec

}
///BFS through nodes that returns the set of seen nodes
fn traverse_graph<T, E>(g: &DiGraph<T, E>, start_point: NodeIndex) -> HashSet<NodeIndex>{

    
    let mut seen_nodes = HashSet::new();
    seen_nodes.insert(start_point);

    let mut curr_queue: VecDeque<NodeIndex> = std::collections::VecDeque::new(); 

    while curr_queue.len() > 0 {
        let curr_ni = curr_queue.pop_front().unwrap();

        seen_nodes.insert(curr_ni);

        let curr_ni_children = g.neighbors_directed(curr_ni, Outgoing);

        for child in curr_ni_children {
            curr_queue.push_back(child);
        }
    }

    seen_nodes 

}

fn find_difference<T: Eq + std::hash::Hash + Clone>(set1: &HashSet<T>, set2: &HashSet<T>) -> HashSet<T> {
    let mut difference = set1.clone();
    difference.retain(|x| !set2.contains(x));
    difference
}

