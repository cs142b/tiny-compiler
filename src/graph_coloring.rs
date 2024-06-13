use petgraph::graph::NodeIndex;
use std::collections::HashMap;
use petgraph::graph::{Graph, UnGraph};
use petgraph::Undirected;
type Color = usize;
type Cluster = LineNumbers;
type LineNumbers = Vec<LineNumber>;
type LineNumber = isize;


pub fn color_graph<N, E>(interference_graph: &UnGraph<N, E>) -> HashMap<NodeIndex, Color> {
    let mut color_mapping = HashMap::<NodeIndex, Color>::new();
    let max_colors = 5; // max according to doc


    // Start coloring from the first node
    if let Some(start_node) = interference_graph.node_indices().next() {
        if !backtrack(interference_graph, start_node, &mut color_mapping, max_colors, 1) {
            panic!("Graph cannot be colored with {} colors", max_colors);
        }
    }

    color_mapping
}

// Function to recursively attempt to color the graph
pub fn backtrack<N, E>(
    graph: &UnGraph<N, E>,
    node: NodeIndex,
    color_mapping: &mut HashMap<NodeIndex, Color>,
    max_colors: Color,
    current_color: Color,
) -> bool {
    if current_color > max_colors {
        return false; // No more colors to try
    }

    // Check if current color is valid for this node
    if graph.neighbors(node).all(|neighbor| {
        color_mapping
            .get(&neighbor)
            .map_or(true, |&neighbor_color| neighbor_color != current_color)
    }) {
        // Color is valid, assign it
        color_mapping.insert(node, current_color);

        // Recursively attempt to color the next node
        let next_node = graph.node_indices().find(|&n| !color_mapping.contains_key(&n));
        if let Some(next) = next_node {
            // Attempt to color the next node
            if backtrack(graph, next, color_mapping, max_colors, 1) {
                return true; // Success
            }
        } else {
            return true; // All nodes colored
        }

        // If we failed to color the next node with this color, backtrack
        color_mapping.remove(&node);
    }

    // Try next color
    backtrack(graph, node, color_mapping, max_colors, current_color + 1)
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
