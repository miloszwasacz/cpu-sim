use super::zicsr_ext_instr;
use crate::instr::CsrControl;

zicsr_ext_instr!(Csrrs, Reg, CsrControl::Rs);
