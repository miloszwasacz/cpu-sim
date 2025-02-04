pub use self::size::MemSize;
use crate::int_impl;

use std::ops::{Index, IndexMut, Range};

mod size;

pub type Address = u32;

#[derive(Debug, Clone)]
pub struct Memory(Box<[u8]>);

impl Memory {
    const SIZE: usize = MemSize(8).MiB();
    
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

pub trait MemoryAccess<T> {
    fn get(&self, address: Address) -> T;
    fn set(&mut self, address: Address, data: T);
}

macro_rules! impl_mem_access {
    ($ty:ident) => {
        impl MemoryAccess<$ty> for Memory {
            fn get(&self, address: Address) -> $ty {
                let bytes = &self.0[addr_range::<$ty>(address)];
                $ty::from_le_bytes(bytes.try_into().unwrap())
            }

            fn set(&mut self, address: Address, data: $ty) {
                let bytes = data.to_le_bytes();
                self.0[addr_range::<$ty>(address)].copy_from_slice(&bytes);
            }
        }
    };
}
int_impl!(impl_mem_access);

/// Returns a range that starts at `addr` and has length [`size_of::<T>()`](size_of)
fn addr_range<T>(addr: Address) -> Range<usize> {
    let addr = addr as usize;
    addr..(addr + size_of::<T>())
}

