use crate::components::memory::Address;

use const_format::formatcp;
use std::fmt;
use std::ops::{BitAnd, BitOr, Not, Shl, Shr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RawInstr {
    bits: RawInstrBits,
    addr: Address,
}

pub type RawInstrBits = u32;

impl RawInstr {
    pub fn new(bits: RawInstrBits, addr: Address) -> RawInstr {
        Self { bits, addr }
    }

    pub fn from_bytes(bytes: [u8; size_of::<RawInstrBits>()], addr: Address) -> RawInstr {
        let bits = RawInstrBits::from_le_bytes(bytes);
        Self { bits, addr }
    }

    pub fn encoding(&self) -> RawInstrBits {
        self.bits
    }

    pub fn extract_bits<const N: u64, const MSB: u64>(&self) -> Bits<N> {
        debug_assert!(
            MSB < MAX_BITS,
            "{}",
            formatcp!("Most-Significant Bit cannot be larger than {}", MAX_BITS)
        );

        let shift = 1 + MSB - N;
        Bits::new(self.bits as u64 >> shift)
    }
}

impl fmt::Display for RawInstr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:032b}", self.bits)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Bits<const N: u64> {
    value: u64,
    mask: u64,
}

const MAX_BITS: u64 = u64::BITS as u64;

impl<const N: u64> Bits<N> {
    pub const fn new(value: u64) -> Self {
        let bits = Self::zeros();
        let value = value & bits.mask;
        Self { value, ..bits }
    }

    pub const fn ones() -> Self {
        let mut bits = Self::zeros();
        bits.value = bits.mask;
        bits
    }

    pub const fn zeros() -> Self {
        debug_assert!(
            N < MAX_BITS,
            "{}",
            formatcp!("N cannot be larger than {}", MAX_BITS)
        );
        let mask = u64::MAX >> (u64::BITS as u64 - N);
        Self { value: 0, mask }
    }

    pub const fn resize<const M: u64>(self) -> Bits<M> {
        debug_assert!(
            M < MAX_BITS,
            "{}",
            formatcp!("M cannot be larger than {}", MAX_BITS)
        );
        let mask = if M > N {
            let mut mask = !self.mask;
            mask <<= M - N;
            !mask
        } else {
            self.mask >> (N - M)
        };
        let value = self.value & mask;
        Bits { value, mask }
    }

    pub const fn at(&self, index: u64) -> u64 {
        (self.value >> index) & 0b1
    }

    pub fn sign_extend<const M: u64>(self) -> Bits<M> {
        debug_assert!(
            M < MAX_BITS,
            "{}",
            formatcp!("M cannot be larger than {}", MAX_BITS)
        );
        debug_assert!(M >= N, "M cannot be smaller than N");
        if M == N {
            return Bits {
                value: self.value,
                mask: self.mask,
            };
        }

        let extend = match self.at(N - 1) {
            0 => Bits::<M>::zeros(),
            1 => {
                let mut bits = Bits::<M>::ones();
                bits.value &= !self.mask;
                bits
            }
            _ => unreachable!(),
        };

        let value = extend.value | self.value;
        Bits { value, ..extend }
    }

    pub const fn ror(self, shift: u64) -> Self {
        if shift == 0 {
            return self;
        }

        let x = self.value;
        let m = shift % N;
        let value = (x >> m) | (x << (N - m)) & self.mask;

        Self { value, ..self }
    }
}

impl<const N: u64> fmt::Display for Bits<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let width = N as usize;
        write!(f, "{:0width$}", self.value)
    }
}

impl<const N: u64> PartialEq for Bits<N> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<const N: u64> Eq for Bits<N> {}

impl<const N: u64> PartialEq<u64> for Bits<N> {
    fn eq(&self, other: &u64) -> bool {
        self.value == *other
    }
}

impl<const N: u64> PartialEq<u32> for Bits<N> {
    fn eq(&self, other: &u32) -> bool {
        self.value == *other as u64
    }
}

impl<const N: u64> BitAnd for Bits<N> {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self::Output {
            value: self.value & rhs.value,
            mask: self.mask,
        }
    }
}

impl<const N: u64> BitAnd<u64> for Bits<N> {
    type Output = Self;

    fn bitand(self, rhs: u64) -> Self::Output {
        Self::Output {
            value: self.value & rhs,
            mask: self.mask,
        }
    }
}

impl<const N: u64> BitAnd<u32> for Bits<N> {
    type Output = Self;

    fn bitand(self, rhs: u32) -> Self::Output {
        Self::Output {
            value: self.value & rhs as u64,
            mask: self.mask,
        }
    }
}

impl<const N: u64> BitOr for Bits<N> {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self::Output {
            value: self.value | rhs.value,
            mask: self.mask | rhs.mask,
        }
    }
}

impl<const N: u64> BitOr<u64> for Bits<N> {
    type Output = Self;

    fn bitor(self, rhs: u64) -> Self::Output {
        let value = (self.value | rhs) & self.mask;
        Self::Output { value, ..self }
    }
}

impl<const N: u64> BitOr<u32> for Bits<N> {
    type Output = Self;

    fn bitor(self, rhs: u32) -> Self::Output {
        let value = (self.value | rhs as u64) & self.mask;
        Self::Output { value, ..self }
    }
}

impl<const N: u64> Not for Bits<N> {
    type Output = Self;

    fn not(self) -> Self::Output {
        let value = !self.value & self.mask;
        Self::Output { value, ..self }
    }
}

impl<const N: u64> Shl<u64> for Bits<N> {
    type Output = Self;

    fn shl(self, rhs: u64) -> Self::Output {
        let value = (self.value << rhs) & self.mask;
        Self::Output { value, ..self }
    }
}

impl<const N: u64> Shl<u32> for Bits<N> {
    type Output = Self;

    fn shl(self, rhs: u32) -> Self::Output {
        let value = (self.value << rhs) & self.mask;
        Self::Output { value, ..self }
    }
}

impl<const N: u64> Shr<u64> for Bits<N> {
    type Output = Self;

    fn shr(self, rhs: u64) -> Self::Output {
        let value = self.value >> rhs;
        Self::Output { value, ..self }
    }
}

impl<const N: u64> Shr<u32> for Bits<N> {
    type Output = Self;

    fn shr(self, rhs: u32) -> Self::Output {
        let value = self.value >> rhs;
        Self::Output { value, ..self }
    }
}

impl<const N: u64> From<Bits<N>> for u32 {
    fn from(bits: Bits<N>) -> Self {
        bits.value as u32
    }
}

impl<const N: u64> From<Bits<N>> for u64 {
    fn from(bits: Bits<N>) -> Self {
        bits.value
    }
}

impl<const N: u64> From<Bits<N>> for i32 {
    fn from(bits: Bits<N>) -> Self {
        let bits = bits.sign_extend::<32>();
        bits.value as u32 as i32
    }
}

impl<const N: u64> From<Bits<N>> for i64 {
    fn from(bits: Bits<N>) -> Self {
        let bits = bits.sign_extend::<64>();
        bits.value as i64
    }
}

impl<const N: u64> From<u32> for Bits<N> {
    fn from(value: u32) -> Self {
        Self::new(value as u64)
    }
}

impl<const N: u64> From<u64> for Bits<N> {
    fn from(value: u64) -> Self {
        Self::new(value)
    }
}

impl<const N: u64> From<i32> for Bits<N> {
    fn from(value: i32) -> Self {
        Self::new(value as u64)
    }
}

impl<const N: u64> From<i64> for Bits<N> {
    fn from(value: i64) -> Self {
        Self::new(value as u64)
    }
}
