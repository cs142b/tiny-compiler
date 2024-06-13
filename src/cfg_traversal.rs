use std::collections::{HashSet, VecDeque};

use petgraph::{graph::{Neighbors, NodeIndex}, visit, Direction::Outgoing};

enum ConditionalType {
    IF, 
    LOOP
}

type Visited = HashSet<NodeIndex>;
use crate::{basic_block::BasicBlockType, live_analysis::{BasicBlockGraph, Instructions}};
fn traverse_in_order (g: &mut BasicBlockGraph) -> Instructions {
    let mut ret_ins = Instructions::new(); 

    let start_node = NodeIndex::<u32>::new(0);
    let mut visited_set = Visited::new(); 

    traverse_straight_til_conditional(start_node, &mut ret_ins, g, &mut visited_set);

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
fn traverse_if_branch (start: NodeIndex, v: &mut Instructions, g: &mut BasicBlockGraph, visited: &mut Visited) -> NodeIndex {
    // returns the join block at which point the next function layer can go straight down again

    // println!("Traversing if branch ")
     
    traverse_til_join(start, v, g, visited).unwrap()
}

fn traverse_if_fall_thru(start: NodeIndex, v: &mut Instructions, g: &mut BasicBlockGraph, visited: &mut Visited) {
    traverse_til_join(start, v, g, visited);
}

fn traverse_while_fall_thru(parent: NodeIndex, v: &mut Instructions, g: &mut BasicBlockGraph, visited: &mut Visited) {
    // let mut frontier = VecDeque::<NodeIndex<u32>>::new(); 
    println!("Inside traverse while fall thru for node {:?} with type {:?}", parent, g[parent].block_type);
    
    let mut start: Option<NodeIndex> = None;

    for child in g.neighbors_directed(parent, Outgoing) {
        if g[child].block_type == BasicBlockType::FallThrough {
            start = Some(child);
            println!("Start =  node : {:?} with type {:?}", start.unwrap(), g[start.unwrap()].block_type);
        }
    }
    

    let curr_bb = &mut g[start.unwrap()];
    if !curr_bb.instructions.is_empty() {

        v.append(&mut curr_bb.instructions); 
    }

    let children = g.neighbors_directed(start.unwrap(), Outgoing); 
    println!();
    let mut other_child: Option<NodeIndex> = None; 
    for child in children {
        if child != parent {

            println!("Child = {:?} with type = {:?}", child, g[child].block_type);
            other_child = Some(child);
            break; 
        }
    }
    if other_child != None {
        traverse_straight_til_conditional(other_child.unwrap(), v, g, visited);
    }
    
}
fn traverse_straight_til_conditional (start: NodeIndex, v: &mut Instructions, g: &mut BasicBlockGraph, visited: &mut Visited) -> (Option<NodeIndex>, Option<ConditionalType>) {
    let mut frontier: VecDeque<NodeIndex> = VecDeque::new();
    frontier.push_back(start);

    while !frontier.is_empty() {
        let curr_node = frontier.pop_front().unwrap(); 
        
        

        let curr_bb = &mut g[curr_node];
        if !curr_bb.instructions.is_empty() {

            v.append(&mut curr_bb.instructions); 
        }

        if curr_bb.block_type == BasicBlockType::Conditional {
            if is_if(curr_node, g) {
                traverse_if_fall_thru(curr_node, v, g, visited);
                let branch_block = select_non_fall_thru(curr_node, g); 
                frontier.push_front(traverse_if_branch(branch_block.unwrap(), v, g, visited));

            } else {
                if visited.contains(&curr_node) {
                    return (None, None);
                } else {
                    visited.insert(curr_node);
                }
                // traverse_in_order_follow_thru_while(curr_node, v, g);
                traverse_while_fall_thru(curr_node, v, g, visited);
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
fn traverse_til_join(start: NodeIndex, v: &mut Instructions, g: &mut BasicBlockGraph, visited: &mut Visited) -> Option<NodeIndex>{
    let mut frontier: VecDeque<NodeIndex> = VecDeque::new(); 
    frontier.push_back(start);
    while !frontier.is_empty() {
        let curr_node = frontier.pop_front().unwrap(); 
        // if visited.contains(&curr_node) {
        //     return None; 
        // } else {
        //     visited.insert(curr_node);
        // }
        visited.insert(curr_node);

        let curr_bb = &mut g[curr_node];
        if !curr_bb.instructions.is_empty() {

            v.append(&mut curr_bb.instructions); 
        }
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
    let children: Neighbors<BasicBlockType, u32> = g.neighbors_directed(parent_ni, Outgoing); 
    for child in children {
        if g[child].block_type != BasicBlockType::FallThrough {
            return Some(child); 
        }
    }

    None
}

fn select_block_with_type(neighbors: &Neighbors<BasicBlockType, u32>, g: &BasicBlockGraph, bb_type: BasicBlockType) -> Option<NodeIndex> {
    for neighbor in neighbors.clone() {
        if g[neighbor].block_type == bb_type {
            return Some(neighbor)
        }
    }

    None
}

fn select_block_not_with_type(neighbors: &Neighbors<BasicBlockType, u32>, g: &BasicBlockGraph, bb_type: BasicBlockType) -> Option<NodeIndex> {
    for neighbor in neighbors.clone() {
        if g[neighbor].block_type != bb_type {
            return Some(neighbor)
        }
    }

    None 
}
#[cfg(test)]
mod cfg_traversal_tests {

    // use serial_test::serial;

    

    use super::*; 
    use crate::parser::Parser;
    use crate::dot_viz::generate_dot_viz;

    fn get_num_instructions_in_graph(g: &BasicBlockGraph) -> usize {
        let mut num_instructions:usize = 0 ; 
        for i in g.node_indices() {
            let curr_bb = &g[i];
            let vec = &curr_bb.instructions;
            num_instructions += vec.len(); 

        }

        num_instructions


    }

    #[test]
    fn works_basic_fn() {
        let input = "main var x; { if 1 < 2 then let x <- 2; else let x <- 1; fi; }.".to_string();
        let mut parser = Parser::new(input);

        parser.parse_computation();

        // Verify that the if statement creates the correct basic blocks and instructions
        let graph = &parser.internal_program.get_curr_fn().bb_graph;
        let number_of_blocks = graph.node_count();

        

        // println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));
        println!("{}", generate_dot_viz("main", &parser.internal_program));

        assert_eq!(number_of_blocks, 6); // should be 5 bc entry + conditional + fallthru + branch
        // + join

        let x = traverse_in_order(&mut graph.clone());
        for j in &x {
            println!("{:?}", *j);
        }

        assert_eq!(x.len(), get_num_instructions_in_graph(&graph));


    
    }

    #[test]
    fn test_parse_while_statement() {
        let input = "main var x; { while 10 >= 6 do while 1 < 2 do let x <- 2; od; od }.".to_string();
        let mut parser = Parser::new(input);

        parser.parse_computation();

        // Verify that the if statement creates the correct basic blocks and instructions
        let graph = &parser.internal_program.get_curr_fn().bb_graph;

        // println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));
        println!("{}", generate_dot_viz("main", &parser.internal_program));
        let x = traverse_in_order(&mut graph.clone());
        for j in &x {
            println!("{:?}", *j);
        }

        assert_eq!(x.len(), get_num_instructions_in_graph(&graph));
    }
    
}