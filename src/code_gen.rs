use crate::instruction::{Instruction, Operation};
use crate::register_allocation::color_graph;

type RegSource = u8;
type RegDestination = u8;
type Constant = isize;
pub type Register = u8;
pub type Generic = u32;
pub type AssemblyInstructions = Vec<AssemblyInstruction>;
#[derive(Debug, PartialEq, Eq)]
pub enum AssemblyInstruction {
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

pub enum Fmt {
    F1,
    F2,
    F3,
}

pub type OpCode = u8;
impl AssemblyInstruction {
    pub fn get_op_code(&self) -> OpCode {
        match *self {
            AssemblyInstruction::ADD(_, _, _) => 0,
            AssemblyInstruction::SUB(_, _, _) => 1,
            AssemblyInstruction::MUL(_, _, _) => 2,
            AssemblyInstruction::DIV(_, _, _) => 3,
            AssemblyInstruction::MOD(_, _, _) => 4,
            AssemblyInstruction::CMP(_, _, _) => 5,

            AssemblyInstruction::OR(_, _, _) => 8,
            AssemblyInstruction::AND(_, _, _) => 9,
            AssemblyInstruction::BIC(_, _, _) => 10,
            AssemblyInstruction::XOR(_, _, _) => 11,

            AssemblyInstruction::LSH(_, _, _) => 12,
            AssemblyInstruction::ASH(_, _, _) => 13,
            AssemblyInstruction::CHK(_, _) => 14,

            AssemblyInstruction::ADDI(_, _, _) => 16,
            AssemblyInstruction::SUBI(_, _, _) => 17,
            AssemblyInstruction::MULI(_, _, _) => 18,
            AssemblyInstruction::DIVI(_, _, _) => 19,
            AssemblyInstruction::MODI(_, _, _) => 20,
            AssemblyInstruction::CMPI(_, _, _) => 21,
            AssemblyInstruction::ORI(_, _, _) => 24,
            AssemblyInstruction::ANDI(_, _, _) => 25,
            AssemblyInstruction::BICI(_, _, _) => 26,
            AssemblyInstruction::XORI(_, _, _) => 27,
            AssemblyInstruction::LSHI(_, _, _) => 28,
            AssemblyInstruction::ASHI(_, _, _) => 29,
            AssemblyInstruction::CHKI(_, _) => 30,

            AssemblyInstruction::LDW(_, _, _) => 32,
            AssemblyInstruction::LDX(_, _, _) => 33,
            AssemblyInstruction::POP(_, _, _) => 34,
            AssemblyInstruction::STW(_, _, _) => 36,
            AssemblyInstruction::STX(_, _, _) => 37,
            AssemblyInstruction::PSH(_, _, _) => 38,

            AssemblyInstruction::BEQ(_, _) => 40,
            AssemblyInstruction::BNE(_, _) => 41,
            AssemblyInstruction::BLT(_, _) => 42,
            AssemblyInstruction::BGE(_, _) => 43,
            AssemblyInstruction::BLE(_, _) => 44,
            AssemblyInstruction::BGT(_, _) => 45,

            AssemblyInstruction::BSR(_) => 46,
            AssemblyInstruction::JSR(_) => 48,
            AssemblyInstruction::RET(_) => 49,

            AssemblyInstruction::RDD(_) => 50,
            AssemblyInstruction::WRD(_) => 51,
            AssemblyInstruction::WRH(_) => 52,
            AssemblyInstruction::WRL => 53,
        }
    }
    pub fn get_fmt(&self) -> Fmt {
        match *self {
            // Fmt::F1 - Instructions with three registers
            AssemblyInstruction::ADD(_, _, _)
            | AssemblyInstruction::SUB(_, _, _)
            | AssemblyInstruction::MUL(_, _, _)
            | AssemblyInstruction::DIV(_, _, _)
            | AssemblyInstruction::MOD(_, _, _)
            | AssemblyInstruction::CMP(_, _, _)
            | AssemblyInstruction::OR(_, _, _)
            | AssemblyInstruction::AND(_, _, _)
            | AssemblyInstruction::BIC(_, _, _)
            | AssemblyInstruction::XOR(_, _, _)
            | AssemblyInstruction::LSH(_, _, _)
            | AssemblyInstruction::ASH(_, _, _)
            | AssemblyInstruction::LDX(_, _, _)
            | AssemblyInstruction::STX(_, _, _) => Fmt::F1,

            // Fmt::F2 - Instructions with two registers and a constant
            AssemblyInstruction::ADDI(_, _, _)
            | AssemblyInstruction::SUBI(_, _, _)
            | AssemblyInstruction::MULI(_, _, _)
            | AssemblyInstruction::DIVI(_, _, _)
            | AssemblyInstruction::MODI(_, _, _)
            | AssemblyInstruction::CMPI(_, _, _)
            | AssemblyInstruction::ORI(_, _, _)
            | AssemblyInstruction::ANDI(_, _, _)
            | AssemblyInstruction::BICI(_, _, _)
            | AssemblyInstruction::XORI(_, _, _)
            | AssemblyInstruction::LSHI(_, _, _)
            | AssemblyInstruction::ASHI(_, _, _)
            | AssemblyInstruction::CHKI(_, _)
            | AssemblyInstruction::LDW(_, _, _)
            | AssemblyInstruction::POP(_, _, _)
            | AssemblyInstruction::STW(_, _, _)
            | AssemblyInstruction::PSH(_, _, _) => Fmt::F2,

            // Fmt::F3 - Instructions with one or two registers and a constant or none
            AssemblyInstruction::CHK(_, _)
            | AssemblyInstruction::BEQ(_, _)
            | AssemblyInstruction::BNE(_, _)
            | AssemblyInstruction::BLT(_, _)
            | AssemblyInstruction::BGE(_, _)
            | AssemblyInstruction::BLE(_, _)
            | AssemblyInstruction::BGT(_, _)
            | AssemblyInstruction::BSR(_)
            | AssemblyInstruction::JSR(_)
            | AssemblyInstruction::RET(_)
            | AssemblyInstruction::RDD(_)
            | AssemblyInstruction::WRD(_)
            | AssemblyInstruction::WRH(_)
            | AssemblyInstruction::WRL => Fmt::F3,
        }
    }

    pub fn get_registers(&self) -> Vec<Register> {
        match *self {
            AssemblyInstruction::ADD(rd, rs1, rs2)
            | AssemblyInstruction::SUB(rd, rs1, rs2)
            | AssemblyInstruction::MUL(rd, rs1, rs2)
            | AssemblyInstruction::DIV(rd, rs1, rs2)
            | AssemblyInstruction::MOD(rd, rs1, rs2)
            | AssemblyInstruction::CMP(rd, rs1, rs2)
            | AssemblyInstruction::OR(rd, rs1, rs2)
            | AssemblyInstruction::AND(rd, rs1, rs2)
            | AssemblyInstruction::BIC(rd, rs1, rs2)
            | AssemblyInstruction::XOR(rd, rs1, rs2)
            | AssemblyInstruction::LSH(rd, rs1, rs2)
            | AssemblyInstruction::ASH(rd, rs1, rs2)
            | AssemblyInstruction::LDX(rd, rs1, rs2)
            | AssemblyInstruction::STX(rs1, rd, rs2) => vec![rd, rs1, rs2],

            AssemblyInstruction::ADDI(rd, rs, _)
            | AssemblyInstruction::SUBI(rd, rs, _)
            | AssemblyInstruction::MULI(rd, rs, _)
            | AssemblyInstruction::DIVI(rd, rs, _)
            | AssemblyInstruction::MODI(rd, rs, _)
            | AssemblyInstruction::CMPI(rd, rs, _)
            | AssemblyInstruction::ORI(rd, rs, _)
            | AssemblyInstruction::ANDI(rd, rs, _)
            | AssemblyInstruction::BICI(rd, rs, _)
            | AssemblyInstruction::XORI(rd, rs, _)
            | AssemblyInstruction::LSHI(rd, rs, _)
            | AssemblyInstruction::ASHI(rd, rs, _) => vec![rd, rs],

            AssemblyInstruction::CHK(rs1, rs2) => vec![rs1, rs2],

            AssemblyInstruction::LDW(rd, rs, _)
            | AssemblyInstruction::POP(rd, rs, _)
            | AssemblyInstruction::STW(rs, rd, _)
            | AssemblyInstruction::PSH(rs, rd, _) => vec![rd, rs],

            AssemblyInstruction::CHKI(rs1, _) => vec![rs1],

            AssemblyInstruction::BEQ(rs, _)
            | AssemblyInstruction::BNE(rs, _)
            | AssemblyInstruction::BLT(rs, _)
            | AssemblyInstruction::BGE(rs, _)
            | AssemblyInstruction::BLE(rs, _)
            | AssemblyInstruction::BGT(rs, _) => vec![rs],

            AssemblyInstruction::RDD(rd) => vec![rd],

            AssemblyInstruction::WRD(rs) | AssemblyInstruction::WRH(rs) => vec![rs],

            AssemblyInstruction::BSR(_)
            | AssemblyInstruction::JSR(_)
            | AssemblyInstruction::RET(_)
            | AssemblyInstruction::WRL => vec![],
        }
    }

    pub fn get_c(&self) -> Option<Generic> {
        match *self {
            AssemblyInstruction::ADD(_, _, c)
            | AssemblyInstruction::SUB(_, _, c)
            | AssemblyInstruction::MUL(_, _, c)
            | AssemblyInstruction::DIV(_, _, c)
            | AssemblyInstruction::MOD(_, _, c)
            | AssemblyInstruction::CMP(_, _, c)
            | AssemblyInstruction::OR(_, _, c)
            | AssemblyInstruction::AND(_, _, c)
            | AssemblyInstruction::BIC(_, _, c)
            | AssemblyInstruction::XOR(_, _, c)
            | AssemblyInstruction::LSH(_, _, c)
            | AssemblyInstruction::ASH(_, _, c)
            | AssemblyInstruction::CHK(_, c) => Some(c as Generic),

            AssemblyInstruction::ADDI(_, _, c)
            | AssemblyInstruction::SUBI(_, _, c)
            | AssemblyInstruction::MULI(_, _, c)
            | AssemblyInstruction::DIVI(_, _, c)
            | AssemblyInstruction::MODI(_, _, c)
            | AssemblyInstruction::CMPI(_, _, c)
            | AssemblyInstruction::ORI(_, _, c)
            | AssemblyInstruction::ANDI(_, _, c)
            | AssemblyInstruction::BICI(_, _, c)
            | AssemblyInstruction::XORI(_, _, c)
            | AssemblyInstruction::LSHI(_, _, c)
            | AssemblyInstruction::ASHI(_, _, c)
            | AssemblyInstruction::CHKI(_, c)
            | AssemblyInstruction::LDW(_, _, c)
            | AssemblyInstruction::POP(_, _, c)
            | AssemblyInstruction::STW(_, _, c)
            | AssemblyInstruction::PSH(_, _, c)
            | AssemblyInstruction::BEQ(_, c)
            | AssemblyInstruction::BNE(_, c)
            | AssemblyInstruction::BLT(_, c)
            | AssemblyInstruction::BGE(_, c)
            | AssemblyInstruction::BLE(_, c)
            | AssemblyInstruction::BGT(_, c)
            | AssemblyInstruction::BSR(c)
            | AssemblyInstruction::JSR(c)
            | AssemblyInstruction::RET(c) => Some(c as Generic),

            _ => None,
        }
    }

    pub fn get_a(&self) -> Option<Generic> {
        match *self {
            AssemblyInstruction::ADD(a, _, _)
            | AssemblyInstruction::SUB(a, _, _)
            | AssemblyInstruction::MUL(a, _, _)
            | AssemblyInstruction::DIV(a, _, _)
            | AssemblyInstruction::MOD(a, _, _)
            | AssemblyInstruction::CMP(a, _, _)
            | AssemblyInstruction::OR(a, _, _)
            | AssemblyInstruction::AND(a, _, _)
            | AssemblyInstruction::BIC(a, _, _)
            | AssemblyInstruction::XOR(a, _, _)
            | AssemblyInstruction::LSH(a, _, _)
            | AssemblyInstruction::ASH(a, _, _)
            | AssemblyInstruction::CHK(a, _)
            | AssemblyInstruction::ADDI(a, _, _)
            | AssemblyInstruction::SUBI(a, _, _)
            | AssemblyInstruction::MULI(a, _, _)
            | AssemblyInstruction::DIVI(a, _, _)
            | AssemblyInstruction::MODI(a, _, _)
            | AssemblyInstruction::CMPI(a, _, _)
            | AssemblyInstruction::ORI(a, _, _)
            | AssemblyInstruction::ANDI(a, _, _)
            | AssemblyInstruction::BICI(a, _, _)
            | AssemblyInstruction::XORI(a, _, _)
            | AssemblyInstruction::LSHI(a, _, _)
            | AssemblyInstruction::ASHI(a, _, _)
            | AssemblyInstruction::CHKI(a, _)
            | AssemblyInstruction::LDW(a, _, _)
            | AssemblyInstruction::LDX(a, _, _)
            | AssemblyInstruction::POP(a, _, _)
            | AssemblyInstruction::STW(a, _, _)
            | AssemblyInstruction::STX(a, _, _)
            | AssemblyInstruction::PSH(a, _, _)
            | AssemblyInstruction::BEQ(a, _)
            | AssemblyInstruction::BNE(a, _)
            | AssemblyInstruction::BLT(a, _)
            | AssemblyInstruction::BGE(a, _)
            | AssemblyInstruction::BLE(a, _)
            | AssemblyInstruction::BGT(a, _)
            | AssemblyInstruction::RDD(a) => Some(a as Generic),

            _ => None,
        }
    }

    pub fn get_const(&self) -> Option<Constant> {
        match *self {
            AssemblyInstruction::ADDI(_, _, c)
            | AssemblyInstruction::SUBI(_, _, c)
            | AssemblyInstruction::MULI(_, _, c)
            | AssemblyInstruction::DIVI(_, _, c)
            | AssemblyInstruction::MODI(_, _, c)
            | AssemblyInstruction::CMPI(_, _, c)
            | AssemblyInstruction::ORI(_, _, c)
            | AssemblyInstruction::ANDI(_, _, c)
            | AssemblyInstruction::BICI(_, _, c)
            | AssemblyInstruction::XORI(_, _, c)
            | AssemblyInstruction::LSHI(_, _, c)
            | AssemblyInstruction::ASHI(_, _, c)
            | AssemblyInstruction::CHKI(_, c)
            | AssemblyInstruction::LDW(_, _, c)
            | AssemblyInstruction::POP(_, _, c)
            | AssemblyInstruction::STW(_, _, c)
            | AssemblyInstruction::PSH(_, _, c)
            | AssemblyInstruction::BEQ(_, c)
            | AssemblyInstruction::BNE(_, c)
            | AssemblyInstruction::BLT(_, c)
            | AssemblyInstruction::BGE(_, c)
            | AssemblyInstruction::BLE(_, c)
            | AssemblyInstruction::BGT(_, c)
            | AssemblyInstruction::BSR(c)
            | AssemblyInstruction::JSR(c)
            | AssemblyInstruction::RET(c) => Some(c),

            _ => None,
        }
    }


    pub fn get_b(&self) -> Option<Generic> {
        match *self {
            AssemblyInstruction::ADD(_, b, _)
            | AssemblyInstruction::SUB(_, b, _)
            | AssemblyInstruction::MUL(_, b, _)
            | AssemblyInstruction::DIV(_, b, _)
            | AssemblyInstruction::MOD(_, b, _)
            | AssemblyInstruction::CMP(_, b, _)
            | AssemblyInstruction::OR(_, b, _)
            | AssemblyInstruction::AND(_, b, _)
            | AssemblyInstruction::BIC(_, b, _)
            | AssemblyInstruction::XOR(_, b, _)
            | AssemblyInstruction::LSH(_, b, _)
            | AssemblyInstruction::ASH(_, b, _)
            | AssemblyInstruction::LDX(_, b, _)
            | AssemblyInstruction::STX(_, b, _)
            => Some(b as Generic),

            AssemblyInstruction::ADDI(_, b, _)
            | AssemblyInstruction::SUBI(_, b, _)
            | AssemblyInstruction::MULI(_, b, _)
            | AssemblyInstruction::DIVI(_, b, _)
            | AssemblyInstruction::MODI(_, b, _)
            | AssemblyInstruction::CMPI(_, b, _)
            | AssemblyInstruction::ORI(_, b, _)
            | AssemblyInstruction::ANDI(_, b, _)
            | AssemblyInstruction::BICI(_, b, _)
            | AssemblyInstruction::XORI(_, b, _)
            | AssemblyInstruction::LSHI(_, b, _)
            | AssemblyInstruction::ASHI(_, b, _)
            => Some(b as Generic),

            

            AssemblyInstruction::LDW(_, b, _)
            | AssemblyInstruction::POP(_, b, _)
            => Some(b as Generic),

            AssemblyInstruction::STW(_, b, _)
            | AssemblyInstruction::PSH(_, b, _)
            => Some(b as Generic),

            _ => None,
        }
    }
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
