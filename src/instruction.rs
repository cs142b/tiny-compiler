#[derive(Debug, Clone)]
pub enum Operand {
    Variable(String),
    Number(i32),
}

#[derive(Debug, Clone)]
pub enum Instruction {
    Const(i32),
    Add(Operand, Operand),
    Sub(Operand, Operand),
    Mul(Operand, Operand),
    Div(Operand, Operand),
    Cmp(Operand, Operand),
    Phi(Operand, Operand),
    Bra(String),
    Bne(Operand, String),
    Beq(Operand, String),
    Ble(Operand, String),
    Blt(Operand, String),
    Bge(Operand, String),
    Bgt(Operand, String),
    Jsr(String),
    Ret(Operand),
    GetPar1,
    GetPar2,
    GetPar3,
    SetPar1(Operand),
    SetPar2(Operand),
    SetPar3(Operand),
}