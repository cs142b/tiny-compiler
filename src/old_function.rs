use crate::basic_block::{BasicBlock, BasicBlockType, VariableType};
use crate::basic_block_list::BasicBlockList;

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub parameters: Vec<String>,
    pub bb_list: BasicBlockList,
}

impl Function {
    pub fn new(name: String, parameters: Vec<String>) -> Self {
        Self {
            name,
            parameters,
            bb_list: BasicBlockList::new(),
        }
    }



}
