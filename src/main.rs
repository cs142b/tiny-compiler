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
    let mut parser = Parser::new("5".to_string());
    let num = parser.parse_expression();
    println!("{:?}", num);
}
