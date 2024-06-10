use std::collections::HashMap;
use crate::instruction::Operation;
use petgraph::graph::NodeIndex;

#[derive(Clone, Debug, Default)]
pub struct DominatorTable {
    add_list: HashMap::<Operation, isize>,
    sub_list: HashMap::<Operation, isize>,
    mul_list: HashMap::<Operation, isize>,
    div_list: HashMap::<Operation, isize>,
    pub dominated_by: NodeIndex,
}

impl DominatorTable {
    pub fn new() -> Self {
        Self {
            add_list: HashMap::<Operation, isize>::new(),
            sub_list: HashMap::<Operation, isize>::new(),
            mul_list: HashMap::<Operation, isize>::new(),
            div_list: HashMap::<Operation, isize>::new(),
            dominated_by: NodeIndex::new(0),
        }
    }

    // pub fn propagate_table() -> DominatorTable {
    // }
    
    // inserts the instructions in the table
    pub fn insert_instruction(&mut self, operation: &Operation, line_number: isize) {
        let inserted_operation = operation.clone();
        match operation {
            Operation::Add(_, _) => { self.add_list.insert(inserted_operation, line_number); },
            Operation::Sub(_, _) => { self.sub_list.insert(inserted_operation, line_number); },
            Operation::Mul(_, _) => { self.mul_list.insert(inserted_operation, line_number); },
            Operation::Div(_, _) => { self.div_list.insert(inserted_operation, line_number); },
            _ => (),
        }
    }

    // checks if the instruction already exists
    pub fn is_in_table(&self, operation: &Operation) -> bool {
        match operation {
            Operation::Add(_, _) => self.add_list.contains_key(operation),
            Operation::Sub(_, _) => self.sub_list.contains_key(operation),
            Operation::Mul(_, _) => self.mul_list.contains_key(operation),
            Operation::Div(_, _) => self.div_list.contains_key(operation),
            _ => false,
        }
    }

    // returns instruction line number of a dominated instruction
    pub fn get_dominated_line_number(&self, operation: &Operation) -> isize {
        match operation {
            Operation::Add(_, _) => *self.add_list.get(operation).unwrap(),
            Operation::Sub(_, _) => *self.sub_list.get(operation).unwrap(),
            Operation::Mul(_, _) => *self.mul_list.get(operation).unwrap(),
            Operation::Div(_, _) => *self.div_list.get(operation).unwrap(),
            _ => panic!("Should only be used for add, sub, mul, div"),
        }
    }
}
