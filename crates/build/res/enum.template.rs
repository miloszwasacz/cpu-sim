/// An enum collecting all supported instructions.
/// 
/// It should only be used for diagnositcs -- for a type that is used 
/// for execution, see [`crate::instr::Instruction`].
#[derive(Debug)]
pub(crate) enum FullInstruction {
    #INSTRS!
}

impl std::fmt::Display for FullInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            #DISPLAYS!
        }
    }
}

impl From<FullInstruction> for Instruction {
    fn from(value: FullInstruction) -> Self {
        match value {
            #FROMS!
        }
    }
}