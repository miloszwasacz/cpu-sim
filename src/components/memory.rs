pub use self::size::MemSize;
pub(super) use self::convert::ByteConvertible;

use std::ops::{Index, IndexMut, Range};

mod convert;
mod size;

pub type Address = u32;

#[derive(Debug, Clone)]
pub struct Memory(Box<[u8]>);

impl Memory {
    const SIZE: usize = MemSize(8).GiB();

    pub fn new() -> Self {
        Memory(vec![0; Self::SIZE].into_boxed_slice())
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}

impl Index<Address> for Memory {
    type Output = u8;

    fn index(&self, index: Address) -> &Self::Output {
        self.0.index(index as usize)
    }
}

impl IndexMut<Address> for Memory {
    fn index_mut(&mut self, index: Address) -> &mut Self::Output {
        self.0.index_mut(index as usize)
    }
}

impl Index<Range<Address>> for Memory {
    type Output = [u8];

    fn index(&self, index: Range<Address>) -> &Self::Output {
        let index = index.start as usize..index.end as usize;
        self.0.index(index)
    }
}

impl IndexMut<Range<Address>> for Memory {
    fn index_mut(&mut self, index: Range<Address>) -> &mut Self::Output {
        let index = index.start as usize..index.end as usize;
        self.0.index_mut(index)
    }
}

pub trait MemoryAccess<T: ByteConvertible> {
    fn get(&self, address: Address) -> T;
    fn set(&mut self, address: Address, data: T);
}

impl<T: ByteConvertible> MemoryAccess<T> for Memory {
    fn get(&self, address: Address) -> T {
        let bytes = &self.0[addr_range::<T>(address)];
        T::from_le_bytes(bytes.try_into().unwrap())
    }

    fn set(&mut self, address: Address, data: T) {
        let bytes = data.to_le_bytes();
        self.0[addr_range::<T>(address)].copy_from_slice(bytes.as_ref());
    }
}

/// Returns a range that starts at `addr` and has length [`size_of::<T>()`](size_of)
fn addr_range<T>(addr: Address) -> Range<usize> {
    let addr = addr as usize;
    addr..(addr + size_of::<T>())
}
