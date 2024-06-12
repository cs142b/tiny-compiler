use petgraph::graph::NodeIndex;
use std::collections::{HashMap, HashSet};
use petgraph::graph::DiGraph;
use crate::basic_block::BasicBlock;
use crate::instruction::Operation;

type LiveSet = HashSet<isize>;

#[derive(Default)]
struct BlockInfo {
    use_set: LiveSet,
    def_set: LiveSet,
    in_set: LiveSet,
    out_set: LiveSet,
}

pub fn compute_live_sets(g: &DiGraph<BasicBlock, ()>) -> HashMap<NodeIndex, BlockInfo> {
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
                | Operation::Const(l)
                | Operation::GetPar1(l)
                | Operation::GetPar2(l)
                | Operation::GetPar3(l)
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
            let mut info = block_info.get_mut(&node_index).unwrap();

            // Compute OUT set
            let mut new_out_set = LiveSet::new();
            for successor in g.neighbors_directed(node_index, Outgoing) {
                let successor_info = &block_info[&successor];
                new_out_set.extend(&successor_info.in_set);
            }

            // Compute IN set
            let mut new_in_set = info.use_set.clone();
            for &var in &new_out_set {
                if !info.def_set.contains(&var) {
                    new_in_set.insert(var);
                }
            }

            // Check for changes
            if new_out_set != info.out_set || new_in_set != info.in_set {
                info.out_set = new_out_set;
                info.in_set = new_in_set;
                changed = true;
            }
        }
    }

    block_info
}