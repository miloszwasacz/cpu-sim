use super::{LdrStr, LdrStrOp, LdrStrSize};
use crate::instr::{impl_display, Instr};
use crate::reg::RegisterSize;

fn reg_size_from_option(option: u32) -> RegisterSize {
    match option & 0b1 {
        0b0 => RegisterSize::W,
        0b1 => RegisterSize::X,
        _ => unreachable!(),
    }
}

fn byte_m_size(option: u32) -> RegisterSize {
    match option {
        0b011 => RegisterSize::X,
        option => reg_size_from_option(option),
    }
}

//#region LDRB (register)

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ldrb(LdrStr<u8>);

impl Ldrb {
    pub fn new(t: u32, n: u32, m: u32, option: u32) -> Self {
        let m_size = byte_m_size(option);
        Self(LdrStr::new_reg(
            LdrStrOp::Ldr,
            (t, RegisterSize::W),
            n,
            (m, m_size),
            option,
            0,
        ))
    }
}

impl Instr for Ldrb {}

impl_display!(Ldrb, |self| &self.0);

//#endregion

//#region LDRSB (register)

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ldrsb(LdrStr<i8>);

impl Ldrsb {
    pub fn new(t: u32, n: u32, m: u32, opc: u32, option: u32) -> Self {
        let t_size = match opc {
            0b11 => RegisterSize::W,
            0b10 => RegisterSize::X,
            opc => panic!("{opc} is not a valid opc encoding"),
        };
        let m_size = byte_m_size(option);
        Self(LdrStr::new_reg(
            LdrStrOp::Ldr,
            (t, t_size),
            n,
            (m, m_size),
            option,
            0,
        ))
    }
}

impl Instr for Ldrsb {}

impl_display!(Ldrsb, |self| &self.0);

//#endregion

//#region LDRH (register)

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ldrh(LdrStr<u16>);

impl Ldrh {
    pub fn new(t: u32, n: u32, m: u32, option: u32, s: u32) -> Self {
        let m_size = reg_size_from_option(option);
        let shift = if s == 1 { 1 } else { 0 };
        Self(LdrStr::new_reg(
            LdrStrOp::Ldr,
            (t, RegisterSize::W),
            n,
            (m, m_size),
            option,
            shift,
        ))
    }
}

impl Instr for Ldrh {}

impl_display!(Ldrh, |self| &self.0);

//#endregion

//#region LDRSH (register)

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ldrsh(LdrStr<i16>);

impl Ldrsh {
    pub fn new(t: u32, n: u32, m: u32, opc: u32, option: u32, s: u32) -> Self {
        let t_size = match opc {
            0b11 => RegisterSize::W,
            0b10 => RegisterSize::X,
            opc => panic!("{opc} is not a valid opc encoding"),
        };
        let m_size = byte_m_size(option);
        let shift = if s == 1 { 1 } else { 0 };
        Self(LdrStr::new_reg(
            LdrStrOp::Ldr,
            (t, t_size),
            n,
            (m, m_size),
            option,
            shift,
        ))
    }
}

impl Instr for Ldrsh {}

impl_display!(Ldrsh, |self| &self.0);

//#endregion

//#region LDR (register)

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ldr(LdrStrSize);

impl Ldr {
    pub fn new(t: u32, n: u32, m: u32, option: u32, s: u32, size: u32) -> Self {
        let m_size = reg_size_from_option(option);
        let scale = size;
        let shift = if s == 1 { scale } else { 0 };
        Self(match size {
            0b10 => LdrStrSize::Word(LdrStr::new_reg(
                LdrStrOp::Ldr,
                (t, RegisterSize::W),
                n,
                (m, m_size),
                option,
                shift,
            )),
            0b11 => LdrStrSize::DoubleWord(LdrStr::new_reg(
                LdrStrOp::Ldr,
                (t, RegisterSize::X),
                n,
                (m, m_size),
                option,
                shift,
            )),
            size => panic!("{size} is not a valid size encoding"),
        })
    }
}

impl Instr for Ldr {}

impl_display!(Ldr, |self| &self.0);

//#endregion

//#region LDRSW (register)

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ldrsw(LdrStr<i32>);

impl Ldrsw {
    pub fn new(t: u32, n: u32, m: u32, option: u32, s: u32) -> Self {
        let m_size = byte_m_size(option);
        let shift = if s == 1 { 2 } else { 0 };
        Self(LdrStr::new_reg(
            LdrStrOp::Ldr,
            (t, RegisterSize::X),
            n,
            (m, m_size),
            option,
            shift,
        ))
    }
}

impl Instr for Ldrsw {}

impl_display!(Ldrsw, |self| &self.0);

//#endregion

//#region STRB (register)

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Strb(LdrStr<u8>);

impl Strb {
    pub fn new(t: u32, n: u32, m: u32, option: u32) -> Self {
        let m_size = byte_m_size(option);
        Self(LdrStr::new_reg(
            LdrStrOp::Str,
            (t, RegisterSize::W),
            n,
            (m, m_size),
            option,
            0,
        ))
    }
}

impl Instr for Strb {}

impl_display!(Strb, |self| &self.0);

//#endregion

//#region STRH (register)

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Strh(LdrStr<u16>);

impl Strh {
    pub fn new(t: u32, n: u32, m: u32, option: u32, s: u32) -> Self {
        let m_size = reg_size_from_option(option);
        let shift = if s == 1 { 1 } else { 0 };
        Self(LdrStr::new_reg(
            LdrStrOp::Str,
            (t, RegisterSize::W),
            n,
            (m, m_size),
            option,
            shift,
        ))
    }
}

impl Instr for Strh {}

impl_display!(Strh, |self| &self.0);

//#endregion

//#region STR (register)

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Str(LdrStrSize);

impl Str {
    pub fn new(t: u32, n: u32, m: u32, option: u32, s: u32, size: u32) -> Self {
        let m_size = reg_size_from_option(option);
        let scale = size;
        let shift = if s == 1 { scale } else { 0 };
        Self(match size {
            0b10 => LdrStrSize::Word(LdrStr::new_reg(
                LdrStrOp::Str,
                (t, RegisterSize::W),
                n,
                (m, m_size),
                option,
                shift,
            )),
            0b11 => LdrStrSize::DoubleWord(LdrStr::new_reg(
                LdrStrOp::Str,
                (t, RegisterSize::X),
                n,
                (m, m_size),
                option,
                shift,
            )),
            size => panic!("{size} is not a valid size encoding"),
        })
    }
}

impl Instr for Str {}

impl_display!(Str, |self| &self.0);

//#endregion
