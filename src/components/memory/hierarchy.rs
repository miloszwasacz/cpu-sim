use super::cache::{Cache, WriteBack, WriteThrough};
use super::size::MemSize;
use super::Memory;

use std::cell::RefCell;
use std::num::NonZeroUsize;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

//TODO Make configurable
const LINE_LENGTH: NonZeroUsize = NonZeroUsize::new(64).unwrap();

const L1I_CAPACITY: NonZeroUsize = NonZeroUsize::new(MemSize(32).KiB()).unwrap();
const L1I_ASSOC: NonZeroUsize = NonZeroUsize::new(4).unwrap();
pub struct L1I(pub(super) Cache<Rc<RefCell<L2>>, WriteThrough>);

const L1D_CAPACITY: NonZeroUsize = NonZeroUsize::new(MemSize(32).KiB()).unwrap();
const L1D_ASSOC: NonZeroUsize = NonZeroUsize::new(4).unwrap();
pub struct L1D(pub(super) Cache<Rc<RefCell<L2>>, WriteThrough>);

const L2_CAPACITY: NonZeroUsize = NonZeroUsize::new(MemSize(256).KiB()).unwrap();
const L2_ASSOC: NonZeroUsize = NonZeroUsize::new(8).unwrap();
type L2 = Cache<L3, WriteBack>;

const L3_CAPACITY: NonZeroUsize = NonZeroUsize::new(MemSize(2).MiB()).unwrap();
const L3_ASSOC: NonZeroUsize = NonZeroUsize::new(16).unwrap();
type L3 = Cache<Arc<Mutex<Memory>>, WriteBack>;

pub struct MemHierarchy {
    l1i: L1I,
    l1d: L1D,
}

impl MemHierarchy {
    pub fn new(mem: Arc<Mutex<Memory>>) -> Self {
        let l3 = Cache::<_, WriteBack>::new(L3_CAPACITY, L3_ASSOC, LINE_LENGTH, mem);
        let l2 = Rc::new(RefCell::new(Cache::<_, WriteBack>::new(
            L2_CAPACITY,
            L2_ASSOC,
            LINE_LENGTH,
            l3,
        )));
        let l1i = L1I(Cache::<_, WriteThrough>::new(
            L1I_CAPACITY,
            L1I_ASSOC,
            LINE_LENGTH,
            l2.clone(),
        ));
        let l1d = L1D(Cache::<_, WriteThrough>::new(
            L1D_CAPACITY,
            L1D_ASSOC,
            LINE_LENGTH,
            l2,
        ));

        Self { l1i, l1d }
    }

    pub fn l1i(&mut self) -> &mut L1I {
        &mut self.l1i
    }

    pub fn l1d(&mut self) -> &mut L1D {
        &mut self.l1d
    }
}
