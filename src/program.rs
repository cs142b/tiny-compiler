use crate::function::Function;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Program {
    pub functions: Vec<Function>,
}

impl Program {
    pub fn new() -> Self {
        Program {
            functions: Vec::new(),
        }
    }
}
