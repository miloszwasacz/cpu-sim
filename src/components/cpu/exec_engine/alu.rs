use crate::components::cpu::reg::RegData;

pub(super) struct Alu(());

impl Alu {
    const SHIFT_MASK: u32 = 0b11111;

    pub fn new() -> Self {
        Self(())
    }

    pub fn process(&self, op: AluControl, src_a: RegData, src_b: RegData) -> RegData {
        match op {
            AluControl::Add => RegData::signed(src_a.i().wrapping_add(src_b.i())),
            AluControl::Sub => RegData::signed(src_a.i().wrapping_sub(src_b.i())),
            AluControl::And => RegData::unsigned(src_a.u() & src_b.u()),
            AluControl::Or => RegData::unsigned(src_a.u() | src_b.u()),
            AluControl::Xor => RegData::unsigned(src_a.u() ^ src_b.u()),
            AluControl::Sll => {
                let sh = src_b.u() & Self::SHIFT_MASK;
                RegData::unsigned(src_a.u() << sh)
            }
            AluControl::Srl => {
                let sh = src_b.u() & Self::SHIFT_MASK;
                RegData::unsigned(src_a.u() >> sh)
            }
            AluControl::Sra => {
                let sh = src_b.u() & Self::SHIFT_MASK;
                RegData::signed(src_a.i() >> sh)
            }
            AluControl::Slt => RegData::signed(if src_a.i() < src_b.i() { 1 } else { 0 }),
            AluControl::Sltu => RegData::unsigned(if src_a.u() < src_b.u() { 1 } else { 0 }),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum AluControl {
    #[default]
    Add,
    Sub,
    And,
    Or,
    Xor,
    Sll,
    Srl,
    Sra,
    Slt,
    Sltu,
}
