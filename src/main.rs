mod instruction;
mod basic_block;
mod function;
mod program;
mod tokenizer;
mod constant_block;

use crate::constant_block::ConstantBlock;

fn main() {
    let mut constant_block = ConstantBlock::new();
    let num = constant_block.get_constant(123);
    let num = constant_block.get_constant(1);
    let num = constant_block.get_constant(2);
    let num = constant_block.get_constant(1);
    println!("{:?}", num);
}
