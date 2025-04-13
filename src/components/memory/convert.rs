use crate::apply_macro;

use std::fmt;

pub trait ByteConvertible: Sized {
    type Bytes: for<'a> TryFrom<&'a [u8], Error: fmt::Debug> + AsRef<[u8]>;
    fn from_le_bytes(bytes: Self::Bytes) -> Self;
    fn to_le_bytes(self) -> Self::Bytes;
}

macro_rules! impl_convert {
    ($ty:ty) => {
        impl ByteConvertible for $ty {
            type Bytes = [u8; size_of::<Self>()];

            fn from_le_bytes(bytes: Self::Bytes) -> Self {
                Self::from_le_bytes(bytes)
            }

            fn to_le_bytes(self) -> Self::Bytes {
                Self::to_le_bytes(self)
            }
        }
    };
}
apply_macro!(u8 u16 u32 u64 i8 i16 i32 i64 => impl_convert!());
