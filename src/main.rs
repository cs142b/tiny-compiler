mod instruction;
mod basic_block;
mod function;
mod program;
mod tokenizer;
mod constant_block;
mod parser;
mod dot_viz;
mod dominator_table;
mod live_analysis;
mod cfg_traversal;
mod register_allocation;
mod code_gen;
mod assembler;

// use crate::parser::Parser;

fn main() {

}

#[cfg(test)]
mod parser_tests {
    use parser::Parser;

    use super::*;
    use crate::live_analysis::compute_live_sets;

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
        ".to_string();
        let mut parser = Parser::new(input);
        parser.parse_computation();
        let graph = &parser.internal_program.get_curr_fn().bb_graph;

        // Compute live sets
        let live_sets = compute_live_sets(graph);

        // Print out the live sets for each block
        for (node_index, block_info) in &live_sets {
            println!("Node {:?} - IN: {:?}, OUT: {:?}", node_index, block_info.in_set, block_info.out_set);
        }

        // Add assertions as needed to verify the live sets
        // For example, check the IN and OUT sets for the specific basic blocks
        assert!(live_sets.len() > 0); // Ensure there are live sets computed
    }
}
