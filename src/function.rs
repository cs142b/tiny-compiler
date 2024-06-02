use std::collections::HashMap;
use std::os::unix::process::parent_id;

use petgraph::adj::NodeIndex;

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

    fn propagate_variables(&mut self) {
        let prev = self.get_parent();
        let curr = self.get_current_block();

        curr.variable_table = curr.variable_table.clone();
    }

    fn propagate_variables_join(&mut self) {
        let parent_bbs = self.get_parents_join();

        // Collect the variables to be assigned first
        let mut variables_to_assign = Vec::new();

        for (var_name, variable) in &parent_bbs.0.variable_table {
            let var_left = parent_bbs.0.get_variable(var_name);
            let var_right = parent_bbs.1.get_variable(var_name);

            let mut var_to_add = var_left;

            if !var_left.is_phi() && !var_right.is_phi() {
                if var_left.get_not_phi_value() != var_right.get_not_phi_value(){
                    var_to_add = VariableType::Phi(var_left.get_not_phi_value(), var_right.get_not_phi_value());
                }
            }

            variables_to_assign.push((var_name.clone(), var_to_add));
        }

        // Mutably borrow `self` to assign the variables
        {
            let curr = self.bb_list.get_current_block();

            for (var_name, var_to_add) in variables_to_assign {
                curr.assign_variable(&var_name, var_to_add); 
            }
        }
    }

}
