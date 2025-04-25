//! Pipeline registers
//TODO Improve docs

use crate::components::cpu::error::Exception;
use crate::components::cpu::front_end::PcPlus4;
use crate::components::cpu::Pc;
use crate::instr::raw::RawInstr;

#[derive(Debug, Clone, Copy)]
pub struct ZbpRegs {
    pub pc: Pc,
    pub pc_plus_4: Pc,
    pub predicted: Option<Pc>,
}

#[derive(Debug, Clone, Copy)]
pub struct IfRegs {
    pub instr: Result<RawInstr, Exception>,
    pub pc: Pc,
    pub pc_plus_4: Pc,
    pub predicted: Option<Pc>,
}

#[derive(Debug, Clone, Copy)]
pub enum Prediction {
    NotTaken,
    TakenUnknown,
    Taken(Pc),
}

#[derive(Debug, Clone, Copy)]
pub struct BpRegs {
    pub instr: Result<RawInstr, Exception>,
    pub pc: Pc,
    pub pc_plus_4: PcPlus4,
    pub predicted: Prediction,
}
