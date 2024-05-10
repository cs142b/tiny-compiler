use std::fmt;


// #[derive(Debug, Clone)]
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

impl fmt::Debug for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Operation::Add(value1, value2) => write!(f, "Add ({:?}) ({:?})", value1, value2),
            Operation::Sub(value1, value2) => write!(f, "Sub ({:?}) ({:?})", value1, value2),
            Operation::Mul(value1, value2) => write!(f, "Mul ({:?}) ({:?})", value1, value2),
            Operation::Div(value1, value2) => write!(f, "Div ({:?}) ({:?})", value1, value2),
            _ => panic!("i do this later"),
        }
    }
}


// #[derive(Debug)]
pub struct Instruction {
    line_number: isize,
    operation: Operation,
}


impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?}: {:?}", 
            self.line_number, self.operation
        )
    }
}
// methods
//
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

