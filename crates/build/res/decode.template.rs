pub fn decode(instr: RawInstr) -> Result<Box<dyn Instr>, RawInstr> {
    match instr.encoding() {
        #SPECIAL!
        _ => {}
    }
    let opcode: RawInstrBits = decode_opcode(instr).into();
    let funct3: RawInstrBits = decode_funct3(instr).into();
    let funct7: RawInstrBits = decode_funct7(instr).into();
    Ok(match (opcode, funct3, funct7) {
        #ENCODINGS!
        _ => return Err(instr),
    })
}
