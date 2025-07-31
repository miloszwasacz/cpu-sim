use crate::components::cpu::csr::CsrAddr;

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrivilegeLevel {
    Unprivileged,
    Supervisor,
    Hypervisor,
    Machine,
}

impl From<CsrAddr> for PrivilegeLevel {
    fn from(value: CsrAddr) -> Self {
        const SHIFT: u16 = 8;
        const MASK: u16 = 0b11;

        let value = (value >> SHIFT) & MASK;
        match value {
            0b00 => Self::Unprivileged,
            0b01 => Self::Supervisor,
            0b10 => Self::Hypervisor,
            0b11 => Self::Machine,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WriteAccess {
    ReadOnly,
    ReadWrite,
}

impl From<CsrAddr> for WriteAccess {
    fn from(value: CsrAddr) -> Self {
        const SHIFT: u16 = 10;
        const MASK: u16 = 0b11;

        let value = (value >> SHIFT) & MASK;
        #[allow(clippy::manual_range_patterns)] 
        match value {
            0b00 | 0b01 | 0b10 => Self::ReadWrite,
            0b11 => Self::ReadOnly,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CsrAccessCtrl {
    pub privilege_level: PrivilegeLevel,
    pub write_access: WriteAccess,
}

impl From<CsrAddr> for CsrAccessCtrl {
    fn from(value: CsrAddr) -> Self {
        let privilege_level = value.privilege_level();
        let write_access = value.write_access();
        Self { privilege_level, write_access }
    }
}

impl fmt::Display for CsrAccessCtrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let pl = match self.privilege_level {
            PrivilegeLevel::Unprivileged => "U",
            PrivilegeLevel::Supervisor => "S",
            PrivilegeLevel::Hypervisor => "H",
            PrivilegeLevel::Machine => "M",
        };
        let wa = match self.write_access {
            WriteAccess::ReadOnly => "RO",
            WriteAccess::ReadWrite => "RW",
        };
        f.write_str(pl)?;
        f.write_str(wa)
    }
}
