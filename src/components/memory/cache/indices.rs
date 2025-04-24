use super::{Cache, CacheSet};
use crate::components::memory::Address;

pub(super) type BlockOffsetMask = Address;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(super) struct BlockOffset(Address);

impl BlockOffset {
    pub fn mask(line_len: usize) -> BlockOffsetMask {
        let shift = line_len.ilog2();
        !((Address::MAX >> shift) << shift)
    }
    
    pub fn from(addr: Address, line_len: usize) -> Self {
        let mask = Self::mask(line_len);
        Self(addr & mask)
    }
}

impl From<BlockOffset> for usize {
    fn from(value: BlockOffset) -> Self {
        value.0 as usize
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(super) struct SetIndex(Address);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(super) struct Tag(Address);

impl<L, P> Cache<L, P> {
    pub(super) fn set_at_mut(&mut self, index: SetIndex) -> &mut CacheSet {
        &mut self.sets[index.0 as usize]
    }

    #[inline(always)]
    pub(super) fn decompose_addr(&self, mut addr: Address) -> (Tag, SetIndex) {
        let line_len = self.line_len.get().ilog2();
        let set_count = self.sets.len().ilog2();

        addr >>= line_len;
        let index_mask = !((Address::MAX >> set_count) << set_count);
        let index = SetIndex(addr & index_mask);

        addr >>= set_count;
        let tag_mask = Address::MAX >> (line_len + set_count);
        let tag = Tag(addr & tag_mask);

        (tag, index)
    }
}
