use crate::components::cpu::reg::arf::ArchRegName;
use crate::components::cpu::reg::RegData;
use crate::components::memory::Address;
use crate::instr::decode::ITypeFormat;
use crate::instr::execute::ExecuteResult;
use crate::instr::{display_width, Immediate, Instr};

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Jalr(pub(in crate::instr) ITypeFormat);

impl Jalr {
    const JR_DISPLAY_NAME: &'static str = "jr";

    pub fn dest(&self) -> ArchRegName {
        self.0.rd
    }

    pub fn base(&self) -> ArchRegName {
        self.0.rs1
    }

    pub fn offset(&self) -> Immediate {
        self.0.imm
    }
}

impl Instr for Jalr {}

impl_execute!(Jalr, |&self, reg_file, pc, alu, _, _| {
    const MASK: RegData = RegData::MAX << 1;
    let base = reg_file.get(self.base());
    let target = alu.add(base.get(), self.offset()) & MASK;
    let link = pc.read_next();
    Ok(ExecuteResult::Jump(target as Address, self.dest(), link))
});

impl_mem_access!(Jalr);

impl fmt::Display for Jalr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let width = display_width!(f);
        let is_jr = self.dest().is_zero() && self.offset() == 0;

        let name = if is_jr {
            Self::JR_DISPLAY_NAME
        } else {
            Self::DISPLAY_NAME
        };
        write!(f, "{:<width$} ", name)?;
        if !is_jr {
            self.dest().fmt(f)?;
            write!(f, ", ")?;
        }

        self.base().fmt(f)?;

        if !is_jr {
            write!(f, ", ")?;
            self.offset().fmt(f)?;
        }

        Ok(())
    }
}
