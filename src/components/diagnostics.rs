use crate::components::memory::Address;

pub mod cpu;

pub trait Diagnostics {
    type Output;

    fn diagnostics(&self) -> Self::Output;
}

pub fn fmt_addr(addr: Address) -> String {
    format!("0x{:05x}", addr)
}
