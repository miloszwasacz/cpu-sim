use super::indices::BlockOffset;
use crate::components::memory::{Address, ByteConvertible, Memory, L1D, L1I};

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

//#region MemoryRead

pub(super) trait MemoryRead {
    fn read_line(&mut self, address: Address, line_len: usize) -> Box<[u8]>;
}

impl MemoryRead for Memory {
    fn read_line(&mut self, address: Address, line_len: usize) -> Box<[u8]> {
        let start = (address & !BlockOffset::mask(line_len)) as usize;
        self.0[start..start + line_len].to_vec().into_boxed_slice()
    }
}

impl<T: MemoryRead> MemoryRead for Rc<RefCell<T>> {
    fn read_line(&mut self, address: Address, line_len: usize) -> Box<[u8]> {
        self.borrow_mut().read_line(address, line_len)
    }
}

impl<T: MemoryRead> MemoryRead for Arc<Mutex<T>> {
    fn read_line(&mut self, address: Address, line_len: usize) -> Box<[u8]> {
        self.lock().unwrap().read_line(address, line_len)
    }
}

//#endregion

//#region MemoryWrite

pub(super) trait MemoryWrite {
    fn write_line(&mut self, address: Address, line: Box<[u8]>);
}

impl MemoryWrite for Memory {
    fn write_line(&mut self, address: Address, line: Box<[u8]>) {
        let line_len = line.len();
        let start = (address & !BlockOffset::mask(line_len)) as usize;
        self.0[start..start + line_len].copy_from_slice(&line);
    }
}

impl<T: MemoryWrite> MemoryWrite for Rc<RefCell<T>> {
    fn write_line(&mut self, address: Address, line: Box<[u8]>) {
        self.borrow_mut().write_line(address, line)
    }
}

impl<T: MemoryWrite> MemoryWrite for Arc<Mutex<T>> {
    fn write_line(&mut self, address: Address, line: Box<[u8]>) {
        self.lock().unwrap().write_line(address, line)
    }
}

//#endregion

//#region MemoryReadAccess

pub trait MemoryReadAccess {
    fn read<T: ByteConvertible>(&mut self, address: Address) -> T;
}

impl<T: MemoryReadAccess> MemoryReadAccess for &mut T {
    fn read<D: ByteConvertible>(&mut self, address: Address) -> D {
        (*self).read(address)
    }
}

impl MemoryReadAccess for L1I {
    //noinspection DuplicatedCode
    fn read<T: ByteConvertible>(&mut self, address: Address) -> T {
        let line_len = self.0.line_len.get();
        read(&mut self.0, line_len, address)
    }
}

impl MemoryReadAccess for L1D {
    //noinspection DuplicatedCode
    fn read<T: ByteConvertible>(&mut self, address: Address) -> T {
        let line_len = self.0.line_len.get();
        read(&mut self.0, line_len, address)
    }
}

fn read<T: MemoryRead, D: ByteConvertible>(cache: &mut T, line_len: usize, address: Address) -> D {
    let line = cache.read_line(address, line_len);

    let block_offset = BlockOffset::from(address, line_len);
    let start = block_offset.into();
    let len = size_of::<D>();

    let bytes = &line[start..start + len];
    D::from_le_bytes(bytes.try_into().unwrap())
}

//#endregion

//#region MemoryWriteAccess

pub trait MemoryWriteAccess {
    fn write<T: ByteConvertible + Clone>(&mut self, address: Address, data: T);
}

impl<T: MemoryWriteAccess> MemoryWriteAccess for &mut T {
    fn write<D: ByteConvertible + Clone>(&mut self, address: Address, data: D) {
        (*self).write(address, data)
    }
}

impl MemoryWriteAccess for L1I {
    fn write<T: ByteConvertible + Clone>(&mut self, address: Address, data: T) {
        self.0.write(address, data);
    }
}

impl MemoryWriteAccess for L1D {
    fn write<T: ByteConvertible + Clone>(&mut self, address: Address, data: T) {
        self.0.write(address, data);
    }
}

impl MemoryWriteAccess for Memory {
    fn write<T: ByteConvertible + Clone>(&mut self, address: Address, data: T) {
        let start = address as usize;
        let len = size_of::<T>();
        let bytes = data.to_le_bytes();
        self.0[start..start + len].copy_from_slice(bytes.as_ref());
    }
}

impl<T: MemoryWriteAccess> MemoryWriteAccess for Rc<RefCell<T>> {
    fn write<D: ByteConvertible + Clone>(&mut self, address: Address, data: D) {
        self.borrow_mut().write(address, data)
    }
}

impl<T: MemoryWriteAccess> MemoryWriteAccess for Arc<Mutex<T>> {
    fn write<D: ByteConvertible + Clone>(&mut self, address: Address, data: D) {
        self.lock().unwrap().write(address, data)
    }
}

//#endregion
