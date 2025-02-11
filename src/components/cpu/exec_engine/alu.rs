use crate::components::cpu::reg::{RegData, RegDataUnsigned};

pub struct Alu(());

impl Alu {
    const SHIFT_MASK: RegData = 0b11111;

    pub(super) fn new() -> Self {
        Self(())
    }

    pub fn add(&self, a: RegData, b: RegData) -> RegData {
        a.wrapping_add(b)
    }

    pub fn sub(&self, a: RegData, b: RegData) -> RegData {
        a.wrapping_sub(b)
    }

    pub fn and(&self, a: RegData, b: RegData) -> RegData {
        a & b
    }

    pub fn or(&self, a: RegData, b: RegData) -> RegData {
        a | b
    }

    pub fn xor(&self, a: RegData, b: RegData) -> RegData {
        a ^ b
    }

    pub fn sll(&self, a: RegData, b: RegData) -> RegData {
        let sh = b & Self::SHIFT_MASK;
        let d = (a as RegDataUnsigned) << sh;
        d as RegData
    }

    pub fn srl(&self, a: RegData, b: RegData) -> RegData {
        let sh = b & Self::SHIFT_MASK;
        let d = a as RegDataUnsigned >> sh;
        d as RegData
    }

    pub fn sra(&self, a: RegData, b: RegData) -> RegData {
        let sh = b & Self::SHIFT_MASK as RegData;
        a >> sh
    }

    pub fn slt(&self, a: RegData, b: RegData) -> RegData {
        if a < b {
            1
        } else {
            0
        }
    }

    pub fn sltu(&self, a: RegData, b: RegData) -> RegData {
        let a = a as RegDataUnsigned;
        let b = b as RegDataUnsigned;
        if a < b {
            1
        } else {
            0
        }
    }
}
