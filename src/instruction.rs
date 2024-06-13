use std::fmt;

type LineNumber = isize; 
type FunctionNumber = isize; 
type BasicBlockNumber = isize; 

#[derive(Clone, PartialEq, Eq, Hash, Copy)]
pub enum Operation {
    Const(LineNumber),
    Add(LineNumber, LineNumber),
    Sub(LineNumber, LineNumber),
    Mul(LineNumber, LineNumber),
    Div(LineNumber, LineNumber),
    Cmp(LineNumber, LineNumber),
    Phi(LineNumber, LineNumber),
    Bra(BasicBlockNumber),
    Bne(LineNumber, BasicBlockNumber),
    Beq(LineNumber, BasicBlockNumber),
    Ble(LineNumber, BasicBlockNumber),
    Blt(LineNumber, BasicBlockNumber),
    Bge(LineNumber, BasicBlockNumber),
    Bgt(LineNumber, BasicBlockNumber),
    Jsr(FunctionNumber),
    Ret(LineNumber),
    GetPar1,
    GetPar2,
    GetPar3,
    SetPar1(LineNumber),
    SetPar2(LineNumber),
    SetPar3(LineNumber),
    Read,
    Write(LineNumber),
    WriteNL,
    Empty,
    End,
}

impl fmt::Debug for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Operation::Const(value1) => write!(f, "const #{}", value1),
            Operation::Add(value1, value2) => write!(f, "add ({:?}) ({:?})", value1, value2),
            Operation::Sub(value1, value2) => write!(f, "sub ({:?}) ({:?})", value1, value2),
            Operation::Mul(value1, value2) => write!(f, "mul ({:?}) ({:?})", value1, value2),
            Operation::Div(value1, value2) => write!(f, "div ({:?}) ({:?})", value1, value2),
            Operation::Cmp(value1, value2) => write!(f, "cmp ({:?}) ({:?})", value1, value2),
            Operation::Phi(value1, value2) => write!(f, "phi ({:?}) ({:?})", value1, value2),
            Operation::Bra(value1) => write!(f, "bra (BB{})", value1),
            Operation::Bne(value1, value2) => write!(f, "bne ({:?}) (BB{})", value1, value2),
            Operation::Beq(value1, value2) => write!(f, "beq ({:?}) (BB{})", value1, value2),
            Operation::Ble(value1, value2) => write!(f, "ble ({:?}) (BB{})", value1, value2),
            Operation::Blt(value1, value2) => write!(f, "blt ({:?}) (BB{})", value1, value2),
            Operation::Bge(value1, value2) => write!(f, "bge ({:?}) (BB{})", value1, value2),
            Operation::Bgt(value1, value2) => write!(f, "bgt ({:?}) (BB{})", value1, value2),
            Operation::Jsr(value1) => write!(f, "jsr ({})", value1),
            Operation::GetPar1 => write!(f, "getPar1"),
            Operation::GetPar2 => write!(f, "getPar2"),
            Operation::GetPar3 => write!(f, "getPar3"),
            Operation::SetPar1(value1) => write!(f, "setPar1 ({})", value1),
            Operation::SetPar2(value1) => write!(f, "setPar2 ({})", value1),
            Operation::SetPar3(value1) => write!(f, "setPar3 ({})", value1),
            Operation::Read => write!(f, "read"),
            Operation::Write(value1) => write!(f, "write ({})", value1),
            Operation::WriteNL => write!(f, "writeNL"),
            Operation::Empty => write!(f, "<empty>"),
            Operation::End => write!(f, "End"),
            _ => unreachable!("No other operations exists."),
        }
    }
}

impl Operation {
    pub fn get_lines(&self) -> Vec<isize> {
        let mut v: Vec<isize> = Vec::new(); 
        match *self {
            Operation::Add(l,r)|
            Operation::Sub(l, r)|
            Operation::Mul(l, r)|
            Operation::Div(l, r)|
            Operation::Cmp(l, r)|
            Operation::Phi(l, r) => {
                v.push(l);
                v.push(r); 
            },
            Operation::SetPar1(l) |
            Operation::SetPar3(l) |
            Operation::SetPar3(l) |
            Operation::Write(l) => {
                v.push(l);
            }
            _ => {}
            
            
        }
        v
    }

    
}

#[derive(Clone, PartialEq)]
pub struct Instruction {
    line_number: isize,
    pub operation: Operation,
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

    pub fn get_operation_ref(&self) -> &Operation {
        &self.operation
    }

    // associated functions 
    /// creates and returns a new instruction
    pub fn new(line_number: isize, operation: Operation) -> Self {
        Self {
            line_number,
            operation
        }
    }

    pub fn create_instruction(line_number: isize, operation: Operation) -> Self {
        Instruction::new(line_number, operation)
    }

    // Method to get the defined variable (if any) from an instruction
    pub fn get_def(&self) -> Option<isize> {
        match self.operation {
            Operation::Add(_, _) |
            Operation::Sub(_, _) |
            Operation::Mul(_, _) |
            Operation::Div(_, _) |
            Operation::SetPar1(_) |
            Operation::SetPar2(_) |
            Operation::SetPar3(_) => Some(self.line_number),

            // Add cases for other operations that define a variable
            _ => None,
        }
    }
}
