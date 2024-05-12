use std::fmt;


// #[derive(Debug, Clone)] 
// not needed since the display format already implements it
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
    Read,
    Write(isize),
    WriteNL,
    Empty,
}

impl fmt::Debug for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Operation::Const(value1) => write!(f, "const #{}", value1),
            Operation::Add(value1, value2) => write!(f, "add ({}) ({})", value1, value2),
            Operation::Sub(value1, value2) => write!(f, "sub ({}) ({})", value1, value2),
            Operation::Mul(value1, value2) => write!(f, "mul ({}) ({})", value1, value2),
            Operation::Div(value1, value2) => write!(f, "div ({}) ({})", value1, value2),
            Operation::Cmp(value1, value2) => write!(f, "cmp ({}) ({})", value1, value2),
            Operation::Phi(value1, value2) => write!(f, "phi ({}) ({})", value1, value2),
            Operation::Bra(value1) => write!(f, "bra ({})", value1),
            Operation::Bne(value1, value2) => write!(f, "bne ({}) ({})", value1, value2),
            Operation::Beq(value1, value2) => write!(f, "beq ({}) ({})", value1, value2),
            Operation::Ble(value1, value2) => write!(f, "ble ({}) ({})", value1, value2),
            Operation::Blt(value1, value2) => write!(f, "blt ({}) ({})", value1, value2),
            Operation::Bge(value1, value2) => write!(f, "bge ({}) ({})", value1, value2),
            Operation::Bgt(value1, value2) => write!(f, "bgt ({}) ({})", value1, value2),
            Operation::Jsr(value1) => write!(f, "jsr ({})", value1),
            Operation::GetPar1 => write!(f, "getPar1"),
            Operation::GetPar2 => write!(f, "gePar2"),
            Operation::GetPar3 => write!(f, "getPar3"),
            Operation::SetPar1(value1) => write!(f, "setPar1 ({})", value1),
            Operation::SetPar2(value1) => write!(f, "setPar2 ({})", value1),
            Operation::SetPar3(value1) => write!(f, "setPar3 ({})", value1),
            Operation::Read => write!(f, "read"),
            Operation::Write(value1) => write!(f, "write ({})", value1),
            Operation::WriteNL => write!(f, "writeNL"),
            Operation::Empty => write!(f, "<empty>"),
            _ => unreachable!("No other operations exists."),
        }
    }
}


// #[derive(Debug)]
// not needed since the display format already implements it
pub struct Instruction {
    line_number: isize,
    operation: Operation,
}


impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {:?}", self.line_number, self.operation)
    }
}

impl Instruction {
    // methods
    pub fn get_line_number(&self) -> isize {
        self.line_number
    }

    // associated functions 
    /// creates and returns a new instruction
    pub fn new(line_number: isize, operation: Operation) -> Self {
        Self {
            line_number,
            operation
        }
    }
}

