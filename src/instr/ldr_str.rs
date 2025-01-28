use self::addr_mode::AddressingMode;
use self::data_size::DataSize;
use self::offset::{ImmOffset, Offset};
use crate::pipeline::decode::sign_extend;
use crate::reg::{RegisterId, RegisterSize};

use std::marker::PhantomData;

pub mod ldr_str_reg_imm;
pub mod ldr_str_reg_reg_off;

struct LdrStr<T> {
    op: LdrStrOp,
    t: RegisterId,
    n: RegisterId,
    datasize: PhantomData<T>,
    offset: Offset,
}

enum LdrStrOp {
    Ldr,
    Str,
}

impl<T: DataSize> LdrStr<T> {
    pub fn new_imm(
        op: LdrStrOp,
        t: (u32, RegisterSize),
        n: u32,
        imm: u32,
        addr_mode: AddressingMode,
    ) -> Self {
        let t = RegisterId::decode(t.0, t.1, false);
        let n = RegisterId::decode(n, RegisterSize::X, true);
        let datasize = PhantomData::<T>;

        let scale = T::SIZE_ENCODED;
        let wback = addr_mode.wback();
        let post_index = addr_mode.post_index();
        let offset = match addr_mode {
            AddressingMode::PostIndex | AddressingMode::PreIndex => {
                ImmOffset::Signed(sign_extend(imm, 9))
            }
            AddressingMode::UnsignedOffset => ImmOffset::Unsigned((imm as u64) << scale),
        };
        let offset = Offset::Imm {
            wback,
            post_index,
            offset,
        };

        Self {
            op,
            t,
            n,
            datasize,
            offset,
        }
    }

    pub fn new_reg(
        op: LdrStrOp,
        t: (u32, RegisterSize),
        n: u32,
        m: (u32, RegisterSize),
        option: u32,
        amount: u32,
    ) -> Self {
        let t = RegisterId::decode(t.0, t.1, false);
        let n = RegisterId::decode(n, RegisterSize::X, true);
        let datasize = PhantomData::<T>;

        let m = RegisterId::decode(m.0, m.1, false);
        let offset = Offset::Reg {
            m,
            extend_type: option.into(),
            shift: amount,
        };

        Self {
            op,
            t,
            n,
            datasize,
            offset,
        }
    }
}

mod data_size {
    pub trait DataSize {
        /// The size of the type, in bits.
        const BITS: u32;

        /// The size of the type, in bytes.
        const BYTES: u32;

        /// The `size` field encoding of the type.
        const SIZE_ENCODED: u64;
    }

    macro_rules! impl_data_size {
        ($( $ty:tt )+) => {
            $(
            impl DataSize for $ty {
                const BITS: u32 = $ty::BITS;
                const BYTES: u32 = Self::BITS / 8;
                const SIZE_ENCODED: u64 = Self::BYTES.ilog2() as u64;
            }
            )+
        };
    }

    impl_data_size!(u8 u16 u32 u64 i8 i16 i32 i64);
}

pub mod addr_mode {
    pub enum AddressingMode {
        PostIndex,
        PreIndex,
        UnsignedOffset,
    }

    impl AddressingMode {
        #[inline]
        pub fn wback(&self) -> bool {
            match self {
                AddressingMode::PostIndex => true,
                AddressingMode::PreIndex => true,
                AddressingMode::UnsignedOffset => false,
            }
        }

        #[inline]
        pub fn post_index(&self) -> bool {
            match self {
                AddressingMode::PostIndex => true,
                AddressingMode::PreIndex => false,
                AddressingMode::UnsignedOffset => false,
            }
        }
    }
}

mod offset {
    use super::extend::ExtendType;
    use crate::reg::RegisterId;

    pub(super) enum Offset {
        Imm {
            wback: bool,
            post_index: bool,
            offset: ImmOffset,
        },
        Reg {
            m: RegisterId,
            extend_type: ExtendType,
            shift: u32,
        },
    }

    pub(super) enum ImmOffset {
        Signed(i64),
        Unsigned(u64),
    }
}

mod extend {
    pub(super) enum ExtendType {
        Uxtw,
        Lsl,
        Sxtw,
        Sxtx,
    }

    impl From<u32> for ExtendType {
        fn from(value: u32) -> Self {
            match value {
                0b010 => ExtendType::Uxtw,
                0b011 => ExtendType::Lsl,
                0b110 => ExtendType::Sxtw,
                0b111 => ExtendType::Sxtx,
                e => panic!("{e} is not a valid extend/shift specifier"),
            }
        }
    }
}
