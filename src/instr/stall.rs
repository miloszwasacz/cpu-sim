use super::decode::ITypeFormat;
use super::{Addi, Instr};
use crate::components::cpu::reg::arf::ArchRegName;

use std::collections::HashSet;
use std::rc::Rc;

pub trait StallControl {
    fn read_regs(&self) -> HashSet<ArchRegName>;
    fn write_reg(&self) -> Option<ArchRegName>;
}

pub(crate) fn nop() -> Rc<dyn Instr> {
    Rc::new(Addi::from(ITypeFormat {
        rd: ArchRegName::ZERO,
        rs1: ArchRegName::ZERO,
        imm: 0,
    }))
}

pub(super) fn read_regs_from_iter(
    iter: impl IntoIterator<Item = ArchRegName>,
) -> HashSet<ArchRegName> {
    iter.into_iter().filter(|reg| !reg.is_zero()).collect()
}

pub(super) fn write_reg(reg: ArchRegName) -> Option<ArchRegName> {
    if reg.is_zero() {
        None
    } else {
        Some(reg)
    }
}

mod generated {
    #[allow(unused_imports)]
    use super::*;
    use crate::include_generated;
    #[allow(unused_imports)]
    use crate::instr::*;

    include_generated!("stall_control_impls.rs");
}
