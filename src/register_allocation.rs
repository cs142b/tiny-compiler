use petgraph::graph::NodeIndex;
use std::collections::HashMap;
use petgraph::graph::{Graph, UnGraph};
use petgraph::Undirected;

type Color = usize;

pub fn color_graph <N, E> (interference_graph: &UnGraph<N, E>) -> HashMap<NodeIndex, Color> {
    let mut color_mapping = HashMap::<NodeIndex, Color>::new();
    let max_colors = 5; // max according to doc

    for node in interference_graph.node_indices() {
        let mut avaliable_colors: Vec<Color> = (1..=max_colors).collect();

        for node_neighbor in interference_graph.neighbors_undirected(node) {
            if let Some(color_num) = color_mapping.get(&node_neighbor) {
                // get the index of the register number I want to remove
                if let Some(index) = avaliable_colors.iter().position(|&r| r == *color_num) {
                    avaliable_colors.remove(index);
                }
            }
        }

        let node_color = *avaliable_colors.iter().next().unwrap();
        color_mapping.insert(node, node_color);
    }

    color_mapping
}

#[cfg(test)]
mod graph_test {
    use super::*;
    #[test]
    pub fn first() {
        let mut graph = Graph::<isize, (), Undirected>::new_undirected();
        let a = graph.add_node(1);
        let b = graph.add_node(2);
        let c = graph.add_node(3);
        let d = graph.add_node(4);

        graph.add_edge(a, b, ());
        graph.add_edge(b, c, ());
        graph.add_edge(a, c, ());
        graph.add_edge(a, d, ());

        let color_mapping = color_graph(&graph);

        for (key, value) in &color_mapping {
            println!("{:?} maps to Color({})", key, value);
        }

    }
}
