pub(super) use self::info::CSR_INFO;
use super::*;
use crate::components::cpu::csr::CsrAccessCtrl;

use std::fmt;

mod info {
    use super::*;
    #[allow(unused_imports)]
    use crate::components::cpu::csr::addr::map::csr_addr_map;
    #[allow(unused_imports)]
    use crate::components::cpu::csr::defs::*;

    /// A static table containing metadata about [CSRs](Csr).
    ///
    /// In particular, it encodes aliasing rules when reading and writing to CSRs
    /// (see [`CsrInfo::read_aliasing`] and [`CsrInfo::write_aliasing`]).
    /// In real hardware, this would be achieved through explicit hardwired connections.
    pub static CSR_INFO: CsrAddrMap<CsrInfo> = csr_addr_map!("csr_vtable.rs");
}

type ReadAliasing = fn(CsrData) -> CsrData;
type WriteAliasing = fn(CsrData, CsrData) -> Result<CsrData, CsrError>;
type FormatFn = fn(CsrData) -> String;

/// Metadata about a CSR.
#[derive(Debug, Clone, Copy)]
pub(super) struct CsrInfo {
    /// The name of the CSR.
    pub name: &'static str,
    /// The address of the CSR.
    pub logical_addr: CsrAddr,
    /// The address of the underlying physical CSR.
    ///
    /// This address only differs from the `logical_addr` if this register
    /// is just an alias of (a part of) another CSR.
    pub physical_addr: CsrAddr,
    /// A function used to transform data returned by the physical CSR when read
    /// (see [`CsrAccess::alias_read`]).
    pub read_aliasing: ReadAliasing,
    /// A function used to transform data that will be provided to the physical
    /// CSR to write (see [`CsrAccess::alias_write`]).
    pub write_aliasing: WriteAliasing,
    /// A function used to format [`CsrData`] according to field specification 
    /// of the given CSR (see [`CsrAccess::format_data`]).
    pub format: FormatFn,
}

impl CsrInfo {
    /// Creates a new entry for [`CSR_INFO`] for the given `CSR`.
    const fn new<CSR: Csr>() -> Self {
        Self {
            name: CSR::NAME,
            logical_addr: CSR::ADDRESS,
            physical_addr: CSR::PHYSICAL_ADDRESS,
            read_aliasing: CSR::alias_read,
            write_aliasing: CSR::alias_write,
            format: CSR::format_data,
        }
    }

    /// Returns the [`PrivilegeLevel`] and [`WriteAccess`] of the given CSR.
    #[inline]
    pub fn access_ctrl(&self) -> CsrAccessCtrl {
        self.logical_addr.into()
    }
}

/// A trait specifying a CSR defined in the ISA that can be either
/// a physical CSR, or just an alias of (a part of) another CSR.
pub(in crate::components::cpu::csr) trait Csr: CsrAccess {
    /// The name of the CSR as defined in the ISA.
    const NAME: &'static str;

    /// The address of the CSR as defined in the ISA.
    const ADDRESS: CsrAddr;

    /// The address of the underlying physical CSR.
    /// This address only differs from the [`Csr::ADDRESS`]
    /// if this register is just an alias of (a part of) another CSR.
    const PHYSICAL_ADDRESS: CsrAddr = Self::ADDRESS;
}

/// A trait defining how a CSR is accessed.
pub(in crate::components::cpu::csr) trait CsrAccess {
    //TODO Add example (fflags and fcsr)
    /// Transforms data returned by the physical CSR when read.
    fn alias_read(data: CsrData) -> CsrData {
        data
    }

    //TODO Add example (fflags and fcsr)
    /// Transforms data that will be provided to the physical CSR to write.
    ///
    /// The `current` data is in raw, unaliased form (see [`CsrAccess::alias_read`]).
    #[allow(unused_variables)]
    fn alias_write(current: CsrData, new: CsrData) -> Result<CsrData, CsrError> {
        Ok(new)
    }

    //TODO Add example
    /// Formats and writes `data` to the provided formatter according to field specification.
    ///
    /// The `data` should be in raw, unaliased form (see [`CsrAccess::alias_read`]).
    fn display_data(f: &mut dyn fmt::Write, data: CsrData) -> fmt::Result;

    //TODO Add example
    /// Formats `data` according to field specification.
    ///
    /// The `data` should be in raw, unaliased form (see [`CsrAccess::alias_read`]).
    fn format_data(data: CsrData) -> String {
        let mut formatted = String::new();
        Self::display_data(&mut formatted, data).expect("writing to string should be infallible");
        formatted
    }
}
