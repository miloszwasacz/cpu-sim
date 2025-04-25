use super::exec_unit::{ExecResult, ExecResultData};
use crate::components::cpu::exec_engine::RobIndex;
use crate::components::cpu::flip_flop::{Clearable, Sequential};
use crate::components::cpu::reg::RegData;

use std::mem;

#[derive(Debug, Default)]
pub struct CommonDataBus {
    results: Vec<ExecResult>,
    exec_results: Vec<ExecResult>,
    jump_links: Vec<ExecResult>,
    cleared: bool,
}

impl CommonDataBus {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn read(&self) -> &[ExecResult] {
        &self.results
    }

    pub fn write(&mut self, results: Vec<ExecResult>) {
        debug_assert!(self.exec_results.is_empty() && !self.cleared);
        self.exec_results = results;
    }

    pub fn write_jump_link(&mut self, tag: RobIndex, data: RegData) {
        let link = ExecResult {
            tag,
            result: Ok(ExecResultData::JumpLink(data)),
        };
        self.jump_links.push(link);
    }
}

impl Sequential for CommonDataBus {
    fn finish_cycle(&mut self) {
        if self.cleared {
            self.results.clear();
            self.exec_results.clear();
            self.jump_links.clear();
            self.cleared = false;
            return;
        }

        let mut results = mem::take(&mut self.exec_results);
        results.append(&mut self.jump_links);
        #[cfg(debug_assertions)]
        {
            use rand::seq::SliceRandom;
            let mut rng = rand::rng();
            results.shuffle(&mut rng)
        }
        self.results = results;
    }
}

impl Clearable for CommonDataBus {
    fn clear(&mut self) {
        self.cleared = true;
    }
}
