#[derive(Debug, Clone)]
pub enum Operation {
    Const(isize),
    Add(isize, isize),
    Sub(isize, isize),
    Mul(isize, isize),
    Div(isize, isize),
    Cmp(isize, isize),
    Phi(isize, isize),
    Bra(isize),
    Bne(isize, isize),
    Beq(isize, isize),
    Ble(isize, isize),
    Blt(isize, isize),
    Bge(isize, isize),
    Bgt(isize, isize),
    Jsr(isize),
    Ret(isize),
    GetPar1,
    GetPar2,
    GetPar3,
    SetPar1(isize),
    SetPar2(isize),
    SetPar3(isize),
}

#[derive(Debug)]
pub struct Instruction {
    line_number: isize,
    operation: Operation,
}


// methods
impl Instruction {
    pub fn get_line_number(&self) -> isize {
        self.line_number
    }
}
// associated functions
impl Instruction {
    pub fn create_instruction(line_number: isize, operation: Operation) -> Self {
        Instruction {
            line_number,
            operation
        }
    }
}

