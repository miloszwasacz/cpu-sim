use super::{AddressingMode, LdrStr, LdrStrOp};
use crate::instr::Instr;
use crate::reg::RegisterSize;

//#region LDRB (immediate)

pub struct Ldrb(LdrStr<u8>);

impl Ldrb {
    pub fn new(t: u32, n: u32, imm: u32, addr_mode: AddressingMode) -> Self {
        Self(LdrStr::new_imm(LdrStrOp::Ldr, (t, RegisterSize::W), n, imm, addr_mode))
    }
}

impl Instr for Ldrb {}

//#endregion

//#region LDRSB (immediate)

pub struct Ldrsb(LdrStr<i8>);

impl Ldrsb {
    pub fn new(t: u32, n: u32, imm: u32, opc: u32, addr_mode: AddressingMode) -> Self {
        let t_size = match opc {
            0b11 => RegisterSize::W,
            0b10 => RegisterSize::X,
            opc => panic!("{opc} is not a valid opc encoding"),
        };
        Self(LdrStr::new_imm(LdrStrOp::Ldr, (t, t_size), n, imm, addr_mode))
    }
}

impl Instr for Ldrsb {}

//#endregion

//#region LDRH (immediate)

pub struct Ldrh(LdrStr<u16>);

impl Ldrh {
    pub fn new(t: u32, n: u32, imm: u32, addr_mode: AddressingMode) -> Self {
        Self(LdrStr::new_imm(LdrStrOp::Ldr, (t, RegisterSize::W), n, imm, addr_mode))
    }
}

impl Instr for Ldrh {}

//#endregion

//#region LDRSH (immediate)

pub struct Ldrsh(LdrStr<i16>);

impl Ldrsh {
    pub fn new(t: u32, n: u32, imm: u32, opc: u32, addr_mode: AddressingMode) -> Self {
        let t_size = match opc {
            0b11 => RegisterSize::W,
            0b10 => RegisterSize::X,
            opc => panic!("{opc} is not a valid opc encoding"),
        };
        Self(LdrStr::new_imm(LdrStrOp::Ldr, (t, t_size), n, imm, addr_mode))
    }
}

impl Instr for Ldrsh {}

//#endregion

//#region LDR (immediate)

pub struct Ldr(LdrSize);

enum LdrSize {
    Word(LdrStr<u32>),
    DoubleWord(LdrStr<u64>),
}

impl Ldr {
    pub fn new(t: u32, n: u32, imm: u32, size: u32, addr_mode: AddressingMode) -> Self {
        Self(match size {
            0b10 => LdrSize::Word(LdrStr::new_imm(
                LdrStrOp::Ldr,
                (t, RegisterSize::W),
                n,
                imm,
                addr_mode,
            )),
            0b11 => LdrSize::DoubleWord(LdrStr::new_imm(
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

//#endregion

//#region LDRSW (immediate)

pub struct Ldrsw(LdrStr<i32>);

impl Ldrsw {
    pub fn new(t: u32, n: u32, imm: u32, addr_mode: AddressingMode) -> Self {
        Self(LdrStr::new_imm(LdrStrOp::Ldr, (t, RegisterSize::X), n, imm, addr_mode))
    }
}

impl Instr for Ldrsw {}

//#endregion

//#region STRB (immediate)

pub struct Strb(LdrStr<u8>);

impl Strb {
    pub fn new(t: u32, n: u32, imm: u32, addr_mode: AddressingMode) -> Self {
        Self(LdrStr::new_imm(LdrStrOp::Str, (t, RegisterSize::W), n, imm, addr_mode))
    }
}

impl Instr for Strb {}

//#endregion

//#region STRH (immediate)

pub struct Strh(LdrStr<u16>);

impl Strh {
    pub fn new(t: u32, n: u32, imm: u32, addr_mode: AddressingMode) -> Self {
        Self(LdrStr::new_imm(LdrStrOp::Str, (t, RegisterSize::W), n, imm, addr_mode))
    }
}

impl Instr for Strh {}

//#endregion

//#region STR (immediate)

pub struct Str(StrSize);

enum StrSize {
    Word(LdrStr<u32>),
    DoubleWord(LdrStr<u64>)
}

impl Str {
    pub fn new(t: u32, n: u32, imm: u32, size: u32, addr_mode: AddressingMode) -> Self {
        Self(match size {
            0b10 => StrSize::Word(LdrStr::new_imm(
                LdrStrOp::Str,
                (t, RegisterSize::W),
                n,
                imm,
                addr_mode,
            )),
            0b11 => StrSize::DoubleWord(LdrStr::new_imm(
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

//#endregion