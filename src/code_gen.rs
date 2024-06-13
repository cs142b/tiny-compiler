use crate::instruction::Instruction;
use crate::register_allocation::color_graph;

type LineNumber = isize;
type RegisterNumber = usize;

pub struct CodeGeneration {
    instructions: Vec<Instructions>,
    register_mapping: HashMap<LineNumber, RegisterNumber>
}

impl CodeGeneration {
    pub fn new(instructions: Vec<Instructions>) -> Self {
        Self {
            instructions,
            register_mapping: HashMap::new(),
        }
    }

}
