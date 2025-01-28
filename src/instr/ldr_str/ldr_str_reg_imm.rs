use super::{AddressingMode, LdrStr, LdrStrOp, LdrStrSize};
use crate::instr::{impl_display, Instr};
use crate::reg::RegisterSize;

//#region LDRB (immediate)

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ldrb(LdrStr<u8>);

impl Ldrb {
    pub fn new(t: u32, n: u32, imm: u32, addr_mode: AddressingMode) -> Self {
        Self(LdrStr::new_imm(
            LdrStrOp::Ldr,
            (t, RegisterSize::W),
            n,
            imm,
            addr_mode,
        ))
    }
}

impl Instr for Ldrb {}

impl_display!(Ldrb, |self| &self.0);

//#endregion

//#region LDRSB (immediate)

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ldrsb(LdrStr<i8>);

impl Ldrsb {
    pub fn new(t: u32, n: u32, imm: u32, opc: u32, addr_mode: AddressingMode) -> Self {
        let t_size = match opc {
            0b11 => RegisterSize::W,
            0b10 => RegisterSize::X,
            opc => panic!("{opc} is not a valid opc encoding"),
        };
        Self(LdrStr::new_imm(
            LdrStrOp::Ldr,
            (t, t_size),
            n,
            imm,
            addr_mode,
        ))
    }
}

impl Instr for Ldrsb {}

impl_display!(Ldrsb, |self| &self.0);

//#endregion

//#region LDRH (immediate)

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ldrh(LdrStr<u16>);

impl Ldrh {
    pub fn new(t: u32, n: u32, imm: u32, addr_mode: AddressingMode) -> Self {
        Self(LdrStr::new_imm(
            LdrStrOp::Ldr,
            (t, RegisterSize::W),
            n,
            imm,
            addr_mode,
        ))
    }
}

impl Instr for Ldrh {}

impl_display!(Ldrh, |self| &self.0);

//#endregion

//#region LDRSH (immediate)

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ldrsh(LdrStr<i16>);

impl Ldrsh {
    pub fn new(t: u32, n: u32, imm: u32, opc: u32, addr_mode: AddressingMode) -> Self {
        let t_size = match opc {
            0b11 => RegisterSize::W,
            0b10 => RegisterSize::X,
            opc => panic!("{opc} is not a valid opc encoding"),
        };
        Self(LdrStr::new_imm(
            LdrStrOp::Ldr,
            (t, t_size),
            n,
            imm,
            addr_mode,
        ))
    }
}

impl Instr for Ldrsh {}

impl_display!(Ldrsh, |self| &self.0);

//#endregion

//#region LDR (immediate)

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ldr(LdrStrSize);

impl Ldr {
    pub fn new(t: u32, n: u32, imm: u32, size: u32, addr_mode: AddressingMode) -> Self {
        Self(match size {
            0b10 => LdrStrSize::Word(LdrStr::new_imm(
                LdrStrOp::Ldr,
                (t, RegisterSize::W),
                n,
                imm,
                addr_mode,
            )),
            0b11 => LdrStrSize::DoubleWord(LdrStr::new_imm(
                LdrStrOp::Ldr,
                (t, RegisterSize::X),
                n,
                imm,
                addr_mode,
            )),
            size => panic!("{size} is not a valid size encoding"),
        })
    }
}

impl Instr for Ldr {}

impl_display!(Ldr, |self| &self.0);

//#endregion

//#region LDRSW (immediate)

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ldrsw(LdrStr<i32>);

impl Ldrsw {
    pub fn new(t: u32, n: u32, imm: u32, addr_mode: AddressingMode) -> Self {
        Self(LdrStr::new_imm(
            LdrStrOp::Ldr,
            (t, RegisterSize::X),
            n,
            imm,
            addr_mode,
        ))
    }
}

impl Instr for Ldrsw {}

impl_display!(Ldrsw, |self| &self.0);

//#endregion

//#region STRB (immediate)

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Strb(LdrStr<u8>);

impl Strb {
    pub fn new(t: u32, n: u32, imm: u32, addr_mode: AddressingMode) -> Self {
        Self(LdrStr::new_imm(
            LdrStrOp::Str,
            (t, RegisterSize::W),
            n,
            imm,
            addr_mode,
        ))
    }
}

impl Instr for Strb {}

impl_display!(Strb, |self| &self.0);

//#endregion

//#region STRH (immediate)

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Strh(LdrStr<u16>);

impl Strh {
    pub fn new(t: u32, n: u32, imm: u32, addr_mode: AddressingMode) -> Self {
        Self(LdrStr::new_imm(
            LdrStrOp::Str,
            (t, RegisterSize::W),
            n,
            imm,
            addr_mode,
        ))
    }
}

impl Instr for Strh {}

impl_display!(Strh, |self| &self.0);

//#endregion

//#region STR (immediate)

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Str(LdrStrSize);

impl Str {
    pub fn new(t: u32, n: u32, imm: u32, size: u32, addr_mode: AddressingMode) -> Self {
        Self(match size {
            0b10 => LdrStrSize::Word(LdrStr::new_imm(
                LdrStrOp::Str,
                (t, RegisterSize::W),
                n,
                imm,
                addr_mode,
            )),
            0b11 => LdrStrSize::DoubleWord(LdrStr::new_imm(
                LdrStrOp::Str,
                (t, RegisterSize::X),
                n,
                imm,
                addr_mode,
            )),
            size => panic!("{size} is not a valid size encoding"),
        })
    }
}

impl Instr for Str {}

impl_display!(Str, |self| &self.0);

//#endregion
