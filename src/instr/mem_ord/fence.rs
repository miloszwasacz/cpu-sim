use crate::components::cpu::alu::AluControl;
use crate::components::cpu::reg::RegName;
use crate::instr::decode::ITypeFormat;
use crate::instr::display::display_width;
use crate::instr::execute::{AluSrcB, ExecUnit};
use crate::instr::issue::Branch;
use crate::instr::mem_access::{MemRead, MemWrite};
use crate::instr::raw::RawInstr;
use crate::instr::{Decode, Execute, Immediate, Issue, MemoryAccess, Writeback};

use cpu_sim_derive::Instr;
use std::fmt;

// TODO Properly implement `Issue`, `Execute`, `MemoryAccess` and `Writeback` for `Fence`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Instr)]
pub struct Fence {
    fm: u32,
    pi: bool,
    po: bool,
    pr: bool,
    pw: bool,
    si: bool,
    so: bool,
    sr: bool,
    sw: bool,
}

impl Fence {
    const FENCE_TSO_DISPLAY_NAME: &'static str = "fence.tso";
    const PAUSE_DISPLAY_NAME: &'static str = "pause";
}

impl Decode for Fence {
    fn decode(raw: RawInstr) -> Self {
        const FLAG_SHIFT: u32 = 1;
        const FLAG_MASK: Immediate = 0b1;
        const FM_MASK: Immediate = 0b1111;

        let format = ITypeFormat::decode(raw);
        let rd = format.rd();
        let rs1 = format.rs1();
        let mut imm = format.imm();
        assert!(
            rd.is_zero(),
            "{} should have `rd` set to `{}`",
            Self::DISPLAY_NAME,
            RegName::ZERO
        );
        assert!(
            rs1.is_zero(),
            "{} should have `rs1` set to `{}`",
            Self::DISPLAY_NAME,
            RegName::ZERO
        );

        let sw = imm & FLAG_MASK == 0b1;
        imm >>= FLAG_SHIFT;
        let sr = imm & FLAG_MASK == 0b1;
        imm >>= FLAG_SHIFT;
        let so = imm & FLAG_MASK == 0b1;
        imm >>= FLAG_SHIFT;
        let si = imm & FLAG_MASK == 0b1;
        imm >>= FLAG_SHIFT;
        let pw = imm & FLAG_MASK == 0b1;
        imm >>= FLAG_SHIFT;
        let pr = imm & FLAG_MASK == 0b1;
        imm >>= FLAG_SHIFT;
        let po = imm & FLAG_MASK == 0b1;
        imm >>= FLAG_SHIFT;
        let pi = imm & FLAG_MASK == 0b1;
        imm >>= FLAG_SHIFT;
        let fm = (imm & FM_MASK) as u32;

        Self {
            fm,
            pi,
            po,
            pr,
            pw,
            si,
            so,
            sr,
            sw,
        }
    }
}

impl Issue for Fence {
    fn branch(&self) -> Branch {
        todo!()
    }
}

impl Execute for Fence {
    fn exec_unit(&self) -> ExecUnit {
        todo!()
    }

    fn alu_src_b(&self) -> AluSrcB {
        todo!()
    }

    fn alu_control(&self) -> AluControl {
        todo!()
    }
}

impl MemoryAccess for Fence {
    fn mem_read(&self) -> Option<MemRead> {
        todo!()
    }

    fn mem_write(&self) -> Option<MemWrite> {
        todo!()
    }
}

impl Writeback for Fence {
    fn reg_write(&self) -> bool {
        todo!()
    }
}

impl fmt::Display for Fence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let width = display_width!(f);
        match (
            self.fm, self.pi, self.po, self.pr, self.pw, self.si, self.so, self.sr, self.sw,
        ) {
            (0b1000, false, false, true, true, false, false, true, true) => {
                write!(f, "{:<width$}", Self::FENCE_TSO_DISPLAY_NAME)
            }
            (0b0000, false, false, false, true, false, false, false, false) => {
                write!(f, "{:<width$}", Self::PAUSE_DISPLAY_NAME)
            }
            _ => {
                macro_rules! flag {
                    ($value:expr, $flag_name:literal) => {
                        if $value { $flag_name } else { "" }
                    };
                }

                let pr = flag!(self.pr, "r");
                let pw = flag!(self.pw, "w");
                let pi = flag!(self.pi, "i");
                let po = flag!(self.po, "o");

                let sr = flag!(self.sr, "r");
                let sw = flag!(self.sw, "r");
                let si = flag!(self.si, "i");
                let so = flag!(self.so, "o");

                write!(
                    f,
                    "{:<width$} {pr}{pw}{pi}{po}, {sr}{sw}{si}{so}",
                    Self::DISPLAY_NAME
                )
            }
        }
    }
}
