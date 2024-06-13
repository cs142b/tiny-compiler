use crate::code_gen::AssemblyInstruction;
use crate::code_gen::Fmt; 

type MachineCodeInstruction = u32; 

type MachineCodeInstructions = Vec<u32>;


fn convert_assembly_to_opcode(asm: AssemblyInstruction) -> MachineCodeInstruction {
    let a = asm.get_a(); 
    let b = asm.get_b();
    let c = asm.get_c(); 

    let a = a.unwrap_or(0); 
    let b = b.unwrap_or(0); 
    let c = c.unwrap_or(0); 
    let op_code = asm.get_op_code();
    let fmt = asm.get_fmt();

    let mut ret: MachineCodeInstruction = 0; 
    match fmt {
        Fmt::F1 => {
            // 6op 5reg 5reg 16reg 
            

            let lower_16 = get_lower_n_bits(c, 16);
            let mut middle = get_lower_n_bits(b, 5);
            middle = middle << 16; 
            let mut upper = get_lower_n_bits(a, 5); 
            upper = upper << (16 + 5); 
            let mut op = get_lower_n_bits(op_code as u32, 6);
            op = op << (16 + 5 + 5);

            return lower_16 + middle + upper + op; 
            



        },
        Fmt::F2 => {
            //6op 5reg 5reg 11none 5c
            let lower_5 = get_lower_n_bits(c, 5);
            let mut middle = get_lower_n_bits(b, 5);
            middle = middle << 16; 
            let mut upper = get_lower_n_bits(a, 5); 
            upper = upper << (16 + 5); 
            let mut op = get_lower_n_bits(op_code as u32, 6);
            op = op << (16 + 5 + 5);

            return lower_5 + middle + upper + op; 
            

            


        },
        Fmt::F3 => {
            //6op 26c
            
            let lower_26= get_lower_n_bits(c, 26);
            
            let mut op = get_lower_n_bits(op_code as u32, 6);
            op = op << (26);

            return op + lower_26;

            
        }
    }

    return 0
}

fn get_lower_n_bits(num: u32, n: u32) -> u32 {
    num & ((1 << n) - 1)
}



