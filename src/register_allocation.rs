use petgraph::graph::NodeIndex;
use std::collections::HashMap;
use petgraph::graph::{Graph, UnGraph};
use petgraph::Undirected;

type LineNumber = isize;
type RegisterNumber = usize;
type InterferenceGraph = UnGraph<LineNumber, ()>;

pub fn color_graph(interference_graph: &InterferenceGraph) -> HashMap<LineNumber, RegisterNumber> {
    let mut register_mapping = HashMap::<LineNumber, RegisterNumber>::new();
    let max_registers = 5; // max according to doc

    // node's weight is a line number
    for node in interference_graph.node_indices() {
        let mut avaliable_registers: Vec<usize> = (1..=max_registers).collect();

        for node_neighbor in interference_graph.neighbors_undirected(node) {
            let neighbor_node_weight = interference_graph.node_weight(node_neighbor).unwrap();
            if let Some(register_num) = register_mapping.get(neighbor_node_weight) {
                // get the index of the register number I want to remove
                if let Some(index) = avaliable_registers.iter().position(|&r| r == *register_num) {
                    avaliable_registers.remove(index);
                }
            }
        }

        let node_weight = *interference_graph.node_weight(node).unwrap();
        let node_register = *avaliable_registers.iter().next().unwrap();
        register_mapping.insert(node_weight, node_register);
    }

    register_mapping
}

#[cfg(test)]
mod graph_test {
    use super::*;
    #[test]
    pub fn first() {
        let mut graph = Graph::<LineNumber, (), Undirected>::new_undirected();
        let a = graph.add_node(1);
        let b = graph.add_node(2);
        let c = graph.add_node(3);
        let d = graph.add_node(4);

        graph.add_edge(a, b, ());
        graph.add_edge(b, c, ());
        graph.add_edge(a, c, ());
        graph.add_edge(a, d, ());

        let register_mapping = color_graph(&graph);

        for (key, value) in &register_mapping {
            println!("Line({}) maps to R{}", key, value);
        }

    }
}
