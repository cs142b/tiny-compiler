
use std::{collections::{self, HashMap, HashSet, VecDeque}, iter::Enumerate};

use bit_matrix::{BitMatrix}; 
// use rustc_index::{Idx, IndexVec};

use std::iter::Map; 


use petgraph::{csr::NodeIdentifiers, graph::{DiGraph, NodeIndex}, Direction::{Incoming, Outgoing}, Graph};
type Position = usize; 
use crate::{basic_block::{BasicBlock, BasicBlockType}, basic_block_list::BasicBlockList, instruction::Instruction};
#[derive(Debug)]
pub struct DominatorTree  {
    bit_matrix: BitMatrix, 
    node_to_position: HashMap<NodeIndex, Position>,
    position_to_node: HashMap<Position, NodeIndex>
}


type Instructions = Vec<Instruction>; 

// let mut c = 0;

// for pair in ['a', 'b', 'c'].into_iter()
//                                .map(|letter| { c += 1; (letter, c) }) {
//     println!("{pair:?}");


impl DominatorTree  {
    pub fn new(bbl: &BasicBlockList) -> Self {
        // let bbl = bbl.clone();
        let num_nodes = bbl.bb_graph.node_count(); 
        let node_indices_iter = bbl.bb_graph.node_indices(); 

        let mut hmap: HashMap<NodeIndex, Position> = HashMap::new(); 

        let mut position_to_node: HashMap<Position, NodeIndex> = HashMap::new();
        for (idx, NodeIndex) in node_indices_iter.enumerate() {

            if hmap.get(&NodeIndex) != None || position_to_node.get(&idx) == None {
                panic!("Attempting to add duplicate to map")
            }
            hmap.insert(NodeIndex, idx);
            position_to_node.insert(idx, NodeIndex);
        }

        let mut bm: BitMatrix = BitMatrix::new(num_nodes, num_nodes);

        bm.set_all(bit_matrix::FALSE); 

        for (dommy_mommy, idx) in hmap.clone().into_iter() {
            let mut dom_tree_of_curr = get_dominator_set(bbl, &dommy_mommy);

            dom_tree_of_curr.insert(dommy_mommy);

            for dommied_baby in dom_tree_of_curr { 

                let dommy_mommy_idx = hmap.get(&dommy_mommy); 
                let dommied_baby_idx = hmap.get(&dommied_baby);

                if dommied_baby_idx == None|| dommy_mommy_idx == None {
                    panic!("Impressively bad")
                }

                let dommy_mommy_idx = dommy_mommy_idx.unwrap(); 
                let dommied_baby_idx = dommied_baby_idx.unwrap();

                

                bm.set(*dommy_mommy_idx, *dommied_baby_idx, bit_matrix::TRUE)
            }
        }



        // for node in node_indices_iter

        Self {
            // creates an nxn bit matrix where n is the number of nodes
            bit_matrix: bm, 
            node_to_position: hmap.clone(), 
            position_to_node: position_to_node
        }
    }

    pub fn get_dominating_instructions(&self, bbl: &mut BasicBlockList, bb: &BasicBlock) -> Result<Instructions, &str> {

        let dominator = bb.id; 

        let mut ret_vec: Vec<Instruction> = Vec::new();
        let index_in_table = self.node_to_position.get(&dominator); 
        if index_in_table == None {
            return Err("Attempting to find dominator that did not exist in BBL");
        }

        let index_in_matrix = *index_in_table.unwrap();

        for other_position in 0..self.bit_matrix.size().0 {
            if self.bit_matrix[index_in_matrix][other_position] == bit_matrix::TRUE {
                let dominated = self.position_to_node.get(&other_position); 
                let dominated = bbl.get_bb_mut(*dominated.unwrap()); 

                if dominated != None {
                    let mut dominated_vector = &mut dominated.unwrap().instructions; 
                    ret_vec.append(&mut dominated_vector);
                } else {
                    return Err("Attempting to access bad stuff"); 
                }
            }
        }


        Ok(ret_vec)
    }

    pub fn get_dom_instructions(&self, bbl: &mut BasicBlockList, id: &NodeIndex) -> Result<Instructions, &str> {
        let dominator = id; 

        let mut ret_vec: Vec<Instruction> = Vec::new();
        let index_in_table = self.node_to_position.get(&dominator); 
        if index_in_table == None {
            return Err("Attempting to find dominator that did not exist in BBL");
        }

        let index_in_matrix = *index_in_table.unwrap();

        for other_position in 0..self.bit_matrix.size().0 {
            if self.bit_matrix[index_in_matrix][other_position] == bit_matrix::TRUE {
                let dominated = self.position_to_node.get(&other_position); 
                let dominated = bbl.get_bb_mut(*dominated.unwrap()); 

                if dominated != None {
                    let mut dominated_vector = &mut dominated.unwrap().instructions; 
                    ret_vec.append(&mut dominated_vector);
                } else {
                    return Err("Attempting to access bad stuff"); 
                }
            }
        }


        Ok(ret_vec)
    }
    


    

}


fn get_dominator_set(bbl: &BasicBlockList, possible_dominator: &NodeIndex) -> HashSet<NodeIndex> {
    let mut bbl_g = bbl.bb_graph.clone(); 

    let bbl_g_nodes = bbl_g.node_indices();

    let bbl_g_node_set:HashSet<NodeIndex> = bbl_g_nodes.clone().collect();

    bbl_g.remove_node(*possible_dominator);

    let non_dominated = traverse_graph(&bbl_g, bbl.get_entry_node()); 

    //nodes that were once reachable to nodes that are no longer reachable 

    //dominated set = diff (bbl_g_node_set, non_dominated)

    find_difference(&bbl_g_node_set, &non_dominated)
    
}
fn get_dominator_set_from_bb(bbl: &BasicBlockList, possible_dominator: &BasicBlock) -> HashSet<NodeIndex> {
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

