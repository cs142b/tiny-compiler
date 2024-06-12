use petgraph::graph::NodeIndex;
use petgraph::Direction::Outgoing;
use std::collections::{HashMap, HashSet};
use petgraph::graph::DiGraph;
use crate::basic_block::{BasicBlock, BasicBlockType};
use crate::instruction::Operation;

type LiveSet = HashSet<isize>;

#[derive(Default)]
pub struct BlockInfo {
    pub use_set: LiveSet,
    pub def_set: LiveSet,
    pub in_set: LiveSet,
    pub out_set: LiveSet,
}

pub fn compute_live_sets(g: &DiGraph<BasicBlock, BasicBlockType>) -> HashMap<NodeIndex, BlockInfo> {
    let mut block_info = HashMap::new();

    // Initialize use and def sets
    for node_index in g.node_indices() {
        let block = &g[node_index];
        let mut info = BlockInfo::default();

        for instruction in &block.instructions {
            // Determine the use and def sets for each instruction
            match instruction.get_operation_ref() {
                Operation::Phi(l, r) => {
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

    block_info
}