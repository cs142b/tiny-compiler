use crate::function::Function;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Program {
    pub functions: Vec<Function>,
}

impl Program {
    pub fn new() -> Self {
        let program = Program {
            functions: Vec::new(),
        };
        
        // add the main function
        program.functions.add(Function::new("main"));

        program
    }

    pub fn add_function(&mut self, function: Function) {
        self.functions.push(function);
    }
}
