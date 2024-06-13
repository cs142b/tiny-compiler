use std::collections::VecDeque;

use petgraph::{graph::{Neighbors, NodeIndex}, Direction::Outgoing};

enum ConditionalType {
    IF, 
    LOOP
}

use crate::{basic_block::BasicBlockType, live_analysis::{BasicBlockGraph, Instructions}};
fn traverse_in_order (g: &mut BasicBlockGraph) -> Instructions {
    let mut ret_ins = Instructions::new(); 

    let start_node = NodeIndex::<u32>::new(0);

    traverse_straight_til_conditional(start_node, &mut ret_ins, g);

    // let mut frontier: VecDeque<NodeIndex> = VecDeque::new(); 
    // frontier.push_back(start_node);

    // while !frontier.is_empty() {
    //     let curr_node = frontier.pop_front().unwrap(); 
    //     let curr_bb = &g[curr_node];    
    //     match curr_bb.block_type {
    //         BasicBlockType::Conditional => {
                
    //         }, 
    //         _ => {}
    //     }



    // }





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
// fn traverse_while_follow(start: NodeIndex, v: &mut Instructions, g: &mut BasicBlockGraph) -> NodeIndex{
//     let ni = NodeIndex::<u32>::new(0);

//     // this should be given to start the follow block of the while loop 
//     //should iterate thru til join block
//     // can never stop at join block that is not its own because of the virtue of the function
//     // it will always restart looking at the join block after its own 
//     // this will stop at its own join block and then return that block

//     let mut frontier : VecDeque<NodeIndex> = VecDeque::new(); 

//     let children = g.neighbors_directed(start, Outgoing);
//     for child in children {
//         let child_bb
//     }

//     traverse_til_join_given_frontier(&mut frontier, v, g);
//     ni
// }

fn traverse_if_branch (start: NodeIndex, v: &mut Instructions, g: &mut BasicBlockGraph) -> NodeIndex {
    // returns the join block at which point the next function layer can go straight down again
     
    traverse_til_join(start, v, g).unwrap()
}

fn traverse_if_fall_thru(start: NodeIndex, v: &mut Instructions, g: &mut BasicBlockGraph) {
    traverse_til_join(start, v, g);
}

fn traverse_while_follow(parent: NodeIndex, v: &mut Instructions, g: &mut BasicBlockGraph) {
    // let mut frontier = VecDeque::<NodeIndex<u32>>::new(); 
    let mut start: Option<NodeIndex> = None;
    for child in g.neighbors_directed(parent, Outgoing) {
        if g[child].block_type == BasicBlockType::FallThrough {
            start = Some(child);
        }
    }

    traverse_straight_til_conditional(start.unwrap(), v, g);
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
                traverse_if_fall_thru(curr_node, v, g);
                let branch_block = select_non_fall_thru(curr_node, g); 
                frontier.push_front(traverse_if_branch(branch_block.unwrap(), v, g));

            } else {

                // traverse_in_order_follow_thru_while(curr_node, v, g);
                traverse_while_follow(curr_node, v, g);
                let follow_block = select_non_fall_thru(curr_node, g);
                frontier.push_front(follow_block.unwrap());
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

// returns the join block that it traverses to
fn traverse_til_join(start: NodeIndex, v: &mut Instructions, g: &mut BasicBlockGraph, ) -> Option<NodeIndex>{
    let mut frontier: VecDeque<NodeIndex> = VecDeque::new(); 
    frontier.push_back(start);
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