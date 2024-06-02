use petgraph::graph::NodeIndex;
use crate::instruction::Instruction;
use std::collections::HashMap;
use std::fmt;


/// in our implementation, all basic blocks have a type 
/// conditional blocks should only store two pieces of information for bookkeeping which are the cmp and then the jmp instruction
/// blocks that loop back to the conditional block will loop back to the block and not the instruction number in the IR
#[derive(Clone, Copy, Default, PartialEq)]
pub enum BasicBlockType {
    #[default]
    Entry,
    Conditional,
    FallThrough, 
    Branch, 
    Join,
    Exit,
}

impl fmt::Debug for BasicBlockType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            BasicBlockType::Entry => write!(f, "Entry"),
            BasicBlockType::Conditional=> write!(f, "Conditional"),
            BasicBlockType::FallThrough=> write!(f, "FallThrough"),
            BasicBlockType::Branch=> write!(f, "Branch"),
            BasicBlockType::Join=> write!(f, "Join"),
            BasicBlockType::Exit=> write!(f, "Exit"),
        }
    }
}


#[derive(Clone, PartialEq,  Copy)]
pub enum VariableType {
    Phi(isize, isize),
    NotPhi(isize),
    NotInit,
}

// pub type Predecessors = Vec<BasicBlock>;
impl fmt::Debug for VariableType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            VariableType::Phi(value1, value2) => write!(f, "{} != {}", value1, value2),
            VariableType::NotPhi(value) => write!(f, "{}", value),
            _ => unreachable!("should never hit here in debug formatting for variable type"),
        }
    }
}

impl VariableType {

    pub fn is_phi(&self) -> bool {
        match self {
            VariableType::Phi(_, _) => true,
            VariableType::NotPhi(_) => false,
            _ => panic!("placeholder"),

        }
    }

    pub fn get_phi_values(&self) -> (isize, isize) {
        if let VariableType::Phi(value1, value2) = self {
            return (*value1, *value2);
        }

        panic!("this function should only be used if variabletype is known to be a phi");
    }

    pub fn get_not_phi_value(&self) -> isize {
        if let VariableType::NotPhi(value) = self {
            return *value;
        }

        panic!("this function should only be used if variabletype is known to be a not phi");
    }
}

#[derive(Default, Clone)]
pub struct BasicBlock {
    pub id: NodeIndex,
    pub instructions: Vec<Instruction>,
    // pub variable_table: HashMap<String, Option<VariableType>>,
    pub variable_table: HashMap<String, VariableType>,
    pub block_type: BasicBlockType,
}


// u dont need this anymore bruh
impl fmt::Debug for BasicBlock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let id_unwrap = self.id.index();
        let instructions = cat_instructions(self);
        write!(f, "{:?} BB{} | {}", self.block_type, id_unwrap, instructions)
    }
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
            block_type: block_type
        }
    }

    pub fn new_with_id(block_type: BasicBlockType, block_id: usize) -> Self {
        Self {
            id: NodeIndex::new(block_id), 
            instructions: Vec::new(),
            variable_table: HashMap::new(),
            block_type: block_type
        }
    }

    pub fn add_instruction(&mut self, instruction: Instruction) {
        self.instructions.push(instruction);
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
        self.variable_table.insert(variable.to_string(), VariableType::NotPhi(line_number));
    }

    pub fn get_first_instruction_line_number(&self) -> isize {
        if let Some(instruction) = self.instructions.first() {
            instruction.get_line_number()
        } else {
            panic!("Basic block is empty")
        }
    }

    // pub fn propagate_variables(&self, next_block: &mut BasicBlock) {
    //     for (var, &line_num) in &self.variable_table {
    //         if let Some(line) = line_num {
    //             next_block.assign_variable(var, line);
    //         }
    //     }
    // }

    pub fn get_max_parents(&self) -> usize {
        match self.block_type {
            BasicBlockType::Entry  => return 0,
            BasicBlockType::Branch | BasicBlockType::FallThrough => return 1,
            BasicBlockType::Conditional | BasicBlockType::Join => return 2,
            BasicBlockType::Exit => return 1,
        }
    }

    pub fn get_max_children(&self) -> usize {
        match self.block_type {
            BasicBlockType::Entry => return 1,
            BasicBlockType::Branch | BasicBlockType::FallThrough | BasicBlockType::Join => return 1,
            BasicBlockType::Conditional => return 2,
            BasicBlockType::Exit => return 0,
        }
    }
}

#[cfg(test)]
pub mod bb_tests {
    use super::*; 

    #[test]
    fn test_creation(){

        let bb = BasicBlock::new(BasicBlockType::Entry); 
        assert_eq!(bb.block_type, BasicBlockType::Entry);
        assert_eq!((bb.instructions).len(), 0);
        assert_eq!(bb.variable_table.len(), 0);

        let bb = BasicBlock::new(BasicBlockType::Exit); 
        assert_eq!(bb.block_type, BasicBlockType::Exit);
        assert_eq!((bb.instructions).len(), 0);
        assert_eq!(bb.variable_table.len(), 0);

        let bb = BasicBlock::new(BasicBlockType::Conditional); 
        assert_eq!(bb.block_type, BasicBlockType::Conditional);
        assert_eq!((bb.instructions).len(), 0);
        assert_eq!(bb.variable_table.len(), 0);

        let bb = BasicBlock::new(BasicBlockType::Join); 
        assert_eq!(bb.block_type, BasicBlockType::Join);
        assert_eq!((bb.instructions).len(), 0);
        assert_eq!(bb.variable_table.len(), 0);
        
        let bb = BasicBlock::new(BasicBlockType::Branch); 
        assert_eq!(bb.block_type, BasicBlockType::Branch);
        assert_eq!((bb.instructions).len(), 0);
        assert_eq!(bb.variable_table.len(), 0);
    }

    #[test]
    fn test_variable_creation() {

        let mut bb = BasicBlock::new(BasicBlockType::Entry); 
        let var_name = "non_phi";
        bb.declare_variable(&var_name.to_string());

        // assert_eq!(bb.variable_table[var_name], None);
    }
    #[test]
    fn test_variable_assignment() {
        let mut bb = BasicBlock::new(BasicBlockType::Entry); 
        let var_name = "non_phi";
        let var_name: String = var_name.to_string();
        let var_val = VariableType::NotPhi(10);
        // bb.assign_variable(&var_name, var_val);

        // assert_eq!(bb.get_variable(&var_name), VariableType::NotPhi(10));

        let mut bb = BasicBlock::new(BasicBlockType::Entry); 
        let var_name = "phi";
        let var_name: String = var_name.to_string();
        let var_val = VariableType::Phi(10, 10);
        // bb.assign_variable(&var_name, var_val);

        // assert_eq!(bb.get_variable(&var_name), VariableType::Phi(10, 10));


    }
    #[test]
    fn test_add_instruction () {
        let mut bb = BasicBlock::new(BasicBlockType::Join);
        let instruction = Instruction::new(10, crate::instruction::Operation::Add(12, 12));
        bb.add_instruction(instruction.clone());

        let first_instruction = bb.get_first_instruction_line_number(); 

        assert_eq!(10, first_instruction);
        assert_eq!(bb.instructions[0],  instruction);
    }

   
    

}
