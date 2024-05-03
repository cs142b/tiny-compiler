use crate::{Function, Operand};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Program {
    pub functions: Vec<Function>,
    pub variable_table: HashMap<String, (usize, usize)>, // (basic block id, instruction index)
}

impl Program {
    pub fn new() -> Self {
        Program {
            functions: Vec::new(),
            variable_table: HashMap::new(),
        }
    }
}