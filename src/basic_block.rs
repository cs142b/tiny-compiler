use petgraph::graph::NodeIndex;
use crate::instruction::{Instruction, Operation};
use crate::dominator_table::DominatorTable;
use std::collections::HashMap;
use std::fmt;

// in our implementation, all basic blocks have a type 
// conditional blocks should only store two pieces of information for bookkeeping which are the cmp and then the jmp instruction
// blocks that loop back to the conditional block will loop back to the block and not the instruction number in the IR
#[derive(Clone, Copy, Default, PartialEq)]
pub enum BasicBlockType {
    #[default]
    Entry,
    Conditional,
    FallThrough, 
    Branch, 
    Follow,
    Join,
    Exit,
}

impl fmt::Debug for BasicBlockType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            BasicBlockType::Entry => write!(f, "entry"),
            BasicBlockType::Conditional=> write!(f, "conditional"),
            BasicBlockType::FallThrough=> write!(f, "fall-through"),
            BasicBlockType::Branch=> write!(f, "branch"),
            BasicBlockType::Follow=> write!(f, "follow"),
            BasicBlockType::Join=> write!(f, "join"),
            BasicBlockType::Exit=> write!(f, "exit"),
        }
    }
}


#[derive(Clone, PartialEq, Eq, Copy)]
pub enum VariableType {
    Value(isize),
    NotInit,
}

// pub type Predecessors = Vec<BasicBlock>;
impl fmt::Debug for VariableType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            VariableType::Value(value) => write!(f, "{}", value),
            VariableType::NotInit => write!(f, "NotInit"),
        }
    }
}

impl VariableType {

    pub fn get_value(&self) -> isize {
        if let VariableType::Value(value) = self {
            return *value;
        }
        eprintln!("Use of an uninitialized value");
        0
    }
}

#[derive(Debug, Default, Clone)]
pub struct BasicBlock {
    pub id: NodeIndex,
    pub instructions: Vec<Instruction>,
    pub variable_table: HashMap<String, VariableType>,
    pub block_type: BasicBlockType,
    pub dominator_table: DominatorTable,
}

impl PartialEq for BasicBlock {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl BasicBlock {
    pub fn new(block_type: BasicBlockType) -> Self {
        Self {
            id: NodeIndex::new(0),
            instructions: Vec::new(),
            variable_table: HashMap::new(),
            block_type,
            dominator_table: DominatorTable::new(),
        }
    }

    pub fn new_with_id(block_type: BasicBlockType, block_id: usize) -> Self {
        Self {
            id: NodeIndex::new(block_id), 
            instructions: Vec::new(),
            variable_table: HashMap::new(),
            block_type,
            dominator_table: DominatorTable::new(),
        }
    }

    pub fn add_instruction(&mut self, instruction: Instruction) {
        self.instructions.push(instruction);
    }

    pub fn add_instruction_on_top(&mut self, instruction: Instruction) {
        self.instructions.insert(0, instruction);
    }

    pub fn declare_variable(&mut self, variable: &String) {
        self.variable_table.insert(variable.to_string(), VariableType::NotInit);
    }

    pub fn get_block_type(&self) -> BasicBlockType {
        self.block_type
    }

    pub fn get_variable(&self, variable: &String) -> VariableType {
        // need to generate phi resolutions in parser
        *self.variable_table.get(variable).unwrap()
    }

    pub fn assign_variable(&mut self, variable: &String, line_number: isize) {
        self.variable_table.insert(variable.to_string(), VariableType::Value(line_number));
    }

    pub fn get_first_instruction_line_number(&self) -> isize {
        if let Some(instruction) = self.instructions.first() {
            instruction.get_line_number()
        } else {
            panic!("Basic block is empty")
        }
    }
    
    pub fn get_last_instruction_line_number(&self) -> isize {
        if let Some(instruction) = self.instructions.last() {
            instruction.get_line_number()
        } else {
            panic!("Basic block is empty")
        }
    }


    pub fn get_max_parents(&self) -> usize {
        match self.block_type {
            BasicBlockType::Entry  => return 0,
            BasicBlockType::Branch | BasicBlockType::FallThrough | BasicBlockType::Follow => return 1,
            BasicBlockType::Conditional | BasicBlockType::Join => return 2,
            BasicBlockType::Exit => return 1,
        }
    }

    pub fn get_max_children(&self) -> usize {
        match self.block_type {
            BasicBlockType::Entry => return 1,
            BasicBlockType::Branch | BasicBlockType::FallThrough | BasicBlockType::Join | BasicBlockType::Follow=> return 1,
            BasicBlockType::Conditional => return 2,
            BasicBlockType::Exit => return 0,
        }
    }

    // Function to modify an existing instruction by its line number
    pub fn modify_instruction(&mut self, line_number: isize, new_operation: Operation) {
        for instruction in &mut self.instructions {
            if instruction.get_line_number() == line_number {
                instruction.operation = new_operation;
                return;
            }
        }
        panic!("Instruction with line number {} not found", line_number);
    }

    pub fn is_empty(&self) -> bool {
        self.instructions.is_empty()
    }
}

// #[cfg(test)]
// pub mod bb_tests {
//     use super::*; 

//     #[test]
//     fn test_creation(){

//         let bb = BasicBlock::new(BasicBlockType::Entry); 
//         assert_eq!(bb.block_type, BasicBlockType::Entry);
//         assert_eq!((bb.instructions).len(), 0);
//         assert_eq!(bb.variable_table.len(), 0);

//         let bb = BasicBlock::new(BasicBlockType::Exit); 
//         assert_eq!(bb.block_type, BasicBlockType::Exit);
//         assert_eq!((bb.instructions).len(), 0);
//         assert_eq!(bb.variable_table.len(), 0);

//         let bb = BasicBlock::new(BasicBlockType::Conditional); 
//         assert_eq!(bb.block_type, BasicBlockType::Conditional);
//         assert_eq!((bb.instructions).len(), 0);
//         assert_eq!(bb.variable_table.len(), 0);

//         let bb = BasicBlock::new(BasicBlockType::Join); 
//         assert_eq!(bb.block_type, BasicBlockType::Join);
//         assert_eq!((bb.instructions).len(), 0);
//         assert_eq!(bb.variable_table.len(), 0);
        
//         let bb = BasicBlock::new(BasicBlockType::Branch); 
//         assert_eq!(bb.block_type, BasicBlockType::Branch);
//         assert_eq!((bb.instructions).len(), 0);
//         assert_eq!(bb.variable_table.len(), 0);
//     }

//     #[test]
//     fn test_variable_creation() {

//         let mut bb = BasicBlock::new(BasicBlockType::Entry); 
//         let var_name = "non_phi";
//         bb.declare_variable(&var_name.to_string());

//         // assert_eq!(bb.variable_table[var_name], None);
//     }

//     #[test]
//     fn test_add_instruction () {
//         let mut bb = BasicBlock::new(BasicBlockType::Join);
//         let instruction = Instruction::new(10, crate::instruction::Operation::Add(12, 12));
//         bb.add_instruction(instruction.clone());

//         let first_instruction = bb.get_first_instruction_line_number(); 

//         assert_eq!(10, first_instruction);
//         assert_eq!(bb.instructions[0],  instruction);
//     }

   
    

// }
