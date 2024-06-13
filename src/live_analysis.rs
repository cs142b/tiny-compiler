use crate::basic_block::{BasicBlock, BasicBlockType};
use crate::instruction::{self, Instruction, Operation};
use petgraph::data::Build;
use petgraph::graph::{DiGraph, UnGraph};
use petgraph::graph::{Node, NodeIndex};
use petgraph::Direction::{Incoming, Outgoing};
use std::collections::{HashMap, HashSet, VecDeque};

type LiveSet = HashSet<isize>;
type LineNumber = isize;
type LineNumbers = Vec<LineNumber>;
type Cluster = LineNumbers;
type Clusters = Vec<Cluster>;
pub type InterferenceGraph = UnGraph<LineNumber, ()>;
pub type BasicBlockGraph = DiGraph<BasicBlock, BasicBlockType>;
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

        info.def_set = get_def_set(&block, g);

        for instruction in &block.instructions {
            // Determine the use and def sets for each instruction
            match instruction.get_operation_ref() {
                Operation::Phi(l, r)
                | Operation::Cmp(l, r)
                | Operation::Add(l, r)
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
            // if let Some(def) = instruction.get_def() {
            //     info.def_set.insert(def);
            // }
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
        println!("{:?}", b);
        println!("def set: ");
        for ins in &binfo.def_set {
            println!("{:?}", ins);
        }
        println!("use set: ");
        for ins in &binfo.use_set {
            println!("{:?}", ins);
        }
        println!("in set: ");
        for ins in &binfo.in_set {
            println!("{:?}", ins);
        }

        println!("out set: ");
        for ins in &binfo.out_set {
            println!("{:?}", ins);
        }
    }
    // for (b, binfo) in &block_info {
    //     println!("{:?}", b);
    //     println!("def set: ");
    //     for ins in &binfo.def_set {
    //         println!("{:?}", ins);
    //     }
    //         println!("use set: ");
    //     for ins in &binfo.use_set {
    //         println!("{:?}", ins);
    //     }
    //         println!("in set: ");
    //     for ins in &binfo.in_set {
    //         println!("{:?}", ins);
    //
    //     }
    //
    //         println!("out set: ");
    //     for ins in &binfo.out_set {
    //         println!("{:?}", ins);
    //     }
    //
    // }

    block_info
}
pub type Instructions = Vec<Instruction>;
fn dead_code_elimination(v: &mut Instructions, inherited_set: &mut LiveSet) {
    // in this case, the set to eliminate should be the out set and given code elimination block by block
    // this function should work
    let v_clone = v.clone();
    for (idx, instruction) in v_clone.iter().rev().enumerate() {
        match instruction.get_operation_ref() {
            Operation::Phi(_, _)
            | Operation::Add(_, _)
            | Operation::Mul(_, _)
            | Operation::Sub(_, _)
            | Operation::Div(_, _)
            | Operation::Read
            | Operation::GetPar1
            | Operation::GetPar2
            | Operation::GetPar3
            | Operation::SetPar1(_)
            | Operation::SetPar2(_)
            | Operation::SetPar3(_) => {
                if !inherited_set.contains(&instruction.get_line_number()) {
                    v.remove(v.len() - idx - 1);
                }
            }

            Operation::Cmp(l, r) => {
                inherited_set.insert(l.clone());
                inherited_set.insert(r.clone());
            }
            Operation::Write(l) | Operation::Ret(l) => {
                inherited_set.insert(l.clone());
            }
            _ => {}
        }
    }
}

fn get_def_set(b: &BasicBlock, g: &BasicBlockGraph) -> LineNumSet {
    let mut def_set = LineNumSet::new();
    let parents: Vec<NodeIndex> = g.neighbors_directed(b.id, Incoming).collect();
    for (var_name, var_type) in &b.variable_table {
        for parent in &parents {
            let par_var_table = &g[*parent].variable_table;

            let par_var = par_var_table.get(var_name);
            if par_var != None && par_var.unwrap() == var_type {
                def_set.insert(var_type.get_value());
            }
        }
    }

    for ins in &b.instructions {
        match ins.operation {
            Operation::Cmp(_, _) => {
                def_set.insert(ins.get_line_number());
            }
            _ => {}
        }
    }
    def_set
}

pub fn get_interference_graph(g: &BasicBlockGraph) -> InterferenceGraph {
    let block_info_map = compute_live_sets(g);

    let all_var_set = collect_all_variables(&block_info_map);

    let mut ig: InterferenceGraph = InterferenceGraph::new_undirected();

    let line_to_nodeidx_map = all_vars_node_defined(&mut ig, &all_var_set);

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

        // create_set_edge_additions(&mut ig, &binfo.def_set, &binfo.in_set, &line_to_nodeidx_map);
        // create_set_edge_additions(
        //     &mut ig,
        //     &binfo.use_set,
        //     &binfo.use_set,
        //     &line_to_nodeidx_map,
        // );
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

fn all_vars_node_defined(
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
fn graph_edge_additions<N>(
    g: &mut UnGraph<N, ()>,
    edges_to_add1: &Vec<LineNumber>,
    edges_to_add2: &Vec<LineNumber>,
    generic_to_nodeindex: &mut HashMap<LineNumber, NodeIndex>,
) {
    for node1 in edges_to_add1 {
        for node2 in edges_to_add2 {
            if node1 != node2 {
                let nodeidx1 = generic_to_nodeindex.get(node1);
                let nodeidx2 = generic_to_nodeindex.get(node2);
                g.add_edge(*nodeidx1.unwrap(), *nodeidx2.unwrap(), ());
            }
        }
    }
}

pub fn get_clusters(g: &BasicBlockGraph) -> Clusters {
    let mut clusters = Clusters::new();

    for ni in g.node_indices().into_iter().rev() {
        let bb_vec = &g[ni].instructions;

        for instruction in bb_vec.iter().rev() {
            match *instruction.get_operation_ref() {
                Operation::Phi(l, r) => {
                    let mut new_cluster = Cluster::new();
                    new_cluster.push(l);
                    new_cluster.push(r);
                    new_cluster.push(instruction.get_line_number());

                    clusters.push(new_cluster);
                }
                _ => {}
            }
        }
    }
    clusters
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



pub fn get_graph_and_map (g: &InterferenceGraph, cluster_possibilities: &Clusters) -> (UpgradedInterferenceGraph, HashMap<LineNumber, NodeIndex>) {
    let mut remapped = LineNumSet::new(); 
    let (mut upgraded_ig, mut line_to_node_idx) = convert_ig_to_upgraded(g);

    for cluster in cluster_possibilities {
        for (idx, line_num) in cluster.iter().enumerate() {
            let mut was_remapped: bool = false; 
            // this line not yet remapped
            for (idx2, line_num2) in cluster.iter().enumerate() {
                if line_num != line_num2 && !remapped.contains(line_num) && !remapped.contains(line_num2) {
                    was_remapped = true; 
                    // remapped.insert(*line_num);
                    remapped.insert(*line_num2);
                    let ug = upgraded_ig.clone();
                    let removed_neighbors = ug.neighbors_undirected(*line_to_node_idx.get(line_num2).unwrap());

                    for removed_neighbor in removed_neighbors {
                        let curr_node = line_to_node_idx.get(line_num).unwrap();
                        if !upgraded_ig.contains_edge(*curr_node, removed_neighbor) {
                            upgraded_ig.add_edge(*curr_node, removed_neighbor, ());
                        }
                    }
                    
                    
                    upgraded_ig.remove_node(*line_to_node_idx.get(line_num2).unwrap());
                    line_to_node_idx.remove(line_num2);
                    let curr_saved_node: NodeIndex = *line_to_node_idx.get(line_num).unwrap(); 
                    line_to_node_idx.insert(*line_num2, curr_saved_node);

                    println!("Curr saved node = {:?}", curr_saved_node);


                    let cluster_to_change = &mut upgraded_ig[curr_saved_node];
                    cluster_to_change.push(*line_num2);

                }
            }
            if was_remapped == true {
                remapped.insert(*line_num);
            }
        }
    }
    (upgraded_ig, line_to_node_idx)
}

pub type UpgradedInterferenceGraph = UnGraph<Cluster, ()>;

// pub fn get_upgraded_interference_graph(
//     g: &InterferenceGraph,
//     cluster_possibilities: &Clusters,
// ) -> UpgradedInterferenceGraph {
//     let mut remapped = LineNumSet::new();
// }

   
pub fn get_upgraded_interference_graph (g: &InterferenceGraph, cluster_possibilities: &Clusters) -> UpgradedInterferenceGraph {
    let mut remapped = LineNumSet::new(); 
    let (mut upgraded_ig, mut line_to_node_idx) = convert_ig_to_upgraded(g);

    for cluster in cluster_possibilities {
        for (idx, line_num) in cluster.iter().enumerate() {
            // this line not yet remapped
            for (idx2, line_num2) in cluster.iter().enumerate() {
                if line_num != line_num2
                    && !remapped.contains(line_num)
                    && !remapped.contains(line_num2)
                {
                    remapped.insert(*line_num);
                    remapped.insert(*line_num2);
                    let removed_neighbors =
                        upgraded_ig.neighbors_undirected(*line_to_node_idx.get(line_num2).unwrap());

                    upgraded_ig.remove_node(*line_to_node_idx.get(line_num2).unwrap());
                    line_to_node_idx.remove(line_num2);
                    let curr_saved_node: NodeIndex = *line_to_node_idx.get(line_num).unwrap();
                    line_to_node_idx.insert(*line_num2, curr_saved_node);

                    let cluster_to_change = &mut upgraded_ig[curr_saved_node];
                    cluster_to_change.push(*line_num2);
                }
            }
        }
    }

    // for cluster in cluster_possibilities {

    // }

    upgraded_ig
}

fn convert_ig_to_upgraded(
    g: &InterferenceGraph,
) -> (UpgradedInterferenceGraph, HashMap<LineNumber, NodeIndex>) {
    
    let mut upgraded_ig: petgraph::Graph<Vec<isize>, (), petgraph::Undirected> =
        UpgradedInterferenceGraph::new_undirected();
    let mut line_to_nodeidx: HashMap<isize, NodeIndex> = HashMap::<LineNumber, NodeIndex>::new();

    let mut old_to_new_map: HashMap<NodeIndex, NodeIndex> = HashMap::<NodeIndex, NodeIndex>::new();

    for node in g.node_indices() {
        let mut new_cluster = Cluster::new();
        let curr_num = g[node];
        new_cluster.push(curr_num);

        let new_node = upgraded_ig.add_node(new_cluster);
        line_to_nodeidx.insert(curr_num, new_node);

        old_to_new_map.insert(node, new_node);
    }


    for old in g.node_indices() {
        let new_node = old_to_new_map.get(&old).unwrap();
        let old_neighbors = g.neighbors_undirected(old);
        for old_neighbor in old_neighbors {
            let new_neighbor = old_to_new_map.get(&old_neighbor).unwrap();
            if !upgraded_ig.contains_edge(*new_node, *new_neighbor) {
                upgraded_ig.add_edge(*new_node, *new_neighbor, ());
            }
        }
    }

    (upgraded_ig, line_to_nodeidx)
}

fn mark_graph_with_layers_helper(
    g: &BasicBlockGraph,
    start_node: NodeIndex,
    layers_map: &mut HashMap<NodeIndex, Layer>,
    curr_layer: &mut Layer,
    lower_layer_idx: Option<NodeIndex>,
) {
    let mut frontier: VecDeque<NodeIndex> = VecDeque::new();
    frontier.push_back(start_node);

    while !frontier.is_empty() {
        let curr_ni = frontier.pop_front();
        let curr_ni = curr_ni.unwrap();
        let curr_bb_type = &g[curr_ni].block_type;

        layers_map.insert(curr_ni, *curr_layer);

        match curr_bb_type {
            BasicBlockType::Follow => {
                // go plus and set the starting point to the end
                let assoc_conditional = *g
                    .neighbors_directed(curr_ni, Incoming)
                    .peekable()
                    .peek()
                    .unwrap();

                *curr_layer += 1;

                // recursive solution here abuses the fuck out of the fact that a graph is also a one way tree in this case as we
                // are isolating for the actual loop blocks
                mark_graph_with_layers_helper(
                    g,
                    start_node,
                    layers_map,
                    curr_layer,
                    Some(assoc_conditional),
                );
            }
            _ => {}
        }

        if lower_layer_idx != None && lower_layer_idx.unwrap() == curr_ni {
            *curr_layer -= 1;
            // just continue as normal going upwards here no reason to change anything.
        }
    }
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

                let c <- a + 2;
            }.
        "
        .to_string();

        let mut parser = Parser::new(input);

        let line_number = parser.parse_computation();

        // Verify that the add operation is correct
        let instructions = &parser.internal_program.get_curr_block().instructions;

        let bbg = &parser.internal_program.get_curr_fn().bb_graph;
        println!("{}", generate_dot_viz("main", &parser.internal_program));

        let graph = get_interference_graph(&parser.internal_program.get_curr_fn().bb_graph);
        println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));

        let cluster_possibilities = get_clusters(&bbg);
        let upgraded_ig = get_upgraded_interference_graph(&graph, &cluster_possibilities);

        println!(
            "{:?}",
            Dot::with_config(&upgraded_ig, &[Config::EdgeNoLabel])
        );
    }
}
