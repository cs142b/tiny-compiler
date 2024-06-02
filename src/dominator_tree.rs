//
// use std::collections::{self, HashSet, VecDeque, HashMap};
//
//
// // use rustc_index::{Idx, IndexVec};
//
// use petgraph::{csr::NodeIdentifiers, graph::{DiGraph, NodeIndex}, Direction::{Incoming, Outgoing}, Graph};
//
// use crate::{basic_block::{BasicBlock, BasicBlockType}, basic_block_list::BasicBlockList};
// #[derive(Debug)]
// pub struct DominatorTree  {
//     bb_vec: Vec<BasicBlock>,
// }
//
// // fn find_next_shared_block() {
//     
// // }
//
// // fn go_up_conditionals_resolving_all_joins(curr_ni: &NodeIndex, graph: &DiGraph<BasicBlock, ()>, num_conditionals: usize) {
// //     let mut num_conditionals = num_conditionals;
// //     for i in 0..num_conditionals {
// //         num_conditionals += go_up_til_conditional_shortest_path_always_up(curr_ni, graph); 
// //     }
// // }
//
// // fn go_up_til_conditional_shortest_path_always_up(curr_ni: &NodeIndex, graph: &DiGraph<BasicBlock, ()>) -> usize {
// //     // let parent_to_go = curr_ni;
//
// //     let actual_node_inc = &go_up_shortest_path(curr_ni, graph); 
// //     go_up_til_conditional_shortest_path(actual_node_inc, graph)
//
//     
// // }
// // /// uses heuristics to estimate shortest path upwards
// // fn go_up_til_conditional_shortest_path(curr_ni: &NodeIndex, graph: &DiGraph<BasicBlock, ()>) -> usize {
// //     let mut parent_to_go_up = *curr_ni; 
// //     let mut num_joins_seen:usize  = 0; 
// //     // let j = graph[parent_to_go_up]; 
//
// //     while graph[parent_to_go_up].block_type != BasicBlockType::Conditional {
// //         if graph[parent_to_go_up].block_type == BasicBlockType::Join {
// //             num_joins_seen += 1; 
// //         }
//
// //         parent_to_go_up = go_up_shortest_path(&parent_to_go_up, graph);
// //     }
//
// //     num_joins_seen
// // }
//
// // fn go_up_shortest_path(curr_ni: &NodeIndex, graph: &DiGraph<BasicBlock, ()>) -> NodeIndex {
// //     //desinged to be used when there are two parents and want to go up on non_join block 
// //     // (in ideal case, will still work on join block go up but will be random)
// //     let parents = graph.neighbors_directed(*curr_ni, Incoming); 
//
// //     let mut parents = parents.peekable(); 
//
// //     let first_parent_selected = parents.peek().unwrap(); 
//
// //     if graph[*first_parent_selected].block_type != BasicBlockType::Join {
// //         return *first_parent_selected; 
// //     } else {
// //         parents.next(); 
//
//
// //         if parents.peek() != None {
// //             return *parents.peek().unwrap();
// //         } else {
// //             return *first_parent_selected;
// //         }
// //     }
//
//
//     
//
//
// // }
//
// // fn find_next_shared_block_non_join(curr_ni: &NodeIndex, graph: &DiGraph<BasicBlock, ()>) -> NodeIndex{
// //     go_up(curr_ni, graph)
// // }
//
// // fn find_next_shared_block_join(curr_ni: &NodeIndex, graph: &DiGraph<BasicBlock, ()>) {
// //     let parents = go_up_both(curr_ni, graph); 
// //     let mut left_parent = parents.0.unwrap(); 
// //     let mut right_parent = parents.0.unwrap(); 
//
// //     while left_parent != right_parent {
//        
// //     }
// // }
//
//
// // fn go_up_right (curr_ni: &NodeIndex, graph: &DiGraph<BasicBlock, ()>) -> Option<NodeIndex> {
//
// // }
//
// /// returns a tuple of left side and then right parent
// /// in the case that these are the same, this will be irrelevant and choose at random
// // fn go_up_both(curr_ni: &NodeIndex, graph: &DiGraph<BasicBlock, ()>) -> (Option<NodeIndex>, Option<NodeIndex>) {
// //     // input should be a nodeIndex with a basic block type of join
//
// //     let parents = graph.neighbors(*curr_ni);
// //     let mut parents = parents.peekable(); 
//
// //     let parent_1 = parents.peek(); 
// //     parents.next(); 
// //     let parent_2 = parents.peek();  
//
// //     if parent_1 == None || parent_2 == None {
// //         return (None, None);
// //     }
//
// //     //confirmed has two parents (is join block in essence)
//
//
// //     //needs to be dereferenced as peekable returns a reference
//
// //     let parent_1_bb = graph[*parent_1.unwrap()];
// //     let parent_2_bb = graph[*parent_2.unwrap()];
//
//     
// //     if parent_1_bb.block_type == BasicBlockType::Join && parent_1_bb.block_type == parent_2_bb.block_type {
// //         //both parents are Joins here so going up has no impact regardless of which one you choose
// //         return (Some(*parent_2.unwrap()), Some(*parent_1.unwrap()));
// //         //in this case, both go up should be called
// //     } else if parent_1_bb.block_type == BasicBlockType::Join || parent_1_bb.block_type == BasicBlockType::Branch { 
// //         return (Some(*parent_2.unwrap()), Some(*parent_1.unwrap()));
// //     } else { //if parent_2_bb.block_type == BasicBlockType::Join || parent_2_bb == BasicBlockType::Branch {
// //         return (Some(*parent_1.unwrap()), Some(*parent_2.unwrap()));
// //     }
// // }
// // /// below function should only be used when there is one neighbor as this works without checking the type of basic block
// // /// there is no way in a graph strcuture to check if there is a left or right parent using neighbors so just one parent should be returned
// // /// as such .last can be called on the neighbor iterator
// // /// returns the parent of the current iterator
// // fn go_up <T> (curr_ni: &NodeIndex, graph: &DiGraph<T, ()>) -> NodeIndex {
// //     let parents = graph.neighbors_directed(*curr_ni, Incoming);
// //     let mut parents = parents.peekable(); 
//
// //     let ret_val = parents.peek(); 
// //     parents.next(); 
//
// //     if parents.peek() != None {
// //         panic!("cannot decide which side to go up on as there is more than 1 parents");
// //     }
//     
//
//     
// //     *ret_val.unwrap()
// // }
//
// impl DominatorTree  {
//     pub fn new() -> Self {
//         Self {
//             bb_vec: Vec::new()
//         }
//     }
//
//     pub fn build_dominator_tree(bbl: &BasicBlockList) -> Self {
//         let mut curr_node = bbl.get_current_index(); 
//
//         let mut dtree: DominatorTree = DominatorTree::new();
//
//         //sub routine
//         //add curr to dominator tree
//         // if curr is a join block go up two levels
//         // else go up one level and then add that to dominator tree
//
//
//         // while bbl.bb_graph[curr_node].block_type != BasicBlockType::Entry {
//
//         // }
//
//         while bbl.bb_graph[curr_node].block_type != BasicBlockType::Entry {
//             let mut curr_bb = &bbl.bb_graph[curr_node]; 
//             dtree.bb_vec.push(curr_bb.clone());
//
//             // let mut tmp_node = curr_node; 
//
//             let parents = bbl.bb_graph.neighbors_directed(curr_node, Incoming);
//             curr_node = parents.last().unwrap(); 
//             curr_bb = &bbl.bb_graph[curr_node]; 
//
//             // if (curr_node)
//
//             while curr_bb.block_type == BasicBlockType::Join && curr_bb.block_type != BasicBlockType::Entry{
//                 //go up two
//                 let parents = bbl.bb_graph.neighbors_directed(curr_node, Incoming); 
//             }
//         }
//
//
//         dtree
//         
//
//     }
//
//     // fn push_bb(&mut self, bb: &BasicBlock) {
//     //     self.bb_vec.push(bb);
//     // }
//
//     // fn pop_bb(&mut self) -> Option<&BasicBlock> {
//     //     self.bb_vec.pop()
//     // }
//
//
//     // fn build_dominator_tree(bbl: &BasicBlockList, bb: &BasicBlock) -> Vec<Vec<<bool>> {
//         
//     // }
//
//
//
// }
//
// fn is_dominator(bbl: &BasicBlockList, possible_dominator: &BasicBlock, possibly_dominated: &BasicBlock, start_point: &BasicBlock) -> Result<bool, &'static str> {
//     
//     if possible_dominator == start_point || possibly_dominated == possible_dominator || start_point == possibly_dominated {
//         return Err("Attempting to start at variable to removed or variable to be checked for (logical error)");
//     }
//     match (bbl.get_bb(possible_dominator.id), bbl.get_bb(possibly_dominated.id)) {
//         (Some(_), Some(_)) => 'check_ok:  {
//             break 'check_ok
//         }, 
//         _ =>  return Err("Attempting to access a basic block that is not in the graph")
//     }; 
//
//     let mut bbl = bbl.clone(); 
//
//     bbl.bb_graph.remove_node(possible_dominator.id);
//
//     let non_dom_set = traverse_cfg(&bbl, start_point); 
//
//     // if non dominator set contains the basic block then basicBlock is not dominated, thus the !
//
//     return Ok(!non_dom_set.contains(possibly_dominated));
//
// }
//
// fn traverse_cfg(bbl: &BasicBlockList, start_point: &BasicBlock) -> Vec<BasicBlock> {
//     let bb_vec: Vec<BasicBlock> = Vec::new(); 
//
//     let start_index = start_point.id; 
//
//     let node_index_set =  traverse_graph(&bbl.bb_graph, start_index);
//
//     let mut ret_vec:Vec<BasicBlock> = Vec::new();
//
//     for node in node_index_set {
//         ret_vec.push(bbl.get_bb(node).unwrap().clone())
//     }
//
//     ret_vec
//
// }
// ///BFS through nodes that returns the set of seen nodes
// fn traverse_graph<T>(g: &DiGraph<T, ()>, start_point: NodeIndex) -> HashSet<NodeIndex>{
//
//     
//     let mut seen_nodes = HashSet::new();
//     seen_nodes.insert(start_point);
//
//     let mut curr_queue: VecDeque<NodeIndex> = std::collections::VecDeque::new(); 
//
//     while curr_queue.len() > 0 {
//         let curr_ni = curr_queue.pop_front().unwrap();
//
//         seen_nodes.insert(curr_ni);
//
//         let curr_ni_children = g.neighbors_directed(curr_ni, Outgoing);
//
//         for child in curr_ni_children {
//             curr_queue.push_back(child);
//         }
//     }
//
//     seen_nodes 
//
// }
//
