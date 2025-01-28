use crate::instr::{Instr, RawInstr};

mod br_eg_si;
mod dp_imm;
mod dp_reg;
mod ldr_str;

macro_rules! invalid_instr {
    () => {
        panic!("invalid or unsupported instruction")
    };
}
use invalid_instr;

macro_rules! extract_bits {
    ($instr:expr, $shift:expr, $mask:expr) => {{
        ($instr >> $shift) & $mask
    }};
}
use extract_bits;

macro_rules! decode {
        (
            $instr:expr,
            $shift:expr,
            $mask:expr,
            match {
                $( $pat:pat => $module:ident, )*
            }
        ) => {{
            let group = extract_bits!($instr, $shift, $mask);
            match group {
                $( $pat => $module ::decode($instr), )*
                _ => invalid_instr!(),
            }
        }};
    }
use decode;

const REG_MASK: u32 = 0b11111;

const OP1_SHIFT: u32 = 25;
const OP1_MASK: u32 = 0b1111;

pub struct DecodeStage;

impl DecodeStage {
    pub fn decode(&self, instr: RawInstr) -> Box<dyn Instr> {
        let instr = instr.encoded();
        decode!(instr, OP1_SHIFT, OP1_MASK, match {
            0b1000 | 0b1001 => dp_imm,
            0b1010 | 0b1011 => br_eg_si,
            0b0101 | 0b1101 => dp_reg,
            0b0100 | 0b0110 | 0b1100 | 0b1110 => ldr_str,
        })
    }
}

pub(crate) fn sign_extend(uimm: u32, size: u32) -> i64 {
    let shift: u32 = 32 - size;
    let simm = ((uimm << shift) as i32) as i64;
    simm >> shift
}

pub(crate) fn ones(count: u32) -> u64 {
    let mut result = 0;
    for _ in 0..count {
        result <<= 1;
        result |= 1;
    }
    result
}

pub(crate) fn ror(x: u64, size: u32, shift: u32) -> u64 {
    if shift == 0 { return x; }
    assert!(shift < 256);
    let m = shift % size;
    let mask = ones(size);
    (x >> m) | (x << (size - m)) & mask
}

pub(crate) fn replicate(x: u64, m: u32, n: u32) -> u64 {
    if n == 1 {
        return x;
    }
    let mut result = 0;
    for _ in 0..n {
        result <<= m;
        result |= x;
    }
    result
}
