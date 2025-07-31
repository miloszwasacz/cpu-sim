use super::{PrivilegeLevel, WriteAccess};
use crate::include_generated;
use crate::instr::raw::Bits;

use std::fmt;
use std::ops::Shr;

pub mod map;

type CsrAddrRepr = u16;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct CsrAddr(CsrAddrRepr);

impl CsrAddr {
    pub(crate) const MSB: u64 = 31;
    pub(crate) const LEN: u64 = 12;

    /// The width when the address is printed out (in hexadecimal format).
    const PRINT_WIDTH: usize = (Self::LEN as usize) / 4 + "0x".len();

    pub fn privilege_level(&self) -> PrivilegeLevel {
        (*self).into()
    }

    pub fn write_access(&self) -> WriteAccess {
        (*self).into()
    }
}

impl fmt::Display for CsrAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#0width$X}", self.0, width = Self::PRINT_WIDTH)
    }
}

impl From<Bits<{ CsrAddr::LEN }>> for CsrAddr {
    fn from(value: Bits<{ CsrAddr::LEN }>) -> Self {
        Self(value.as_u64() as _)
    }
}

impl Shr<CsrAddrRepr> for CsrAddr {
    type Output = CsrAddrRepr;

    fn shr(self, rhs: CsrAddrRepr) -> Self::Output {
        self.0 >> rhs
    }
}

include_generated!("csr_addr_consts.rs");
