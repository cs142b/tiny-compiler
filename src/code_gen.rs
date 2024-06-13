use crate::instruction::{Operation, Instruction};
use crate::register_allocation::color_graph;
use crate::live_analysis::*;
use crate::cfg_traversal::*;
use crate::parser::*;
use crate::register_allocation::*;
use std::collections::HashMap;

type RegSource = usize;
type RegDestination = usize;
type Constant = isize; 
type RegisterNumber = usize;
type LineNumber = isize;

#[derive(Debug)]
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
    register_mapping: HashMap<LineNumber, RegisterNumber>
}

impl CodeGeneration {
    pub fn new(graph: &mut BasicBlockGraph) -> Self {
        let graph1 = graph.clone();
        let new_graph = get_interference_graph(&graph1);
        let cluster_possibilities = get_clusters(&graph1);
        let new_upgraded_graph = get_upgraded_interference_graph(&new_graph, &cluster_possibilities);
        
        let register_mapping = generate_register_mapping(&new_upgraded_graph);

        let mut graph2 = graph.clone();
        let instructions = traverse_in_order(&mut graph2);

        Self {
            instructions,
            register_mapping,
        }
    }

    pub fn generate_code(&mut self) {
        let mut assembly_instructions: Vec<AssemblyInstruction> = Vec::new();
        for instruction in &self.instructions {
            let line_number = instruction.get_line_number();
            let operation = *instruction.get_operation_ref();

            if operation == Operation::Empty {
                continue;
            }

            if operation == Operation::End {
                break;
            }
            
            let line_num_register = *self.register_mapping.get(&line_number).unwrap();
            match operation {
                Operation::Add(value1, value2) => {
                    match (value1, value2) {
                        _ if value1 <= 0 && value2 <= 0 => {
                            assembly_instructions.push(AssemblyInstruction::ADDI(line_num_register, 0, -value1));
                            assembly_instructions.push(AssemblyInstruction::ADDI(line_num_register, 0, -value2));
                        },
                        _ if value1 > 0 && value2 > 0 => {
                            let value1_register = *self.register_mapping.get(&value1).unwrap(); 
                            let value2_register = *self.register_mapping.get(&value2).unwrap();
                            assembly_instructions.push(AssemblyInstruction::ADD(line_num_register, value1_register, value2_register));

                        },
                        _ if value1 <= 0 => {
                            let constant = -value1;
                            let value2_register = *self.register_mapping.get(&value2).unwrap();
                            assembly_instructions.push(AssemblyInstruction::ADDI(line_num_register, value2_register, constant));
                        },
                        _ if value2 <= 0 => {
                            let constant = -value2;
                            let value1_register = *self.register_mapping.get(&value1).unwrap();
                            assembly_instructions.push(AssemblyInstruction::ADDI(line_num_register, value1_register, constant));

                        },
                        _ => unreachable!("should never reach here"),
                    }
                },
                Operation::Sub(value1, value2) => {
                    match (value1, value2) {
                        _ if value1 <= 0 && value2 <= 0 => {
                            assembly_instructions.push(AssemblyInstruction::SUBI(line_num_register, 0, -value1));
                            assembly_instructions.push(AssemblyInstruction::SUBI(line_num_register, 0, -value2));

                        },
                        _ if value1 > 0 && value2 > 0 => {
                            let value1_register = *self.register_mapping.get(&value1).unwrap(); 
                            let value2_register = *self.register_mapping.get(&value2).unwrap();
                            assembly_instructions.push(AssemblyInstruction::SUB(line_num_register, value1_register, value2_register));

                        },
                        _ if value1 <= 0 => {
                            let constant = -value1;
                            let value2_register = *self.register_mapping.get(&value2).unwrap();
                            assembly_instructions.push(AssemblyInstruction::SUBI(line_num_register, value2_register, constant));
                        },
                        _ if value2 <= 0 => {
                            let constant = -value2;
                            let value1_register = *self.register_mapping.get(&value1).unwrap();
                            assembly_instructions.push(AssemblyInstruction::SUBI(line_num_register, value1_register, constant));

                        },
                        _ => unreachable!("should never reach here"),
                    }
                },
                Operation::Mul(value1, value2) => {
                    match (value1, value2) {
                        _ if value1 <= 0 && value2 <= 0 => {
                            assembly_instructions.push(AssemblyInstruction::MULI(line_num_register, 0, -value1));
                            assembly_instructions.push(AssemblyInstruction::MULI(line_num_register, 0, -value2));

                        },
                        _ if value1 > 0 && value2 > 0 => {
                            let value1_register = *self.register_mapping.get(&value1).unwrap(); 
                            let value2_register = *self.register_mapping.get(&value2).unwrap();
                            assembly_instructions.push(AssemblyInstruction::MUL(line_num_register, value1_register, value2_register));

                        },
                        _ if value1 <= 0 => {
                            let constant = -value1;
                            let value2_register = *self.register_mapping.get(&value2).unwrap();
                            assembly_instructions.push(AssemblyInstruction::MULI(line_num_register, value2_register, constant));
                        },
                        _ if value2 <= 0 => {
                            let constant = -value2;
                            let value1_register = *self.register_mapping.get(&value1).unwrap();
                            assembly_instructions.push(AssemblyInstruction::MULI(line_num_register, value1_register, constant));

                        },
                        _ => unreachable!("should never reach here"),
                    }
                },
                Operation::Div(value1, value2) => {
                    match (value1, value2) {
                        _ if value1 <= 0 && value2 <= 0 => {
                            assembly_instructions.push(AssemblyInstruction::DIVI(line_num_register, 0, -value1));
                            assembly_instructions.push(AssemblyInstruction::DIVI(line_num_register, 0, -value2));

                        },
                        _ if value1 > 0 && value2 > 0 => {
                            let value1_register = *self.register_mapping.get(&value1).unwrap(); 
                            let value2_register = *self.register_mapping.get(&value2).unwrap();
                            assembly_instructions.push(AssemblyInstruction::DIV(line_num_register, value1_register, value2_register));

                        },
                        _ if value1 <= 0 => {
                            let constant = -value1;
                            let value2_register = *self.register_mapping.get(&value2).unwrap();
                            assembly_instructions.push(AssemblyInstruction::DIVI(line_num_register, value2_register, constant));
                        },
                        _ if value2 <= 0 => {
                            let constant = -value2;
                            let value1_register = *self.register_mapping.get(&value1).unwrap();
                            assembly_instructions.push(AssemblyInstruction::DIVI(line_num_register, value1_register, constant));

                        },
                        _ => unreachable!("should never reach here"),
                    }
                },
                Operation::Phi(value1, value2) => {
                    let value1_register = *self.register_mapping.get(&value1).unwrap();
                    let value2_register = *self.register_mapping.get(&value2).unwrap();

                    match (line_num_register, value1_register, value2_register) {
                        _ if (line_num_register == value1_register) & (value1_register == value2_register) => {
                            // if all phi matches, do nothing?
                        },
                        _ if (line_num_register != value1_register) & (value1_register == value2_register) => {
                            // conflict
                            assembly_instructions.push(AssemblyInstruction::ADD(value1_register, line_num_register, 0));

                        },
                        _ if (line_num_register == value1_register) & (value1_register != value2_register) => {
                            // conflict
                            assembly_instructions.push(AssemblyInstruction::ADD(line_num_register, value2_register, 0));
                        },
                        _ => unreachable!("should never reach here"),
                        
                    }

                },
                _ => unreachable!("placeholder"),
            }


        }

        for assembly_instruction in &assembly_instructions {
            println!("{:?}", assembly_instruction);
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn first() {
        let input = "
            main var a, b, c, d; {
                let a <- 1 + 2;  
                let b <- a - 2; 
            }.
        "
        .to_string();

        let mut parser = Parser::new(input);

        parser.parse_computation();


        let mut bbg = &parser.internal_program.get_curr_fn().bb_graph;
        let mut bbg = bbg.clone(); 

        let mut bruh = CodeGeneration::new(&mut bbg);
        bruh.generate_code();

    }
}
