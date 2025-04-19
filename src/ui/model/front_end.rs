use cpu_sim::components::diagnostics::cpu::{Exception, IdIsRegs, IfIdRegs, Instruction};
use cpu_sim::components::memory::Address;
use cpu_sim::instr::raw::RawInstr;

pub type Fetched = Result<RawInstr, Exception>;
pub type Decoded = Result<Instruction, Exception>;

pub struct FrontEndModel {
    pc: Address,
    fetched: Option<Fetched>,
    decoded: Option<Decoded>,
}

impl FrontEndModel {
    pub(super) fn new(pc: Address, if_id_regs: Option<IfIdRegs>, id_is_regs: Option<IdIsRegs>) -> Self {
        let fetched = if_id_regs.map(|regs| regs.instr);
        let decoded = id_is_regs.map(|regs| regs.instr.map(|(_, i)| i));
        Self {
            pc,
            fetched,
            decoded,
        }
    }

    pub fn pc(&self) -> Address {
        self.pc
    }

    pub fn fetched(&self) -> Option<Fetched> {
        self.fetched
    }

    pub fn decoded(&self) -> Option<Decoded> {
        self.decoded
    }
}
