use crate::components::cpu::reg::arf::ArchRegName;
use crate::components::cpu::reg::RegData;
use crate::components::memory::Address;
use crate::instr::decode::JTypeFormat;
use crate::instr::execute::ExecuteResult;
use crate::instr::{display_width, Immediate, Instr};

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Jal(pub(in crate::instr) JTypeFormat);

impl Jal {
    const J_DISPLAY_NAME: &'static str = "j";

    pub fn dest(&self) -> ArchRegName {
        self.0 .0.rd
    }

    pub fn offset(&self) -> Immediate {
        self.0 .0.imm
    }
}

impl Instr for Jal {}

impl_execute!(Jal, |&self, _, pc, alu, _, _| {
    let target = alu.add(pc.read() as RegData, self.offset());
    let link = pc.read_next();
    pc.write(target as Address);
    Ok(ExecuteResult::Alu(self.dest(), link as RegData))
});

impl_mem_access!(Jal);

impl fmt::Display for Jal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let width = display_width!(f);
        let is_j = self.dest().is_zero();

        let name = if is_j {
            Self::J_DISPLAY_NAME
        } else {
            Self::DISPLAY_NAME
        };
        write!(f, "{:<width$} ", name)?;
        if is_j {
            self.offset().fmt(f)
        } else {
            self.0.fmt(f)
        }
    }
}
