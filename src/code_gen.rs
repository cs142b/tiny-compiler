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

type AssemblyIndex = usize;

pub struct CodeGeneration {
    instructions: Vec<Instruction>,
    register_mapping: HashMap<LineNumber, RegisterNumber>,
    assembly_instructions: Vec<AssemblyInstruction>,
    line_number_to_assembly_map: HashMap<LineNumber, AssemblyIndex>,
}

impl CodeGeneration {
    pub fn new(graph: &mut BasicBlockGraph) -> Self {
        let graph1 = graph.clone();
        let new_graph = get_interference_graph(&graph1);
        let cluster_possibilities = get_clusters(&graph1);
        let (new_upgraded_graph, line_node_map) = get_graph_and_map(&new_graph, &cluster_possibilities);
        
        let register_mapping = generate_register_mapping(&new_upgraded_graph);

        let mut graph2 = graph.clone();
        let instructions = traverse_in_order(&mut graph2);

        Self {
            instructions,
            register_mapping,
            assembly_instructions: Vec::new(),
            line_number_to_assembly_map: HashMap::new(),
        }
    }

    pub fn generate_code(&mut self) {
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
                    if value1 <= 0 && value2 <= 0 {
                        self.assembly_instructions.push(AssemblyInstruction::ADDI(line_num_register, 0, -value1));
                        self.assembly_instructions.push(AssemblyInstruction::ADDI(line_num_register, 0, -value2));
                    }
                    
                    if value1 > 0 && value2 > 0 {
                        let value1_register = *self.register_mapping.get(&value1).unwrap(); 
                        let value2_register = *self.register_mapping.get(&value2).unwrap();
                        self.assembly_instructions.push(AssemblyInstruction::ADD(line_num_register, value1_register, value2_register));

                    }
                    
                    if value1 <= 0 {
                        let constant = -value1;
                        let value2_register = *self.register_mapping.get(&value2).unwrap();
                        self.assembly_instructions.push(AssemblyInstruction::ADDI(line_num_register, value2_register, constant));
                    }
                    
                    if value2 <= 0 {
                        let constant = -value2;
                        let value1_register = *self.register_mapping.get(&value1).unwrap();
                        self.assembly_instructions.push(AssemblyInstruction::ADDI(line_num_register, value1_register, constant));
                    }
                },
                Operation::Sub(value1, value2) => {
                    if value1 <= 0 && value2 <= 0 {
                        self.assembly_instructions.push(AssemblyInstruction::SUBI(line_num_register, 0, -value1));
                        self.assembly_instructions.push(AssemblyInstruction::SUBI(line_num_register, 0, -value2));

                    }

                    if value1 > 0 && value2 > 0 {
                        let value1_register = *self.register_mapping.get(&value1).unwrap(); 
                        let value2_register = *self.register_mapping.get(&value2).unwrap();
                        self.assembly_instructions.push(AssemblyInstruction::SUB(line_num_register, value1_register, value2_register));

                    }
                    
                    if value1 <= 0 {
                        let constant = -value1;
                        let value2_register = *self.register_mapping.get(&value2).unwrap();
                        self.assembly_instructions.push(AssemblyInstruction::SUBI(line_num_register, value2_register, constant));
                    }
                    
                    if value2 <= 0 {
                        let constant = -value2;
                        let value1_register = *self.register_mapping.get(&value1).unwrap();
                        self.assembly_instructions.push(AssemblyInstruction::SUBI(line_num_register, value1_register, constant));
                    }
                },
                Operation::Mul(value1, value2) => {
                    if value1 <= 0 && value2 <= 0 {
                        self.assembly_instructions.push(AssemblyInstruction::MULI(line_num_register, 0, -value1));
                        self.assembly_instructions.push(AssemblyInstruction::MULI(line_num_register, 0, -value2));

                    }
                    
                    if value1 > 0 && value2 > 0 {
                        let value1_register = *self.register_mapping.get(&value1).unwrap(); 
                        let value2_register = *self.register_mapping.get(&value2).unwrap();
                        self.assembly_instructions.push(AssemblyInstruction::MUL(line_num_register, value1_register, value2_register));

                    }
                    
                    if value1 <= 0 {
                        let constant = -value1;
                        let value2_register = *self.register_mapping.get(&value2).unwrap();
                        self.assembly_instructions.push(AssemblyInstruction::MULI(line_num_register, value2_register, constant));
                    }

                    if value2 <= 0 {
                        let constant = -value2;
                        let value1_register = *self.register_mapping.get(&value1).unwrap();
                        self.assembly_instructions.push(AssemblyInstruction::MULI(line_num_register, value1_register, constant));

                    }
                },
                Operation::Div(value1, value2) => {
                    if value1 <= 0 && value2 <= 0 {
                        self.assembly_instructions.push(AssemblyInstruction::DIVI(line_num_register, 0, -value1));
                        self.assembly_instructions.push(AssemblyInstruction::DIVI(line_num_register, 0, -value2));

                    }
                    if value1 > 0 && value2 > 0 {
                        let value1_register = *self.register_mapping.get(&value1).unwrap(); 
                        let value2_register = *self.register_mapping.get(&value2).unwrap();
                        self.assembly_instructions.push(AssemblyInstruction::DIV(line_num_register, value1_register, value2_register));

                    }
                    if value1 <= 0 {
                        let constant = -value1;
                        let value2_register = *self.register_mapping.get(&value2).unwrap();
                        self.assembly_instructions.push(AssemblyInstruction::DIVI(line_num_register, value2_register, constant));
                    }
                    if value2 <= 0 {
                        let constant = -value2;
                        let value1_register = *self.register_mapping.get(&value1).unwrap();
                        self.assembly_instructions.push(AssemblyInstruction::DIVI(line_num_register, value1_register, constant));

                    }
                },
                Operation::Phi(value1, value2) => {
                    let value1_register = *self.register_mapping.get(&value1).unwrap();
                    let value2_register = *self.register_mapping.get(&value2).unwrap();

                    if line_num_register != value1_register {
                        let index_to_insert = self.find_instruction_index_in_vector_given_line(value1_register as isize);
                        self.assembly_instructions.insert(index_to_insert, AssemblyInstruction::ADD(line_num_register, value1_register, 0));

                    }

                    if line_num_register != value2_register {
                        let index_to_insert = self.find_instruction_index_in_vector_given_line(value2_register as isize);
                        self.assembly_instructions.insert(index_to_insert, AssemblyInstruction::ADD(line_num_register, value2_register, 0));
                    }


                },
                _ => unreachable!("placeholder"),
            }
            
            self.line_number_to_assembly_map.insert(line_number, self.assembly_instructions.len() - 1);
        }

        println!("MAPPING OF REGISTERS");
        for (line_number, register_num) in &self.register_mapping {
            println!("Line({}): R{}", line_number, register_num);

        }
        
        println!("ASSEMBLY INSTRUCTIONS");
        for assembly_instruction in &self.assembly_instructions {
            println!("{:?}", assembly_instruction);
        }
    }

    fn find_instruction_index_in_vector_given_line(&self, line_number: isize) -> AssemblyIndex {
        *self.line_number_to_assembly_map.get(&line_number).unwrap()
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dot_viz::generate_dot_viz;
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

        println!("{}", generate_dot_viz("main", &parser.internal_program));

        let mut bbg = &parser.internal_program.get_curr_fn().bb_graph;
        let mut bbg = bbg.clone(); 

        let mut bruh = CodeGeneration::new(&mut bbg);
        bruh.generate_code();


    }
}
