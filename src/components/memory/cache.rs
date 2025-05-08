use self::indices::*;
use self::policies::WritePolicy;
pub use self::policies::{WriteBack, WriteThrough};
use self::traits::{MemoryRead, MemoryWrite};
pub use self::traits::{MemoryReadAccess, MemoryWriteAccess};
use super::{Address, ByteConvertible};

use std::marker::PhantomData;
use std::mem;
use std::num::NonZeroUsize;

mod indices;
mod policies;
mod traits;

//TODO Add support for Inclusive & Exclusive caches

//#region Cache

//TODO Latencies
pub struct Cache<L, P> {
    sets: Box<[CacheSet]>,
    line_len: NonZeroUsize,
    lower_level: L,
    write_policy: PhantomData<P>,
}

impl<L, P> Cache<L, P> {
    /// Makes a new `n`-way associative cache with size of `capacity` bytes.
    #[allow(private_bounds)]
    pub fn new(
        capacity: NonZeroUsize,
        n: NonZeroUsize,
        line_len: NonZeroUsize,
        lower_level: L,
    ) -> Self
    where
        P: WritePolicy,
    {
        let capacity = capacity.get();

        let set_cap = n.get() * line_len.get();
        let set_count = capacity / set_cap;
        assert_eq!(
            capacity % set_cap,
            0,
            "number of sets in the cache has to be an integer"
        );
        assert_eq!(
            1 << line_len.get().ilog2(),
            line_len.get(),
            "cache line length has to be a power of two"
        );
        assert_eq!(
            1 << set_count.ilog2(),
            set_count,
            "number of sets in the cache has to be a power of two"
        );

        Self {
            sets: vec![CacheSet::with_lines(n, line_len); set_count].into_boxed_slice(),
            line_len,
            lower_level,
            write_policy: PhantomData,
        }
    }
}

impl<L: MemoryRead> MemoryRead for Cache<L, WriteThrough> {
    fn read_line(&mut self, address: Address, line_len: usize) -> Box<[u8]> {
        debug_assert_eq!(self.line_len.get(), line_len);
        let (tag, index) = self.decompose_addr(address);
        if let Some(line) = self.set_at_mut(index).find(tag) {
            return line.data.clone();
        }

        let data = self.lower_level.read_line(address, line_len);
        let evicted = self.set_at_mut(index).write_line(tag, data.clone());
        debug_assert!(evicted.is_none());
        data
    }
}

impl<L: MemoryRead + MemoryWriteAccess> MemoryWriteAccess for Cache<L, WriteThrough> {
    //noinspection DuplicatedCode
    fn write<T: ByteConvertible + Clone>(&mut self, address: Address, data: T) {
        let line_len = self.line_len.get();
        let (tag, index) = self.decompose_addr(address);
        let line = match self.set_at_mut(index).find(tag) {
            Some(line) => line,
            None => {
                self.read_line(address, line_len);
                // Line brought in during read
                self.set_at_mut(index).find(tag).unwrap()
            }
        };

        let block_offset = BlockOffset::from(address, line_len);
        line.set(block_offset, data.clone());
        self.lower_level.write(address, data);
    }
}

impl<L: MemoryRead + MemoryWrite> MemoryRead for Cache<L, WriteBack> {
    fn read_line(&mut self, address: Address, line_len: usize) -> Box<[u8]> {
        debug_assert_eq!(self.line_len.get(), line_len);
        let (tag, index) = self.decompose_addr(address);
        if let Some(line) = self.set_at_mut(index).find(tag) {
            return line.data.clone();
        }

        let data = self.lower_level.read_line(address, line_len);
        self.write_line(address, data.clone());
        data
    }
}

impl<L: MemoryWrite> MemoryWrite for Cache<L, WriteBack> {
    fn write_line(&mut self, address: Address, data: Box<[u8]>) {
        debug_assert_eq!(self.line_len.get(), data.len());
        let (tag, index) = self.decompose_addr(address);
        if let Some(evicted) = self.set_at_mut(index).write_line(tag, data) {
            self.lower_level.write_line(address, evicted);
        }
    }
}

impl<L: MemoryRead + MemoryWrite> MemoryWriteAccess for Cache<L, WriteBack> {
    //noinspection DuplicatedCode
    fn write<T: ByteConvertible + Clone>(&mut self, address: Address, data: T) {
        let line_len = self.line_len.get();
        let (tag, index) = self.decompose_addr(address);
        let line = match self.set_at_mut(index).find(tag) {
            Some(line) => line,
            None => {
                self.read_line(address, line_len);
                // Line brought in during read
                self.set_at_mut(index).find(tag).unwrap()
            }
        };

        let block_offset = BlockOffset::from(address, line_len);
        line.set(block_offset, data);
    }
}

//#endregion

//#region CacheSet

type Used = bool;

/// A fully associative set that uses pseudo-LRU as replacement policy
#[derive(Debug, Clone)]
struct CacheSet(Box<[CacheLine]>, Box<[Used]>);

impl CacheSet {
    fn with_lines(line_count: NonZeroUsize, line_len: NonZeroUsize) -> Self {
        let line_count = line_count.get();
        let lines = vec![CacheLine::new(line_len); line_count].into_boxed_slice();
        let used = vec![false; line_count].into_boxed_slice();
        Self(lines, used)
    }

    fn find(&mut self, tag: Tag) -> Option<&mut CacheLine> {
        let (i, line) = self
            .0
            .iter_mut()
            .enumerate()
            .find(|(_, line)| matches!(line.tag, Some(line_tag) if line_tag == tag))?;

        Self::update_used(&mut self.1, i);
        Some(line)
    }

    fn write_line(&mut self, tag: Tag, data: Box<[u8]>) -> Option<Box<[u8]>> {
        let mut line = self
            .0
            .iter_mut()
            .enumerate()
            .find(|(_, line)| matches!(line.tag, Some(line_tag) if line_tag == tag));
        if line.is_none() {
            line = self
                .0
                .iter_mut()
                .enumerate()
                .find(|(_, line)| line.tag.is_none());
        }
        let (i, line) = match line {
            Some(line) => line,
            None => loop {
                let i = rand::random_range(0..self.1.len());
                if !self.1[i] {
                    break (i, &mut self.0[i]);
                }
            },
        };
        Self::update_used(&mut self.1, i);

        line.tag = Some(tag);
        let data = mem::replace(&mut line.data, data);
        let dirty = mem::take(&mut line.dirty);

        if dirty { Some(data) } else { None }
    }

    fn update_used(used: &mut Box<[Used]>, used_bank: usize) {
        used[used_bank] = true;
        if used.iter().all(|used| *used) {
            *used = vec![false; used.len()].into_boxed_slice();
            used[used_bank] = true;
        }
    }
}

//#endregion

//#region CacheLine

#[derive(Debug, Clone)]
struct CacheLine {
    data: Box<[u8]>,
    tag: Option<Tag>,
    dirty: bool,
}

impl CacheLine {
    fn new(len: NonZeroUsize) -> Self {
        Self {
            data: vec![0; len.get()].into_boxed_slice(),
            tag: None,
            dirty: false,
        }
    }

    fn set<T: ByteConvertible>(&mut self, index: BlockOffset, data: T) {
        let start = index.into();
        let len = size_of::<T>();
        let bytes = data.to_le_bytes();
        self.data[start..start + len].copy_from_slice(bytes.as_ref());
        self.dirty = true;
    }
}

//#endregion
