pub use self::data_type::DataType;
use crate::apply_macro;
use crate::components::memory::Address;

use std::fmt;

mod data_type;

macro_rules! regdata_from_impls {
    ($ty:ty, SignedRegData) => {
        impl From<RegData> for $ty {
            #[inline(always)]
            fn from(value: RegData) -> Self {
                value.i() as $ty
            }
        }

        impl From<$ty> for RegData {
            #[inline(always)]
            fn from(value: $ty) -> Self {
                RegData::signed(value as SignedRegData)
            }
        }
    };
    ($ty:ty, UnsignedRegData) => {
        impl From<RegData> for $ty {
            #[inline(always)]
            fn from(value: RegData) -> Self {
                value.u() as $ty
            }
        }

        impl From<$ty> for RegData {
            #[inline(always)]
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

impl PartialEq for RegData {
    fn eq(&self, other: &Self) -> bool {
        self.i().eq(&other.i())
    }
}

impl Default for RegData {
    fn default() -> Self {
        Self { i: 0 }
    }
}

apply_macro!(i8 i16 i32 i64 => regdata_from_impls!(SignedRegData));
apply_macro!(u8 u16 u32 u64 => regdata_from_impls!(UnsignedRegData));
