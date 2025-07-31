pub use self::addr::CsrAddr;
pub use self::file::{CsrAccessResult, CsrFile};
pub use self::access::{PrivilegeLevel, WriteAccess, CsrAccessCtrl};

mod access;
mod addr;
mod defs;
pub mod diagnostics;
mod file;

type CsrData = u64;
