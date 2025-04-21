use cpu_sim::components::diagnostics::cpu::{Exception, RawInstr};
use cpu_sim::components::memory::Address;

pub type Fetched = Result<RawInstr, Exception>;

pub struct FrontEndModel {
    pc: Address,
    fetched: Box<[Fetched]>,
    decode_width: usize,
}

impl FrontEndModel {
    pub(super) fn new(pc: Address, fetched: Box<[Fetched]>, decode_width: usize) -> Self {
        Self {
            pc,
            fetched,
            decode_width,
        }
    }

    pub fn pc(&self) -> Address {
        self.pc
    }

    pub fn fetched(&self) -> &[Fetched] {
        &self.fetched
    }

    pub fn decode_width(&self) -> usize {
        self.decode_width
    }
}
