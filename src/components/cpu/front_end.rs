use self::decoder::Decoder;
use super::circuit::{Circuit, ClockCycle};
use super::error::{DecodeError, FetchError};
use super::{make_pipeline_regs, PipelineRegs, ProgramCounter, Result};
use crate::components::memory::{Address, Memory, MemoryAccess};
use crate::components::Bus;
use crate::instr::raw::RawInstr;
use crate::instr::Instr;
use crate::{BITS_IN_BYTE, IALIGN};

use std::rc::Rc;

mod decoder;

pub(super) struct FrontEnd<'m> {
    mem_bus: Circuit<Bus<'m, Memory>>,
    fetch_regs: PipelineRegs<FetchRegs>,
    decoder: Circuit<Decoder>,
    decode_regs: PipelineRegs<DecodeRegs>,
}

impl<'m> FrontEnd<'m> {
    pub(super) fn new(mem_bus: Bus<'m, Memory>) -> Self {
        let mem_bus = mem_bus.into();
        let fetch_regs = make_pipeline_regs();
        let decoder = Decoder::new().into();
        let decode_regs = make_pipeline_regs();
        Self {
            mem_bus,
            fetch_regs,
            decoder,
            decode_regs,
        }
    }

    pub(super) fn decode_regs(&self) -> &PipelineRegs<DecodeRegs> {
        &self.decode_regs
    }

    pub fn fetch(&mut self, pc: &mut ProgramCounter) -> Result<FetchError> {
        const ALIGN: usize = IALIGN / BITS_IN_BYTE;

        let addr = pc.read();
        if addr % ALIGN as Address != 0 {
            return Err(FetchError::InstructionAddressMisaligned(addr));
        }
        let bits = self.mem_bus.read(ClockCycle::Full).borrow().get(addr);

        self.fetch_regs
            .borrow_mut()
            .write(ClockCycle::SecondHalf)
            .instr = Some(RawInstr::new(bits));

        pc.advance();
        Ok(())
    }

    pub fn decode(&mut self) -> Result<DecodeError> {
        let decoder = self.decoder.read(ClockCycle::FirstHalf);
        let instr = self
            .fetch_regs
            .borrow()
            .read(ClockCycle::FirstHalf)
            .instr
            .as_ref()
            .map(|instr| decoder.decode(*instr))
            .transpose()?;

        self.decode_regs
            .borrow_mut()
            .write(ClockCycle::SecondHalf)
            .instr = instr;

        Ok(())
    }

    pub fn finish_fetch_cycle(&mut self) {
        self.mem_bus.reset();
        self.fetch_regs.borrow_mut().reset();
    }

    pub fn finish_decode_cycle(&mut self) {
        self.fetch_regs.borrow_mut().reset();
        self.decoder.reset();
        self.decode_regs.borrow_mut().reset();
    }
}

#[derive(Debug, Default)]
struct FetchRegs {
    pub instr: Option<RawInstr>,
}

#[derive(Debug, Default)]
pub(super) struct DecodeRegs {
    pub instr: Option<Rc<dyn Instr>>,
}
