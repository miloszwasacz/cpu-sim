use crate::components::cpu::reg::RegData;
use crate::instr::Branch;

pub(in crate::components::cpu) struct BranchUnit;

impl BranchUnit {
    pub fn branch(&mut self, cmp: Branch, src1: RegData, src2: RegData) -> bool {
        cmp.jumps(src1, src2)
    }
}
