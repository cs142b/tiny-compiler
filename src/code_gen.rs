use crate::instruction::{Operation, Instruction};
use crate::register_allocation::color_graph;
use crate::live_analysis::*;
use crate::cfg_traversal::*;
use crate::parser::*;
use crate::register_allocation::*;
use std::collections::HashMap;
use petgraph::graph::NodeIndex;

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

impl AssemblyInstruction {
    fn update(&mut self, new_value2: isize) {
        match self {
            AssemblyInstruction::BEQ(_, ref mut value2) => {
                *value2 = new_value2;
            },
            AssemblyInstruction::BNE(_, ref mut value2) => {
                *value2 = new_value2;
            },
            AssemblyInstruction::BLT(_, ref mut value2) => {
                *value2 = new_value2;
            },
            AssemblyInstruction::BGE(_, ref mut value2) => {
                *value2 = new_value2;
            },
            AssemblyInstruction::BLE(_, ref mut value2) => {
                *value2 = new_value2;
            },
            AssemblyInstruction::BGT(_, ref mut value2) => {
                *value2 = new_value2;
            },
            _ => {
                panic!("yaaa add more shit later");
            }
        }
    }
}

type AssemblyIndex = usize;

pub struct CodeGeneration {
    instructions: Vec<Instruction>,
    original_graph: BasicBlockGraph,
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
        
        let mut graph3 = graph.clone();

        Self {
            original_graph: graph3,
            instructions,
            register_mapping,
            assembly_instructions: Vec::new(),
            line_number_to_assembly_map: HashMap::new(),
        }
    }

    pub fn generate_code(&mut self) {
        let mut waiting_to_be_mapped: HashMap<LineNumber, LineNumber> = HashMap::new(); // the line
        // number that needs to be fined and the line number thats waiting to update
        println!("MAPPING OF REGISTERS");
        for (line_number, register_num) in &self.register_mapping {
            println!("Line({}): R{}", line_number, register_num);

        }
        for instruction in &self.instructions {

            // update any waiting instructions
            let line_number = instruction.get_line_number();
            let operation = *instruction.get_operation_ref();

            if operation == Operation::Empty {
                continue;
            }

            if operation == Operation::End {
                self.assembly_instructions.push(AssemblyInstruction::RET(0));
                break;
            }

            println!("{}", line_number);

            match operation {
                Operation::Add(value1, value2) => {
                    let line_num_register = *self.register_mapping.get(&line_number).unwrap();
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
                    let line_num_register = *self.register_mapping.get(&line_number).unwrap();
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
                    let line_num_register = *self.register_mapping.get(&line_number).unwrap();
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
                    let line_num_register = *self.register_mapping.get(&line_number).unwrap();
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
                    let line_num_register = *self.register_mapping.get(&line_number).unwrap();
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

                Operation::Cmp(value1, value2) => {
                    let line_num_register = *self.register_mapping.get(&line_number).unwrap();
                    if value1 <= 0 && value2 <= 0 {
                        let value1_register = *self.register_mapping.get(&value1).unwrap(); 
                        self.assembly_instructions.push(AssemblyInstruction::ADDI(value1_register, 0, -value1));
                        self.assembly_instructions.push(AssemblyInstruction::CMPI(line_num_register, value1_register, -value2));

                    }
                    if value1 > 0 && value2 > 0 {
                        let value1_register = *self.register_mapping.get(&value1).unwrap(); 
                        let value2_register = *self.register_mapping.get(&value2).unwrap();
                        self.assembly_instructions.push(AssemblyInstruction::CMP(line_num_register, value1_register, value2_register));

                    }
                    if value1 <= 0 {
                        let value2_register = *self.register_mapping.get(&value2).unwrap(); 
                        self.assembly_instructions.push(AssemblyInstruction::CMPI(line_num_register, value2_register, -value1));
                    }
                    if value2 <= 0 {
                        let value1_register = *self.register_mapping.get(&value1).unwrap();
                        self.assembly_instructions.push(AssemblyInstruction::DIVI(line_num_register, value1_register, -value2));

                    }
                },
                Operation::Beq(comparison_line_number, block_index) => {
                    let comparison_line_number_register = *self.register_mapping.get(&comparison_line_number).unwrap();
                    self.assembly_instructions.push(AssemblyInstruction::BEQ(comparison_line_number_register, 0)); // 0 is a BS value
                    // get first instruction of block_index
                    let first_instruction = self.original_graph.node_weight(NodeIndex::from(block_index as u32)).unwrap().instructions.first().unwrap();
                    let first_instruction_line = first_instruction.get_line_number();
                    waiting_to_be_mapped.insert(first_instruction_line, line_number);
                },
                Operation::Bne(comparison_line_number, block_index) => {
                    let comparison_line_number_register = *self.register_mapping.get(&comparison_line_number).unwrap();
                    self.assembly_instructions.push(AssemblyInstruction::BNE(comparison_line_number_register, 0)); // 0 is a BS value
                    // get first instruction of block_index
                    let first_instruction = self.original_graph.node_weight(NodeIndex::from(block_index as u32)).unwrap().instructions.first().unwrap();
                    let first_instruction_line = first_instruction.get_line_number();
                    waiting_to_be_mapped.insert(first_instruction_line, line_number);
                },
                Operation::Blt(comparison_line_number, block_index) => {
                    let comparison_line_number_register = *self.register_mapping.get(&comparison_line_number).unwrap();
                    self.assembly_instructions.push(AssemblyInstruction::BLT(comparison_line_number_register, 0)); // 0 is a BS value
                    // get first instruction of block_index
                    let first_instruction = self.original_graph.node_weight(NodeIndex::from(block_index as u32)).unwrap().instructions.first().unwrap();
                    let first_instruction_line = first_instruction.get_line_number();
                    waiting_to_be_mapped.insert(first_instruction_line, line_number);
                },
                Operation::Bge(comparison_line_number, block_index) => {
                    let comparison_line_number_register = *self.register_mapping.get(&comparison_line_number).unwrap();
                    self.assembly_instructions.push(AssemblyInstruction::BGE(comparison_line_number_register, 0)); // 0 is a BS value
                    // get first instruction of block_index
                    let first_instruction = self.original_graph.node_weight(NodeIndex::from(block_index as u32)).unwrap().instructions.first().unwrap();
                    let first_instruction_line = first_instruction.get_line_number();
                    waiting_to_be_mapped.insert(first_instruction_line, line_number);
                },
                Operation::Ble(comparison_line_number, block_index) => {
                    let comparison_line_number_register = *self.register_mapping.get(&comparison_line_number).unwrap();
                    self.assembly_instructions.push(AssemblyInstruction::BLE(comparison_line_number_register, 0)); // 0 is a BS value
                    // get first instruction of block_index
                    let first_instruction = self.original_graph.node_weight(NodeIndex::from(block_index as u32)).unwrap().instructions.first().unwrap();
                    let first_instruction_line = first_instruction.get_line_number();
                    waiting_to_be_mapped.insert(first_instruction_line, line_number);
                },
                Operation::Bgt(comparison_line_number, block_index) => {
                    let comparison_line_number_register = *self.register_mapping.get(&comparison_line_number).unwrap();
                    self.assembly_instructions.push(AssemblyInstruction::BGT(comparison_line_number_register, 0)); // 0 is a BS value
                    // get first instruction of block_index
                    let first_instruction = self.original_graph.node_weight(NodeIndex::from(block_index as u32)).unwrap().instructions.first().unwrap();
                    let first_instruction_line = first_instruction.get_line_number();
                    waiting_to_be_mapped.insert(first_instruction_line, line_number);
                },
                _ => unreachable!("placeholder"),
            }
            
            self.line_number_to_assembly_map.insert(line_number, self.assembly_instructions.len() - 1);
            
            // update any waiting instructions ( the magic happens here for branching )
            if waiting_to_be_mapped.contains_key(&line_number) {
                let new_index = self.assembly_instructions.len() - 1;
                let original_index = self.find_instruction_index_in_vector_given_line(line_number);
                self.assembly_instructions[original_index].update((new_index - original_index) as isize);
                waiting_to_be_mapped.remove(&line_number);
            }
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
                if 1 < 2 then 
                    let c <- 100 + 2;
                fi;
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
