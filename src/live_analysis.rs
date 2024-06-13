use crate::basic_block::{BasicBlock, BasicBlockType};
use crate::instruction::Operation;
use core::panic;
use petgraph::data::Build;
use petgraph::graph::{DiGraph, UnGraph};
use petgraph::graph::{Node, NodeIndex};
use petgraph::Direction::{Incoming, Outgoing};
use std::collections::{HashMap, HashSet, VecDeque};

type LiveSet = HashSet<isize>;
type LineNumber = isize;
type InterferenceGraph = UnGraph<LineNumber, ()>;
type BasicBlockGraph = DiGraph<BasicBlock, BasicBlockType>;
type LineNumSet = HashSet<LineNumber>;
type UsgCnt = usize;

#[derive(Default)]
pub struct BlockInfo {
    pub use_set: LiveSet,
    pub def_set: LiveSet,
    pub in_set: LiveSet,
    pub out_set: LiveSet,
}

pub fn compute_live_sets(g: &BasicBlockGraph) -> HashMap<NodeIndex, BlockInfo> {
    let mut block_info = HashMap::new();

    // Initialize use and def sets
    for node_index in g.node_indices() {
        let block = &g[node_index];
        let mut info = BlockInfo::default();

        for instruction in &block.instructions {
            // Determine the use and def sets for each instruction
            match instruction.get_operation_ref() {
                Operation::Phi(l, r) | Operation::Cmp(l, r) => {
                    info.use_set.insert(*l);
                    info.use_set.insert(*r);
                }
                Operation::Add(l, r)
                | Operation::Mul(l, r)
                | Operation::Div(l, r)
                | Operation::Sub(l, r) => {
                    info.use_set.insert(*l);
                    info.use_set.insert(*r);
                }
                Operation::Write(l)
                | Operation::Ret(l)
                | Operation::SetPar1(l)
                | Operation::SetPar2(l)
                | Operation::SetPar3(l) => {
                    info.use_set.insert(*l);
                }
                _ => {}
            }
            // Collect defs (simplified for this example, adjust as needed)
            if let Some(def) = instruction.get_def() {
                info.def_set.insert(def);
            }
        }

        block_info.insert(node_index, info);
    }

    let mut changed = true;
    while changed {
        changed = false;

        for node_index in g.node_indices().rev() {
            // Compute OUT set
            let mut new_out_set = LiveSet::new();
            let successors: Vec<NodeIndex> = g.neighbors_directed(node_index, Outgoing).collect();
            for successor in &successors {
                let successor_info = &block_info[&successor];
                new_out_set.extend(&successor_info.in_set);
            }

            // Compute IN set
            let mut new_in_set = block_info[&node_index].use_set.clone();
            for &var in &new_out_set {
                if !block_info[&node_index].def_set.contains(&var) {
                    new_in_set.insert(var);
                }
            }

            // Check for changes
            let info = block_info.get_mut(&node_index).unwrap();
            if new_out_set != info.out_set || new_in_set != info.in_set {
                info.out_set = new_out_set;
                info.in_set = new_in_set;
                changed = true;
            }
        }
    }

    for (b, binfo) in &block_info {
        for ins in &binfo.use_set {
            println!("{:?}", ins);
        }


        println!("Next block");
    }


    block_info
}

pub fn get_interference_graph(g: &BasicBlockGraph) -> InterferenceGraph {
    let block_info_map = compute_live_sets(g);

    let all_var_set = collect_all_variables(&block_info_map);

    let mut ig: InterferenceGraph = InterferenceGraph::new_undirected();

    let line_to_nodeidx_map = alter_ig_from_all_vars(&mut ig, &all_var_set);

    for binfo in block_info_map.values() {
        // draw edges between variables in the outset and make sure that those variables are connected in the live set
        create_set_edge_additions(
            &mut ig,
            &binfo.out_set,
            &binfo.out_set,
            &line_to_nodeidx_map,
        );

        // draw edges between variables in the inset and make sure that those variables are connected in the live set
        create_set_edge_additions(&mut ig, &binfo.in_set, &binfo.in_set, &line_to_nodeidx_map);

        create_set_edge_additions(&mut ig, &binfo.def_set, &binfo.in_set, &line_to_nodeidx_map);
        create_set_edge_additions(&mut ig, &binfo.use_set, &binfo.use_set, &line_to_nodeidx_map);
    }

    ig
}

fn collect_all_variables(map: &HashMap<NodeIndex, BlockInfo>) -> LineNumSet {
    let mut inst_num_set = LineNumSet::new();
    // let mut ig = InterferenceGraph::new_undirected();
    for (ni, bi) in map {
        for def_num in &bi.def_set {
            inst_num_set.insert(*def_num);
        }
        for use_num in &bi.use_set {
            inst_num_set.insert(*use_num);
        }
        for in_num in &bi.in_set {
            inst_num_set.insert(*in_num);
        }
        for out_num in &bi.out_set {
            inst_num_set.insert(*out_num);
        }
    }

    inst_num_set
}

fn alter_ig_from_all_vars(
    ig: &mut InterferenceGraph,
    all_vars_set: &LineNumSet,
) -> HashMap<LineNumber, NodeIndex> {
    let mut line_num_to_node_idx = HashMap::<LineNumber, NodeIndex>::new();

    for line_num in all_vars_set {
        let node = ig.add_node(*line_num);
        line_num_to_node_idx.insert(*line_num, node);
    }

    line_num_to_node_idx
}

fn create_set_edge_additions(
    ig: &mut InterferenceGraph,
    set1: &LineNumSet,
    set2: &LineNumSet,
    line_nodeidx_map: &HashMap<LineNumber, NodeIndex>,
) {
    for line_num1 in set1 {
        for line_num2 in set2 {
            let ni1 = line_nodeidx_map.get(line_num1);
            let ni2 = line_nodeidx_map.get(line_num2);
            if ni1 == None || ni2 == None {
                panic!("Can't actually add node as it doesnt exist in the interference graph");
            }

            let ni1 = ni1.unwrap();
            let ni2 = ni2.unwrap();
            if line_num1 != line_num2 && !ig.contains_edge(*ni1, *ni2) {
                ig.add_edge(*ni1, *ni2, ());
            }
        }
    }
}

/// bfs through use counts and get the requisite use counts of each line number this will help for choosing the variables
/// to put into a register if some need to get pushed out
fn get_use_counts(
    g: &mut BasicBlockGraph,
    live_sets: HashMap<NodeIndex, BlockInfo>,
) -> HashMap<LineNumber, UsgCnt> {
    let mut res = HashMap::<LineNumber, UsgCnt>::new();

    let curr_level: usize = 0;

    // let curr_node = NodeIndex::new();

    // let curr_bb = &g[NodeIndex::new(g.node_count() - 1)];
    let start: &NodeIndex<u32> = &NodeIndex::new(g.node_count() - 1);

    let mut frontier: VecDeque<NodeIndex> = VecDeque::new();

    frontier.push_back(*start);

    while frontier.is_empty() == false {
        let curr_el = frontier.pop_front();
        if curr_el == None {
            panic!("WTF");
        }

        let curr_el = curr_el.unwrap();

        let parents = g.neighbors_directed(curr_el, Incoming);
        for parent in parents {
            frontier.push_back(parent);
        }

        let bb_mut = &mut g[curr_el];
        for instruction in &mut bb_mut.instructions.iter().rev() {
            let relevant_lines = instruction.get_operation_ref().get_lines();
            for relevant_line in relevant_lines {
                let curr = res.get_mut(&relevant_line);
                if curr != None {
                    *curr.unwrap() += 1;
                } else {
                    res.insert(relevant_line, 1);
                }
            }
        }
    }

    res
}

type Layer = usize;
fn mark_graph_with_layers(g: &BasicBlockGraph) -> HashMap<NodeIndex, Layer> {
    let mut ret_map = HashMap::<NodeIndex, Layer>::new();
    let mut curr_layer: Layer = 0;

    let start: &NodeIndex<u32> = &NodeIndex::new(g.node_count() - 1);

    let mut frontier: VecDeque<NodeIndex> = VecDeque::new();

    frontier.push_back(*start);

    while frontier.is_empty() == false {
        let curr_el = frontier.pop_front();
        if curr_el == None {
            panic!("WTF");
        }

        let curr_el = curr_el.unwrap();

        let parents = g.neighbors_directed(curr_el, Incoming);
        for parent in parents {
            frontier.push_back(parent);
        }
    }

    ret_map
}

#[cfg(test)]
mod live_anal_tests {
    use super::*;
    use crate::dot_viz::generate_dot_viz;
    use crate::parser::Parser;
    use petgraph::dot::{Config, Dot};
    #[test]
    pub fn test_parse_computation() {
        let input = "
            main var a, b, c; {
                let a <- 1 + 50;  
                let a <- 1 + 50; 
                if 1 < 2 then 
                    let c <- 1 + 50; 
                fi;
            }.
        "
        .to_string();

        let mut parser = Parser::new(input);

        let line_number = parser.parse_computation();

        // Verify that the add operation is correct
        let instructions = &parser.internal_program.get_curr_block().instructions;

        let graph = &parser.internal_program.get_curr_fn().bb_graph;
        println!("{}", generate_dot_viz("main", &parser.internal_program));

        let graph = get_interference_graph(&parser.internal_program.get_curr_fn().bb_graph);
        println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));
    }
}
