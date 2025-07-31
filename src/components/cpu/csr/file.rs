use self::logical::CSR_INFO;
pub(super) use self::logical::{Csr, CsrAccess};
pub use self::physical::*;
use super::addr::map::CsrAddrMap;
use super::{CsrAddr, CsrData, WriteAccess};
use crate::components::cpu::error::CsrError;
use crate::components::cpu::flip_flop::Sequential;
use crate::components::cpu::reg::{RegData, RegName};
use crate::include_generated;
use crate::instr::{CsrControl, CsrSrc};

use std::collections::HashMap;

pub mod diagnostics;
mod logical;
mod physical;

//TODO Side-effects
#[derive(Debug, Clone, PartialEq)]
pub enum CsrAccessResult {
    NotRead {
        // side_effects: Vec<_>,
    },
    Read {
        data: RegData,
        // side_effects: Vec<_>,
    },
    Exception(CsrError),
}

//TODO Add examples to docs
/// Register file for CSRs.
#[derive(Debug)]
pub struct CsrFile(HashMap<CsrAddr, PhysicalCsr>);

impl CsrFile {
    const INVALID_ALIAS_MESSAGE: &'static str =
        "logical CSR should be associated with an implemented physical CSR";

    pub fn new() -> Self {
        let regs = include_generated!("csr_physical_map.rs" as expr);
        Self(HashMap::from(regs))
    }

    fn get(&self, addr: CsrAddr) -> &PhysicalCsr {
        self.0.get(&addr).expect(Self::INVALID_ALIAS_MESSAGE)
    }

    fn get_mut(&mut self, addr: CsrAddr) -> &mut PhysicalCsr {
        self.0.get_mut(&addr).expect(Self::INVALID_ALIAS_MESSAGE)
    }

    //TODO Document errors
    /// Performs explicit atomic access to a CSR at the provided `addr`.
    /// The current value is read and returned, and the provided `value`
    /// is written according to `ctrl`.
    pub fn access(
        &mut self,
        addr: CsrAddr,
        ctrl: CsrControl,
        src: CsrSrc,
        value: RegData,
        dest: RegName,
    ) -> CsrAccessResult {
        use CsrAccessResult::Exception;
        use CsrControl as Ctrl;
        use CsrSrc as Src;

        let value = value.u();
        let info = match CSR_INFO.get(addr) {
            Some(info) => info,
            None => return Exception(CsrError::InvalidCsr(addr)),
        };
        let physical = self.get_mut(info.physical_addr);
        //TODO Access control (privilege level, write access)

        let read_raw = physical.read();
        let read_data = (info.read_aliasing)(read_raw);
        let write_data = match ctrl {
            Ctrl::Rw => value,
            Ctrl::Rs => read_data | value,
            Ctrl::Rc => read_data & !value,
        };

        let reads = !(dest.is_zero() && ctrl == Ctrl::Rw);
        let writes = !(matches!(ctrl, Ctrl::Rs | Ctrl::Rc)
            && matches!(src, Src::Reg(RegName::ZERO) | Src::Imm(0)));

        if writes {
            let write_data = match (info.write_aliasing)(read_raw, write_data) {
                Ok(data) => data,
                Err(err) => return Exception(err),
            };
            physical.explicit_write(write_data);
        }

        if reads {
            let data = RegData::unsigned(read_data);
            CsrAccessResult::Read { data }
        } else {
            CsrAccessResult::NotRead {}
        }
    }

    pub fn implicit_read(&self, addr: CsrAddr) -> CsrData {
        let info = CSR_INFO
            .get(addr)
            .unwrap_or_else(|| panic!("CSR with address {addr} is not implemented"));
        let physical = self.get(addr);
        (info.read_aliasing)(physical.read())
    }

    pub fn implicit_write(&mut self, addr: CsrAddr, data: CsrData) {
        let info = CSR_INFO
            .get(addr)
            .unwrap_or_else(|| panic!("CSR with address {addr} is not implemented"));
        let physical = self.get_mut(addr);
        let data = (info.write_aliasing)(physical.read(), data).unwrap_or_else(|err| {
            let data = (info.format)(data);
            panic!("implicit CSR write of invalid value ({data}): {err}")
        });
        debug_assert_ne!(
            info.access_ctrl().write_access,
            WriteAccess::ReadOnly,
            "implicit CSR write to a read-only CSR with address {addr}"
        );
        physical.implicit_write(data);
    }
}

impl Default for CsrFile {
    fn default() -> Self {
        Self::new()
    }
}

impl Sequential for CsrFile {
    fn finish_cycle(&mut self) {
        for reg in self.0.values_mut() {
            reg.finish_cycle();
        }
    }
}
