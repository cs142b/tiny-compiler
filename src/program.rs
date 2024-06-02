use crate::{
    basic_block::{BasicBlock, BasicBlockType, VariableType},
    constant_block::ConstantBlock,
    function::Function,
    instruction::Instruction,
};

use petgraph::graph::NodeIndex;

#[derive(Debug)]
pub struct Program {
    pub functions: Vec<Function>,
    pub current_function: usize,
    pub constant_block: ConstantBlock,
}

impl Program {
    pub fn new() -> Self {
        Self {
            // as for now, program will assume there will only be 1 MAIN function before we do part 3
            functions: Vec::new(),
            current_function: 0,
            constant_block: ConstantBlock::new(),
        }
    }

    pub fn add_function(&mut self, name: String, parameters: Vec<String>) -> usize {
        let new_fn = Function::new(name, parameters);
        self.functions.push(new_fn);
        self.functions.len() - 1
    }


    pub fn get_number_of_functions(&self) -> usize {
        self.functions.len()
    }

    pub fn get_curr_fn_index(&self) -> usize {
        self.current_function // assume its main, cuz of part 3
    }

    pub fn get_curr_fn(&self) -> &Function {
        &self.functions[self.current_function]
    }

    pub fn get_curr_fn_mut(&mut self) -> &mut Function {
        &mut self.functions[self.current_function]
    }
    
    pub fn get_curr_block(&mut self) -> &BasicBlock {
        self.get_curr_fn().get_curr_bb()
    }

    pub fn get_curr_block_mut(&mut self) -> &mut BasicBlock {
        self.get_curr_fn_mut().get_curr_bb_mut()
    }

    pub fn add_fallthru_block(&mut self) -> NodeIndex {
        self.get_curr_fn_mut().add_node_to_curr(BasicBlockType::FallThrough)
    }

    pub fn add_branch_block(&mut self) -> NodeIndex {
        self.get_curr_fn_mut().add_node_to_curr(BasicBlockType::Branch) 
    }

    pub fn add_join_block(&mut self, left_parent: NodeIndex, right_parent: NodeIndex) -> NodeIndex{
        self.get_curr_fn_mut().add_join_block(left_parent, right_parent)
    }

    pub fn add_cond_block(&mut self) -> NodeIndex {
        self.get_curr_fn_mut().add_node_to_curr(BasicBlockType::Conditional)
    }

    ///returns the basic block that you are altering in mutable form
    pub fn add_instruction_to_curr_block(&mut self, instruction_to_add: Instruction) {
        let curr_block = self.get_curr_fn_mut().get_curr_bb_mut();
        curr_block.add_instruction(instruction_to_add);
    }


    pub fn assign_variable_to_curr_block(&mut self, var_name: &String, line_number: isize) {
        self.get_curr_block_mut().assign_variable(&var_name, line_number);
    }

    pub fn declare_variable_to_curr_block(&mut self, var_name: &String) {
        self.get_curr_block_mut().declare_variable(&var_name);
    }

    pub fn get_variable(&mut self, variable: &String) -> isize {
        self.get_curr_block().get_variable(variable)
    }

    pub fn add_constant(&mut self, constant: isize) {
        self.constant_block.add_constant(constant);
    }

    pub fn get_constant(&mut self, constant: isize) -> isize {
        self.constant_block.get_constant(constant)
    }
}
