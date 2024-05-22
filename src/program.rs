use crate::{basic_block::BasicBlock, function::Function};

#[derive(Debug)]
pub struct Program {
    pub functions: Vec<Function>,
    num_functions: usize
}

impl Program {

    pub fn new() -> Self {
        Self {
            functions: Vec::new(),
            num_functions: 0
        }

    }

    pub fn add_function(&mut self, name: String, parameters: Vec<String>) -> &mut Function {
        let new_fn = Function::new(name, parameters);
        self.num_functions += 1; 
        self.functions.push(new_fn);
        self.functions.last_mut().expect("Unexpected error in adding new function in program.")
    }

    //returns the position of the current function in the program vector
    fn get_curr_function_pos(&self) -> usize {
        self.num_functions - 1
    }
    
    pub fn get_curr_fn(&self) -> &Function {
        &self.functions[self.get_curr_function_pos()]
    }

    pub fn get_curr_fn_mut(&self) -> &mut Function {
        &mut self.functions[self.get_curr_function_pos()]
    }

    pub fn get_curr_block_mut(&mut self) -> &mut BasicBlock {
        &mut self.get_curr_fn_mut().get_current_block()
    }

    pub fn get_prev_block_mut(&mut self) -> &mut BasicBlock {
        self.get_curr_fn_mut().get_parent()
    }

    pub fn add_branch_block(&mut self) {
        self.get_curr_fn_mut().add_branch_block(); 
    }
    
    pub fn add_join_block(&mut self) {
        self.get_curr_fn_mut().add_join_block(); 
    }

    pub fn add_fall_thru_block(&mut self) {
        self.get_curr_fn_mut().add_fall_thru_block();
    }
}
