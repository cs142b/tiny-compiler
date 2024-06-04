use petgraph::graph::{DiGraph, NodeIndex};
use crate::basic_block::{BasicBlock, BasicBlockType};

pub fn generate_dot_viz(input_graph: &DiGraph<BasicBlock, BasicBlockType>) -> String {
    let mut output_graph = String::new();
    output_graph.push_str("digraph {\n");
    generate_blocks(&mut output_graph, input_graph);
    generate_edges(&mut output_graph, input_graph);
    generate_doms(&mut output_graph, input_graph);
    output_graph.push_str("}");

    output_graph
}


fn generate_blocks(output_graph: &mut String, graph: &DiGraph<BasicBlock, BasicBlockType>) {
    let block_indices = get_block_indices(graph);

    for block_index in block_indices {
        let instructions = cat_instructions(graph.node_weight(block_index).unwrap());
        output_graph.push_str(format!("bb{} [shape=record, label=\"<b>BB{} | {}\"];\n", block_index.index(), block_index.index(), instructions).as_str());
    }
}

fn cat_instructions(block: &BasicBlock) -> String {
    let mut instructions = String::from("{");
    for instruction in &block.instructions {
        instructions.push_str(format!("{:?}", instruction).as_str());
        instructions.push('|');
    }

    instructions.pop();
    instructions.push('}');

    if instructions.len() == 1 {
        instructions.pop();
    }

    instructions
}

fn generate_edges(output_graph: &mut String, graph: &DiGraph<BasicBlock, BasicBlockType>) {
    for edge in graph.raw_edges() {
        output_graph.push_str(format!("bb{}:s -> bb{}:n [label=\"   {:?}\"];\n", edge.source().index(), edge.target().index(), edge.weight).as_str());
    }
}

fn generate_doms(output_graph: &mut String, graph: &DiGraph<BasicBlock, BasicBlockType>) {
    let mut block_indices = get_block_indices(graph);
    block_indices.remove(0);

    for block_index in block_indices {
        let dominated_by_index = graph.node_weight(block_index).unwrap().dominator_table.dominated_by;
        output_graph.push_str(format!("bb{}:b -> bb{}:b [color=blue, style=dotted];\n", dominated_by_index.index(), block_index.index()).as_str());
    }

}

fn get_block_indices(graph: &DiGraph<BasicBlock, BasicBlockType>) -> Vec<NodeIndex> {
    graph.node_indices().collect()
}

