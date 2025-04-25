use cpu_sim::components::diagnostics::cpu::{BpRegs, IfRegs, ZbpRegs};
use cpu_sim::components::memory::Address;

pub struct FrontEndModel {
    pc: Address,
    zbp_regs: Box<[ZbpRegs]>,
    if_regs: Box<[IfRegs]>,
    bp_regs: Box<[BpRegs]>,
    decode_width: usize,
}

impl FrontEndModel {
    pub(super) fn new(
        pc: Address,
        zbp_regs: Box<[ZbpRegs]>,
        if_regs: Box<[IfRegs]>,
        bp_regs: Box<[BpRegs]>,
        decode_width: usize,
    ) -> Self {
        Self {
            pc,
            zbp_regs,
            if_regs,
            bp_regs,
            decode_width,
        }
    }

    pub fn pc(&self) -> Address {
        self.pc
    }

    pub fn zbp_regs(&self) -> &[ZbpRegs] {
        &self.zbp_regs
    }

    pub fn if_regs(&self) -> &[IfRegs] {
        &self.if_regs
    }

    pub fn bp_regs(&self) -> &[BpRegs] {
        &self.bp_regs
    }

    pub fn decode_width(&self) -> usize {
        self.decode_width
    }
}
