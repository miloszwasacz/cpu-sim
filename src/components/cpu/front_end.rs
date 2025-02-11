use self::decoder::Decoder;
use super::circuit::{Circuit, ClockCycle};
use super::error::{DecodeError, FetchError};
use super::reg::arf::ArchRegName;
use super::{flush_pipeline_regs, make_pipeline_regs, PipelineRegs, ProgramCounter, Stall};
use crate::components::memory::{Address, Memory, MemoryAccess};
use crate::components::Bus;
use crate::instr::raw::RawInstr;
use crate::instr::stall::nop;
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

    pub fn fetch(&mut self, pc: &mut ProgramCounter, stall: Stall) -> Result<(), FetchError> {
        const ALIGN: usize = IALIGN / BITS_IN_BYTE;

        let addr = pc.read();
        if addr % ALIGN as Address != 0 {
            return Err(FetchError::InstructionAddressMisaligned(addr));
        }
        let bits = self.mem_bus.read(ClockCycle::Full).borrow().get(addr);

        if stall {
            return Ok(());
        }

        pc.advance();
        *self.fetch_regs.borrow_mut().write(ClockCycle::SecondHalf) = FetchRegs {
            instr: Some(RawInstr::new(bits)),
            pc: *pc,
        };

        Ok(())
    }

    pub fn decode(
        &mut self,
        id_ex_write_reg: Option<ArchRegName>,
        ex_mem_write_reg: Option<ArchRegName>,
        mem_wb_write_reg: Option<ArchRegName>,
    ) -> Result<Stall, DecodeError> {
        let decoder = self.decoder.read(ClockCycle::FirstHalf);
        let fetch_regs_circ = self.fetch_regs.borrow();
        let fetch_regs = fetch_regs_circ.read(ClockCycle::FirstHalf);

        let instr = fetch_regs
            .instr
            .as_ref()
            .map(|instr| decoder.decode(*instr))
            .transpose()?;

        let stall = instr
            .as_ref()
            .map(|instr| {
                let read_regs = instr.read_regs();
                [id_ex_write_reg, ex_mem_write_reg, mem_wb_write_reg]
                    .iter()
                    .flatten()
                    .any(|write_reg| read_regs.contains(write_reg))
            })
            .unwrap_or_default();

        *self.decode_regs.borrow_mut().write(ClockCycle::SecondHalf) = DecodeRegs {
            instr: if stall { Some(nop()) } else { instr },
            pc: fetch_regs.pc,
        };

        Ok(stall)
    }

    pub fn finish_fetch_cycle(&mut self) {
        self.mem_bus.reset();
        self.fetch_regs.borrow_mut().reset();
    }

    pub fn finish_decode_cycle(&mut self) {
        self.decoder.reset();
        self.decode_regs.borrow_mut().reset();
    }

    pub fn flush(&mut self) {
        flush_pipeline_regs(&mut self.fetch_regs);
        flush_pipeline_regs(&mut self.decode_regs);
    }
}

#[derive(Debug, Default)]
struct FetchRegs {
    pub instr: Option<RawInstr>,
    pub pc: ProgramCounter,
}

#[derive(Debug, Default)]
pub(super) struct DecodeRegs {
    pub instr: Option<Rc<dyn Instr>>,
    pub pc: ProgramCounter,
}
