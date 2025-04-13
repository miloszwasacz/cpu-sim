use super::exec_unit::ExecResult;
use crate::components::cpu::flip_flop::{Clearable, FlipFlop, Sequential};

#[derive(Debug)]
pub struct CommonDataBus(FlipFlop<Vec<ExecResult>>);

impl CommonDataBus {
    pub fn new() -> Self {
        Self(FlipFlop::new(Vec::new()))
    }
    
    pub fn read(&self) -> &[ExecResult] {
        self.0.read()
    }

    pub fn write(&mut self, results: Vec<ExecResult>) {
        self.0.write(results);
    }
}

impl Sequential for CommonDataBus {
    fn finish_cycle(&mut self) {
        self.0.finish_cycle();
    }
}

impl Clearable for CommonDataBus {
    fn clear(&mut self) {
        self.0.clear();
    }
}
