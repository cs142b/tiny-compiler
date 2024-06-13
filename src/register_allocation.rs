use petgraph::graph::NodeIndex;
use std::collections::HashMap;
use petgraph::graph::{Graph, UnGraph};
use petgraph::Undirected;

type Cluster = LineNumbers;
type LineNumbers = Vec<LineNumber>;
type LineNumber = isize;
type Color = usize;

pub fn generate_register_mapping(interference_graph: &UnGraph<Cluster, ()>) -> HashMap<LineNumber, Color> {
    let mut register_mapping: HashMap<LineNumber, Color> = HashMap::new();

    for (node_index, register_number) in color_graph(interference_graph) {
        for line_number in interference_graph.node_weight(node_index).unwrap() {
            register_mapping.insert(*line_number, register_number);
        }
    }

    register_mapping
}

pub fn color_graph<N, E>(interference_graph: &UnGraph<N, E>) -> HashMap<NodeIndex, Color> {
    let mut color_mapping = HashMap::<NodeIndex, Color>::new();
    let max_colors = 15; // max according to doc


    // start coloring from the first node
    if let Some(start_node) = interference_graph.node_indices().next() {
        if !backtrack(interference_graph, start_node, &mut color_mapping, max_colors, 1) {
            panic!("Graph cannot be colored with {} colors", max_colors);
        }
    }

    color_mapping
}

// function to recursively attempt to color the graph
pub fn backtrack<N, E>(
    graph: &UnGraph<N, E>,
    node: NodeIndex,
    color_mapping: &mut HashMap<NodeIndex, Color>,
    max_colors: Color,
    current_color: Color,
) -> bool {
    if current_color > max_colors {
        return false; // no more colors to try
    }

    // check if current color is valid for this node
    if graph.neighbors(node).all(|neighbor| {
        color_mapping
            .get(&neighbor)
            .map_or(true, |&neighbor_color| neighbor_color != current_color)
    }) {
        // color is valid, assign it
        color_mapping.insert(node, current_color);

        // recursively attempt to color the next node
        let next_node = graph.node_indices().find(|&n| !color_mapping.contains_key(&n));
        if let Some(next) = next_node {
            // Attempt to color the next node
            if backtrack(graph, next, color_mapping, max_colors, 1) {
                return true; // Success
            }
        } else {
            return true; // All nodes colored
        }

        // if we failed to color the next node with this color, backtrack
        color_mapping.remove(&node);
    }

    // try next color
    backtrack(graph, node, color_mapping, max_colors, current_color + 1)
}


#[cfg(test)]
mod graph_test {
    use super::*;
    use crate::parser::Parser;
    use crate::live_analysis::*; //{get_interference_graph, get_upgraded_interference_graph};
    use petgraph::dot::{Dot, Config};

    #[test]
    pub fn first() {
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

        let graph = get_interference_graph(&parser.internal_program.get_curr_fn().bb_graph);

        let cluster_possibilities = get_clusters(&bbg);
        let upgraded_ig = get_upgraded_interference_graph(&graph, &cluster_possibilities);
        println!("{:?}", Dot::with_config(&upgraded_ig, &[Config::EdgeNoLabel]));
        println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));
        
        println!("line numbers start here");
        for (line_number, register_num) in &generate_register_mapping(&upgraded_ig) {
            println!("{:?}: {:?}", line_number, register_num);
        }

    }
}
