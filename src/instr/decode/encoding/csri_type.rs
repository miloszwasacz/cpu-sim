use super::EncodingFormat;
use crate::components::cpu::csr::CsrAddr;
use crate::components::cpu::reg::RegName;
use crate::instr::decode::shared::{decode_csr, decode_rd};
use crate::instr::decode::Decode;
use crate::instr::raw::RawInstr;

use std::fmt;

pub type CsrImmediate = u8;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CsriTypeFormat {
    rd: RegName,
    imm: CsrImmediate,
    csr: CsrAddr,
}

impl CsriTypeFormat {
    #[inline(always)]
    pub fn rd(&self) -> RegName {
        self.rd
    }

    #[inline(always)]
    pub fn imm(&self) -> CsrImmediate {
        self.imm
    }

    #[inline(always)]
    pub fn csr(&self) -> CsrAddr {
        self.csr
    }

    fn decode_imm(instr: RawInstr) -> CsrImmediate {
        const LEN: u64 = 5;
        const MSB: u64 = 19;
        instr.extract_bits::<LEN, MSB>().as_u64() as _
    }
}

impl Decode for CsriTypeFormat {
    fn decode(raw: RawInstr) -> Self {
        let rd = decode_rd(raw);
        let imm = Self::decode_imm(raw);
        let csr = decode_csr(raw);

        Self { rd, imm, csr }
    }
}

impl CsriTypeFormat {}

impl fmt::Display for CsriTypeFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.rd.fmt(f)?;
        write!(f, ", ")?;
        if f.alternate() {
            write!(f, "{:>4}", self.imm)?;
        } else {
            write!(f, "{}", self.imm)?;
        }
        write!(f, ", ")?;
        self.csr.fmt(f)
    }
}

impl EncodingFormat for CsriTypeFormat {}
