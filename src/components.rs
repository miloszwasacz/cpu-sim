use std::cell::RefCell;

pub mod cpu;
pub mod diagnostics;
pub mod memory;

type Bus<'a, T> = &'a RefCell<T>;
