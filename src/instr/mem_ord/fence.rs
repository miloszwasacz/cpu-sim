use crate::components::cpu::reg::arf::ArchRegName;
use crate::instr::decode::ITypeFormat;
use crate::instr::stall::StallControl;
use crate::instr::{display_width, Immediate, Instr};

use std::collections::HashSet;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    const TSO_DISPLAY_NAME: &'static str = "fence.tso";
    const PAUSE_DISPLAY_NAME: &'static str = "pause";
}

impl Instr for Fence {}

// TODO Properly implement `Execute`, `MemoryAccess` and `StallControl` for `Fence`
impl_execute!(Fence, |&self, _, _, _, _, _| { todo!() });
impl_mem_access!(Fence);

impl StallControl for Fence {
    fn read_regs(&self) -> HashSet<ArchRegName> {
        todo!()
    }

    fn write_reg(&self) -> Option<ArchRegName> {
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
                write!(f, "{:<width$}", Self::TSO_DISPLAY_NAME)
            }
            (0b0000, false, false, false, true, false, false, false, false) => {
                write!(f, "{:<width$}", Self::PAUSE_DISPLAY_NAME)
            }
            _ => {
                macro_rules! flag {
                    ($value:expr, $flag_name:literal) => {
                        if $value {
                            $flag_name
                        } else {
                            ""
                        }
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

impl From<ITypeFormat> for Fence {
    fn from(value: ITypeFormat) -> Self {
        const FLAG_SHIFT: u32 = 1;
        const FLAG_MASK: Immediate = 0b1;
        const FM_MASK: Immediate = 0b1111;

        let ITypeFormat { rd, rs1, mut imm } = value;
        assert!(rd.is_zero());
        assert!(rs1.is_zero());

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
