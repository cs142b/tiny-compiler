use crate::{basic_block::BasicBlock, function::Function, instruction::Instruction, constant_block::ConstantBlock};

#[derive(Debug)]
pub struct Program {
    pub functions: Vec<Function>,
    pub constant_block: ConstantBlock,
    num_functions: usize,
}

impl Program {

    pub fn new() -> Self {
        Self {
            functions: Vec::new(),
            constant_block: ConstantBlock::new(),
            num_functions: 0,
        }

    }

    pub fn add_function(&mut self, name: String, parameters: Vec<String>) -> &mut Function {
        let new_fn = Function::new(name, parameters);
        self.num_functions += 1; 
        self.functions.push(new_fn);
        self.get_curr_fn_mut()
    }

    //returns the position of the current function in the program vector
    fn get_curr_function_pos(&self) -> usize {
        self.num_functions - 1
    }
    
    pub fn get_curr_fn(&self) -> &Function {
        &self.functions[self.get_curr_function_pos()]
    }

    pub fn get_curr_fn_mut(&mut self) -> &mut Function {
        let pos = self.get_curr_function_pos();
        &mut self.functions[pos]
    }

    pub fn get_curr_block_mut(&mut self) -> &mut BasicBlock {
        self.get_curr_fn_mut().get_current_block()
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

    pub fn add_cond_block(&mut self) {
        self.get_curr_fn_mut().add_cond_block();
    }

    ///returns the basic block that you are altering in mutable form 
    pub fn add_instruction_to_curr_block(&mut self, instruction_to_add: Instruction) {
        let curr_block = self.get_curr_fn_mut().get_current_block(); 
        curr_block.add_instruction(instruction_to_add); 
    }

    ///adds a vector of instructions to the current block on which you are operating
    pub fn add_instructions_to_curr_block(&mut self, instructions_to_add: &mut Vec<Instruction>) {
        let curr_block = self.get_curr_fn_mut().get_current_block(); 
        curr_block.instructions.append(instructions_to_add);
    }

    pub fn assign_variable_to_curr_block(&mut self, var_name: &String, line_num: isize){
        self.get_curr_block_mut().assign_variable(&var_name, line_num);
    }

    pub fn add_uninitialized_variable_to_curr_block(&mut self, var_name: &String){
        self.get_curr_block_mut().initalize_variable(&var_name);
    }
    
    pub fn get_variable(&mut self, variable: &String) -> isize {
        self.get_curr_block_mut().get_variable(variable)
    }

    pub fn add_constant(&mut self, constant: isize) {
        self.constant_block.add_constant(constant);
    }

    pub fn get_constant(&mut self, constant: isize) -> isize {
        self.constant_block.get_constant(constant)
    }


}
