use petgraph::graph::{DiGraph, NodeIndex};
use crate::basic_block::{BasicBlock, BasicBlockType};
use crate::constant_block::ConstantBlock;
use crate::function::Function;
use crate::program::Program;

pub fn generate_dot_viz(input_function: &str, program: &Program) -> String {
    let input_graph = program.get_fn(input_function).get_graph();
    let mut output_graph = String::new();
    output_graph.push_str(format!("digraph {} ", input_function.to_string()).as_str());
    output_graph.push_str("{ \n");
    generate_constant_table(&mut output_graph, program);
    generate_blocks(&mut output_graph, input_graph);
    generate_edges(&mut output_graph, input_graph);
    generate_doms(&mut output_graph, input_graph);
    output_graph.push_str("}");

    output_graph
}

fn generate_constant_table(output_graph: &mut String, program: &Program) {
    let mut instructions = String::from("{");
    for (_constant, instruction) in program.get_constant_table() {
        instructions.push_str(format!("{:?}", instruction).as_str());
        instructions.push('|');
    }

    instructions.pop();
    instructions.push('}');

    if instructions.len() == 1 {
        instructions.pop();
    }

    output_graph.push_str(format!("\tCB [shape=record, label=\"<b>CB | {}\"];\n\n", instructions).as_str());
}

fn generate_blocks(output_graph: &mut String, graph: &DiGraph<BasicBlock, BasicBlockType>) {
    let block_indices = get_block_indices(graph);

    for block_index in block_indices {
        let instructions = cat_instructions(graph.node_weight(block_index).unwrap());
        output_graph.push_str(format!("\tbb{} [shape=record, label=\"<b>BB{} | {}\"];\n", block_index.index(), block_index.index(), instructions).as_str());
    }
    output_graph.push_str("\n");
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
    output_graph.push_str(format!("\tCT:s -> bb0:n;\n").as_str());

    for edge in graph.raw_edges() {
        output_graph.push_str(format!("\tbb{}:s -> bb{}:n [label=\"   {:?}\"];\n", edge.source().index(), edge.target().index(), edge.weight).as_str());
    }
    output_graph.push_str("\n");
}

fn generate_doms(output_graph: &mut String, graph: &DiGraph<BasicBlock, BasicBlockType>) {
    let mut block_indices = get_block_indices(graph);
    block_indices.remove(0);

    for block_index in block_indices {
        let dominated_by_index = graph.node_weight(block_index).unwrap().dominator_table.dominated_by;
        output_graph.push_str(format!("\tbb{}:b -> bb{}:b [color=blue, style=dotted];\n", dominated_by_index.index(), block_index.index()).as_str());
    }
    output_graph.push_str("\n");

}

fn get_block_indices(graph: &DiGraph<BasicBlock, BasicBlockType>) -> Vec<NodeIndex> {
    graph.node_indices().collect()
}

