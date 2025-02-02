pub fn decode(instr: RawInstr) -> Box<dyn Instr> {
    #SPECIAL!
    let opcode = decode_opcode(instr);
    match opcode {
        x if #OPCODES_r! => {
            let funct3 = decode_funct3(instr);
            let funct7 = decode_funct7(instr);
            match (opcode, funct3, funct7) {
                #DECODE_r!
                _ => invalid_instr!(instr),
            }
        }
        x if #OPCODES_i! ||
        #OPCODES_i_shift! ||
        #OPCODES_s! ||
        #OPCODES_b! => {
            let funct3 = decode_funct3(instr);
            let shift_type = decode_shift_type(instr);
            match (opcode, funct3) {
                #DECODE_i_shift!
                #DECODE_i!
                #DECODE_s!
                #DECODE_b!
                _ => invalid_instr!(instr),
            }
        }
        #DECODE_u!
        #DECODE_j!
        _ => invalid_instr!(instr),
    }
}