use crate::instruction::{Operation, Instruction};
use crate::register_allocation::color_graph;
enum AssemblyInstructions {
    
}
pub struct CodeGeneration {
    instructions: Vec<Instruction>,
    // register_mapping: HashMap<LineNumber, RegisterNumber>
}

impl CodeGeneration {
    pub fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            instructions,
            // register_mapping: HashMap::new(),
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn first() {
        let mut instructions: Vec<Instruction> = Vec::new();
        let instruction1 = Instruction::create_instruction(1, Operation::Add(1, 1));
        instructions.push(instruction1);

        for instruction in &instructions {
            println!("{:?}", instruction);
        }
    }
}
