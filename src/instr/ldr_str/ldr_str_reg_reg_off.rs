use crate::instr::ldr_str::{LdrStr, LdrStrOp};
use crate::instr::Instr;
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

//#endregion

//#region LDRSB (register)

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

//#endregion

//#region LDRH (register)

pub struct Ldrh(LdrStr<u16>);

impl Ldrh {
    pub fn new(t: u32, n: u32, m: u32, option: u32, s: u32) -> Self {
        let m_size = reg_size_from_option(option); 
        let shift = if s == 1 { 1 } else { 0 };
        Self(LdrStr::new_reg(LdrStrOp::Ldr, (t, RegisterSize::W), n, (m, m_size), option, shift))
    }
}

impl Instr for Ldrh {}

//#endregion

//#region LDRSH (register)

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

//#endregion

//#region LDR (register)

pub struct Ldr(LdrSize);

enum LdrSize {
    Word(LdrStr<u32>),
    DoubleWord(LdrStr<u64>),
}

impl Ldr {
    pub fn new(t: u32, n: u32, m: u32, option: u32, s: u32, size: u32) -> Self {
        let m_size = reg_size_from_option(option);
        let scale = size;
        let shift = if s == 1 { scale } else { 0 };
        Self(match size {
            0b10 => LdrSize::Word(LdrStr::new_reg(
                LdrStrOp::Ldr,
                (t, RegisterSize::W),
                n,
                (m, m_size),
                option,
                shift,
            )),
            0b11 => LdrSize::DoubleWord(LdrStr::new_reg(
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

//#endregion

//#region LDRSW (register)

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

//#endregion

//#region STRB (register)

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

//#endregion

//#region STRH (register)

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

//#endregion

//#region STR (register)

pub struct Str(StrSize);

enum StrSize {
    Word(LdrStr<u32>),
    DoubleWord(LdrStr<u64>),
}

impl Str {
    pub fn new(t: u32, n: u32, m: u32, option: u32, s: u32, size: u32) -> Self {
        let m_size = reg_size_from_option(option);
        let scale = size;
        let shift = if s == 1 { scale } else { 0 };
        Self(match size {
            0b10 => StrSize::Word(LdrStr::new_reg(
                LdrStrOp::Str,
                (t, RegisterSize::W),
                n,
                (m, m_size),
                option,
                shift,
            )),
            0b11 => StrSize::DoubleWord(LdrStr::new_reg(
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

//#endregion
