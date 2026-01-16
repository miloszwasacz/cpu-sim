//! Type definitions, utilities, and decoding logic for RISC-V instructions.

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

/// A trait defining an instruction.
/// 
/// It should be implemented by all supported instructions,
/// e.g. [`Addi`], [`Bge`], [`Ld`], etc.
pub trait Instr: fmt::Debug + fmt::Display + Into<Instruction> {}

/// An immediate value held by an instruction.
pub type Immediate = i64;

/// An enum collecting all instructions into a single type, 
/// based on the execution unit that should handle it.
#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    /// An instruction to be processed by an ALU.
    Alu {
        ctrl: AluControl,
        src1: AluSrcA,
        src2: AluSrcB,
        dest: RegName,
    },
    /// An instruction to be processed by a MUL unit.
    Mul {
        ctrl: MulControl,
        src1: RegName,
        src2: RegName,
        dest: RegName,
    },
    /// An unconditional jump.
    Jump {
        base: AluSrcA,
        offset: Immediate,
        apply_mask: bool,
        link_reg: RegName,
    },
    /// A conditional jump (branch).
    Branch {
        ctrl: Branch,
        src1: RegName,
        src2: RegName,
        offset: Immediate,
    },
    /// A load instruction.
    Load {
        load: MemRead,
        byte_count: usize,
        base: RegName,
        offset: Immediate,
        dest: RegName,
    },
    /// A store instruction.
    Store {
        store: MemWrite,
        byte_count: usize,
        src: RegName,
        base: RegName,
        offset: Immediate,
    },
    /// An environment trap.
    EnvTrap(EnvTrap),
    //TODO Improve fence granularity
    /// A memory fence.
    Fence,
    /// An instruction accessing CSRs.
    Csr {
        ctrl: CsrControl,
        csr: CsrAddr,
        src: CsrSrc,
        dest: RegName,
    },
}

/// The first source of an ALU instruction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AluSrcA {
    Reg(RegName),
    Pc,
}

/// The second source of an ALU instruction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AluSrcB {
    Reg(RegName),
    Imm(Immediate),
}

/// The comparison performed by a branch instruction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Branch {
    /// Equal
    Eq,
    /// Not equal
    Ne,
    /// Less than
    Lt,
    /// Less than (unsigned)
    Ltu,
    /// Greater than or equal
    Ge,
    /// Greater than or equal (unsigned)
    Geu,
}

impl Branch {
    /// Tests whether the condition is met the two source registers, 
    /// and the jump should be performed.
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

/// The environment trap type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnvTrap {
    /// Environment call (change privilege mode).
    Ecall,
    /// Breakpoint exception
    Ebreak,
    /// Exception.
    Exception(Exception),
}

/// The source for modifying a CSR.
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

/// Operation to perform when accessing a CSR.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CsrControl {
    /// Atomic Read/Write
    Rw,
    /// Atomic Read and Set Bits
    Rs,
    /// Atomic Read and Clear Bits
    Rc,
}

/// Utilities for printing instructions.
pub(crate) mod display {
    #[allow(unused_imports)]
    use super::*;
    use crate::include_generated;

    /// The width that should be reserved when pretty-printing 
    pub(crate) const DISPLAY_PRETTY_WIDTH: usize = 9;

    /// Returns the width that should be reserved for printing an instruction,
    /// based on the `formatter` state.
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
