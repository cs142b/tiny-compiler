use std::collections::VecDeque;

use petgraph::{graph::{Neighbors, NodeIndex}, Direction::Outgoing};

enum ConditionalType {
    IF, 
    LOOP
}

use crate::{basic_block::BasicBlockType, live_analysis::{BasicBlockGraph, Instructions}};
fn traverse_in_order (g: &BasicBlockGraph) -> Instructions {
    let ret_ins = Instructions::new(); 

    let start_node = NodeIndex::<u32>::new(1);

    let mut frontier: VecDeque<NodeIndex> = VecDeque::new(); 
    frontier.push_back(start_node);

    while !frontier.is_empty() {
        let curr_node = frontier.pop_front().unwrap(); 
        let curr_bb = &g[curr_node];    
        match curr_bb.block_type {
            BasicBlockType::Conditional => {
                
            }, 
            _ => {}
        }



    }



    ret_ins
}

/// returns the join block which it stops at 
// fn traverse_in_order_helper_follow_thru_if (start: NodeIndex, v: &mut Instructions, g: &mut BasicBlockGraph) -> NodeIndex{
//     let ni = NodeIndex::<u32>::new(0);
//     //should iterate thru til join block
//     // can never stop at join block that is not its own because of the virtue of the function
//     // it will always restart looking at the join block after its own 
//     ni
// }
/// returns the join block which it stops at 
fn traverse_in_order_helper_fall_thru_if (start: NodeIndex, v: &mut Instructions, g: &mut BasicBlockGraph) -> NodeIndex{
    let ni = NodeIndex::<u32>::new(0);
    //should iterate thru til join block
    // can never stop at join block that is not its own because of the virtue of the function
    // it will always restart looking at the join block after its own 
    ni
}

fn traverse_in_order_follow_thru_while(start: NodeIndex, v: &mut Instructions, g: &mut BasicBlockGraph) -> NodeIndex{
    let ni = NodeIndex::<u32>::new(0);
    //should iterate thru til join block
    // can never stop at join block that is not its own because of the virtue of the function
    // it will always restart looking at the join block after its own 
    ni
}
fn traverse_in_order_helper_branch_or_follow(start: NodeIndex, v: &mut Instructions, g: &mut BasicBlockGraph) -> NodeIndex{
    let ni = NodeIndex::<u32>::new(0);
    //should iterate thru til join block
    // can never stop at join block that is not its own because of the virtue of the function
    // it will always restart looking at the join block after its own 
    ni
}
fn traverse_straight_til_conditional (start: NodeIndex, v: &mut Instructions, g: &mut BasicBlockGraph) -> (Option<NodeIndex>, Option<ConditionalType>) {
    let mut frontier: VecDeque<NodeIndex> = VecDeque::new();
    frontier.push_back(start);

    while !frontier.is_empty() {
        let curr_node = frontier.pop_front().unwrap(); 

        let curr_bb = &mut g[curr_node];
        v.append(&mut curr_bb.instructions); 

        if curr_bb.block_type == BasicBlockType::Conditional {
            if is_if(curr_node, g) {
                traverse_in_order_helper_fall_thru_if(curr_node, v, g);
                let branch_block = select_non_fall_thru(curr_node, g); 
                frontier.push_front(traverse_in_order_helper_branch_or_follow(branch_block.unwrap(), v, g));

            } else {
                traverse_in_order_follow_thru_while(curr_node, v, g);
                let follow_block = select_non_fall_thru(curr_node, g);
                frontier.push_front(traverse_in_order_helper_branch_or_follow(follow_block.unwrap(), v, g));
            }
        } else {
            let children = g.neighbors_directed(curr_node, Outgoing); 
            for child in children {
                frontier.push_back(child);
            }
        }


    }

    (None, None)
}

fn traverse_straight_til_block (start: NodeIndex, v: &mut Instructions, g: &mut BasicBlockGraph, bbtype: BasicBlockType) -> Option<NodeIndex> {
    let mut frontier: VecDeque<NodeIndex> = VecDeque::new();
    frontier.push_back(start);

    while !frontier.is_empty() {
        let curr_node = frontier.pop_front().unwrap(); 

        let curr_bb = &mut g[curr_node];
        v.append(&mut curr_bb.instructions); 

        if curr_bb.block_type == bbtype {

            return Some(curr_node);
            // if is_if(curr_node, g) {
            //     traverse_in_order_helper_fall_thru_if(curr_node, v, g);
            //     let branch_block = select_non_fall_thru(curr_node, g); 
            //     frontier.push_front(traverse_in_order_helper_branch_or_follow(branch_block.unwrap(), v, g));

            // } else {
            //     traverse_in_order_follow_thru_while(curr_node, v, g);
            //     let follow_block = select_non_fall_thru(curr_node, g);
            //     frontier.push_front(traverse_in_order_helper_branch_or_follow(follow_block.unwrap(), v, g));
            // }
        } else {
            let children = g.neighbors_directed(curr_node, Outgoing); 
            for child in children {
                frontier.push_back(child);
            }
        }


        // match curr_bb.block_type {
        //     BasicBlockType::Conditional => {
                
        //     }
        // }
    }

    None
}

// returns the join block that it traverses to
fn traverse_til_join_given_frontier(frontier: &mut VecDeque<NodeIndex>, v: &mut Instructions, g: &mut BasicBlockGraph, ) -> Option<NodeIndex>{

    while !frontier.is_empty() {
        let curr_node = frontier.pop_front().unwrap(); 

        let curr_bb = &mut g[curr_node];
        v.append(&mut curr_bb.instructions); 

        if curr_bb.block_type == BasicBlockType::Join{

            return Some(curr_node)
        } else {
            let children = g.neighbors_directed(curr_node, Outgoing); 
            for child in children {
                frontier.push_back(child);
            }
        }


    } 

    None
}

fn is_if(ni: NodeIndex, g: &BasicBlockGraph) -> bool {
    let children = g.neighbors_directed(ni, Outgoing); 
    for child in children {
        if g[child].block_type == BasicBlockType::Branch {
            return true; 
        }
    }  
    false
}

fn select_non_fall_thru(parent_ni: NodeIndex, g: &BasicBlockGraph) -> Option<NodeIndex>{
    let children = g.neighbors_directed(parent_ni, Outgoing); 
    for child in children {
        if g[child].block_type != BasicBlockType::FallThrough {
            return Some(child); 
        }
    }

    None
}