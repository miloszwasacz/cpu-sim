use crate::components::memory::Address;

use std::fmt;

macro_rules! regdata_from_int {
    ($ty:ident, SignedRegData) => {
        impl From<$ty> for RegData {
            fn from(value: $ty) -> Self {
                RegData::signed(value as SignedRegData)
            }
        }
    };
    ($ty:ident, UnsignedRegData) => {
        impl From<$ty> for RegData {
            fn from(value: $ty) -> Self {
                RegData::unsigned(value as UnsignedRegData)
            }
        }
    };
}

type SignedRegData = i32;
type UnsignedRegData = u32;

#[derive(Clone, Copy)]
pub union RegData {
    i: SignedRegData,
    u: UnsignedRegData,
    addr: Address,
}

impl RegData {
    pub fn signed(i: SignedRegData) -> Self {
        Self { i }
    }

    pub fn unsigned(u: UnsignedRegData) -> Self {
        Self { u }
    }

    pub fn address(addr: Address) -> Self {
        Self { addr }
    }

    pub fn i(&self) -> SignedRegData {
        unsafe { self.i }
    }

    pub fn u(&self) -> UnsignedRegData {
        unsafe { self.u }
    }

    pub fn addr(&self) -> Address {
        unsafe { self.addr }
    }
}

impl fmt::Debug for RegData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.i().fmt(f)
    }
}

impl Default for RegData {
    fn default() -> Self {
        Self { i: 0 }
    }
}

regdata_from_int!(u8, UnsignedRegData);
regdata_from_int!(u16, UnsignedRegData);
regdata_from_int!(u32, UnsignedRegData);
regdata_from_int!(u64, UnsignedRegData);
regdata_from_int!(i8, SignedRegData);
regdata_from_int!(i16, SignedRegData);
regdata_from_int!(i32, SignedRegData);
regdata_from_int!(i64, SignedRegData);
