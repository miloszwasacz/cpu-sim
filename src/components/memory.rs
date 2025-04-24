pub(super) use self::convert::ByteConvertible;
pub use self::hierarchy::{MemHierarchy, L1D, L1I};
pub use self::size::MemSize;
pub use self::cache::{MemoryWriteAccess, MemoryReadAccess};

use std::ops::{Index, IndexMut, Range};

mod cache;
mod convert;
mod hierarchy;
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
