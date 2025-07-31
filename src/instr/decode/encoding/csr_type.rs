use super::EncodingFormat;
use crate::components::cpu::csr::CsrAddr;
use crate::components::cpu::reg::RegName;
use crate::instr::decode::shared::{decode_csr, decode_rd, decode_rs1};
use crate::instr::decode::Decode;
use crate::instr::raw::RawInstr;

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CsrTypeFormat {
    rd: RegName,
    rs1: RegName,
    csr: CsrAddr,
}

impl CsrTypeFormat {
    #[inline(always)]
    pub fn rd(&self) -> RegName {
        self.rd
    }

    #[inline(always)]
    pub fn rs1(&self) -> RegName {
        self.rs1
    }

    #[inline(always)]
    pub fn csr(&self) -> CsrAddr {
        self.csr
    }
}

impl Decode for CsrTypeFormat {
    fn decode(raw: RawInstr) -> Self {
        let rd = decode_rd(raw);
        let rs1 = decode_rs1(raw);
        let csr = decode_csr(raw);

        Self { rd, rs1, csr }
    }
}

impl fmt::Display for CsrTypeFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.rd.fmt(f)?;
        write!(f, ", ")?;
        self.rs1.fmt(f)?;
        write!(f, ", ")?;
        self.csr.fmt(f)
    }
}

impl EncodingFormat for CsrTypeFormat {}
