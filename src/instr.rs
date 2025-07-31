pub use self::decode::encoding::CsrImmediate;
use self::mem_access::{MemRead, MemWrite};
use crate::components::cpu::csr::CsrAddr;
use crate::components::cpu::error::Exception;
use crate::components::cpu::reg::{RegData, RegName};
use crate::components::cpu::{AluControl, MulControl};
use crate::export_mod as instr_mod;

use std::fmt;

instr_mod!(ctrl_trans);
instr_mod!(env_call);
instr_mod!(int_comput);
instr_mod!(ld_str);
instr_mod!(mem_ord);
instr_mod!(zicsr_ext);
instr_mod!(m_ext);

pub mod decode;
pub(crate) mod full;
pub mod mem_access;
pub mod raw;

pub trait Instr: fmt::Debug + fmt::Display + Into<Instruction> {}

pub type Immediate = i64;

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    Alu {
        ctrl: AluControl,
        src1: AluSrcA,
        src2: AluSrcB,
        dest: RegName,
    },
    Mul {
        ctrl: MulControl,
        src1: RegName,
        src2: RegName,
        dest: RegName,
    },
    Jump {
        base: AluSrcA,
        offset: Immediate,
        apply_mask: bool,
        link_reg: RegName,
    },
    Branch {
        ctrl: Branch,
        src1: RegName,
        src2: RegName,
        offset: Immediate,
    },
    Load {
        load: MemRead,
        byte_count: usize,
        base: RegName,
        offset: Immediate,
        dest: RegName,
    },
    Store {
        store: MemWrite,
        byte_count: usize,
        src: RegName,
        base: RegName,
        offset: Immediate,
    },
    EnvTrap(EnvTrap),
    //TODO Improve fence granularity
    Fence,
    Csr {
        ctrl: CsrControl,
        csr: CsrAddr,
        src: CsrSrc,
        dest: RegName,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AluSrcA {
    Reg(RegName),
    Pc,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AluSrcB {
    Reg(RegName),
    Imm(Immediate),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Branch {
    Eq,
    Ne,
    Lt,
    Ltu,
    Ge,
    Geu,
}

impl Branch {
    pub fn jumps(&self, src1: RegData, src2: RegData) -> bool {
        match self {
            Branch::Eq => src1.i() == src2.i(),
            Branch::Ne => src1.i() != src2.i(),
            Branch::Lt => src1.i() < src2.i(),
            Branch::Ltu => src1.u() < src2.u(),
            Branch::Ge => src1.i() >= src2.i(),
            Branch::Geu => src1.u() >= src2.u(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnvTrap {
    Ecall,
    Ebreak,
    Exception(Exception),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CsrSrc {
    Reg(RegName),
    Imm(CsrImmediate),
}

impl fmt::Display for CsrSrc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CsrSrc::Reg(reg) => fmt::Display::fmt(reg, f),
            CsrSrc::Imm(imm) => fmt::Display::fmt(imm, f),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CsrControl {
    /// Atomic Read/Write
    Rw,
    /// Atomic Read and Set Bits
    Rs,
    /// Atomic Read and Clear Bits
    Rc,
}

pub(crate) mod display {
    #[allow(unused_imports)]
    use super::*;
    use crate::include_generated;

    pub(crate) const DISPLAY_PRETTY_WIDTH: usize = 9;

    macro_rules! display_width {
        ($formatter:expr) => {
            if $formatter.alternate() {
                crate::instr::display::DISPLAY_PRETTY_WIDTH
            } else {
                0
            }
        };
    }
    pub(crate) use display_width;

    include_generated!("display_name_impls.rs");
}
