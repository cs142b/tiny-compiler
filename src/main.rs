mod instruction;
mod basic_block;
mod function;
mod program;
mod tokenizer;
mod constant_block;
mod parser;

use crate::constant_block::ConstantBlock;
use crate::parser::Parser;

fn main() {
    let mut parser = Parser::new("5 + 5; 5 - 1; 1 * 3; 3 / 3.".to_string());
    // parser.parse_lol();
}
