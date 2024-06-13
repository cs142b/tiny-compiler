use crate::instruction::{Operation, Instruction};
use crate::register_allocation::color_graph;

type RegSource = usize;
type RegDestination = usize;
type Constant = isize; 

enum AssemblyInstruction {
    ADD(RegDestination, RegSource, RegSource),
    SUB(RegDestination, RegSource, RegSource),
    MUL(RegDestination, RegSource, RegSource),
    DIV(RegDestination, RegSource, RegSource),
    MOD(RegDestination, RegSource, RegSource),
    CMP(RegDestination, RegSource, RegSource),
    OR(RegDestination, RegSource, RegSource),
    AND(RegDestination, RegSource, RegSource),
    BIC(RegDestination, RegSource, RegSource),
    XOR(RegDestination, RegSource, RegSource),

    LSH(RegDestination, RegSource, RegSource),
    ASH(RegDestination, RegSource, RegSource),
    CHK(RegSource, RegSource),


    ADDI(RegDestination, RegSource, Constant),
    SUBI(RegDestination, RegSource, Constant),
    MULI(RegDestination, RegSource, Constant),
    DIVI(RegDestination, RegSource, Constant),
    MODI(RegDestination, RegSource, Constant),
    CMPI(RegDestination, RegSource, Constant),
    ORI(RegDestination, RegSource, Constant),
    ANDI(RegDestination, RegSource, Constant),
    BICI(RegDestination, RegSource, Constant),
    XORI(RegDestination, RegSource, Constant),

    LSHI(RegDestination, RegSource, Constant),
    ASHI(RegDestination, RegSource, Constant),

    CHKI(RegSource, Constant),

    LDW(RegDestination, RegSource, Constant),
    LDX(RegDestination, RegSource, RegSource),
    POP(RegDestination, RegSource, Constant),

    STW(RegSource, RegDestination, Constant),
    STX(RegSource, RegDestination, RegDestination),
    PSH(RegSource, RegDestination, Constant),

    BEQ(RegSource, Constant),
    BNE(RegSource, Constant),
    BLT(RegSource, Constant),
    BGE(RegSource, Constant),
    BLE(RegSource, Constant),
    BGT(RegSource, Constant),

    BSR(Constant),
    JSR(Constant),
    RET(Constant),

    RDD(RegDestination),
    WRD(RegSource),
    WRH(RegSource),
    WRL,
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
