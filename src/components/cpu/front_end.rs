use self::decoder::{DecodeResult, Decoder};
use super::circuit::{Circuit, ClockCycle};
use super::error::{DecodeError, FetchError};
use super::reg::pipeline::{
    DecodeRegs, ErrorControl, FetchRegs, IssueRegs, Jump, PcControl, PipelineRegs,
};
use super::Pc;
use crate::components::memory::{Memory, MemoryAccess};
use crate::components::Bus;
use crate::instr::raw::RawInstrBits;
use crate::{BITS_IN_BYTE, IALIGN};

mod decoder;

pub(super) struct FrontEnd<'m> {
    // Fetch
    fetch_regs: PipelineRegs<FetchRegs>,
    mem_bus: Circuit<Bus<'m, Memory>>,
    decode_regs: PipelineRegs<DecodeRegs>,

    // Decode
    decoder: Circuit<Decoder>,
    issue_regs: PipelineRegs<IssueRegs>,
}

impl<'m> FrontEnd<'m> {
    pub(super) fn new(mem_bus: Bus<'m, Memory>) -> Self {
        // Fetch
        let fetch_regs = Default::default();
        let mem_bus = mem_bus.into();
        let decode_regs = Default::default();

        // Decode
        let decoder = Decoder::new().into();
        let issue_regs = Default::default();

        Self {
            // Fetch
            fetch_regs,
            mem_bus,
            decode_regs,

            // Decode
            decoder,
            issue_regs,
        }
    }

    pub(super) unsafe fn set_pc(&mut self, pc: Pc) {
        self.fetch_regs.reset();
        self.fetch_regs
            .write(ClockCycle::SecondHalf, FetchRegs { pc });
        self.fetch_regs.reset();
    }

    pub(super) fn fetch_regs(&self) -> &PipelineRegs<FetchRegs> {
        &self.fetch_regs
    }

    pub(super) fn decode_regs(&self) -> &PipelineRegs<DecodeRegs> {
        &self.decode_regs
    }

    pub(super) fn issue_regs(&self) -> &PipelineRegs<IssueRegs> {
        &self.issue_regs
    }

    pub fn start_cycle(&mut self) {
        self.fetch_regs.reset();
        self.mem_bus.reset();
        self.decode_regs.reset();
        self.decoder.reset();
        self.issue_regs.reset();
    }

    pub fn fetch(&mut self, jump: Jump) {
        const ALIGN: usize = IALIGN / BITS_IN_BYTE;
        let FetchRegs { pc } = self.fetch_regs.read(ClockCycle::FirstHalf);

        let mut err_ctrl = ErrorControl {
            pc,
            ..Default::default()
        };
        if pc % ALIGN as Pc != 0 {
            err_ctrl.fetch_error = Some(FetchError::MisalignedInstr(pc));
        }

        let pc_ctrl = PcControl {
            pc,
            pc_plus4: pc + size_of::<RawInstrBits>() as Pc,
        };
        let instr: RawInstrBits = self.mem_bus.read(ClockCycle::FirstHalf).borrow().get(pc);
        let decode_regs = DecodeRegs {
            pc_ctrl,
            err_ctrl,
            instr,
        };

        self.decode_regs.write(ClockCycle::SecondHalf, decode_regs);

        // PC mutex
        let pc = jump.unwrap_or(pc_ctrl.pc_plus4);
        let fetch_regs = FetchRegs { pc };
        self.fetch_regs.write(ClockCycle::SecondHalf, fetch_regs);
    }

    pub fn decode(&mut self) {
        let DecodeRegs {
            pc_ctrl,
            mut err_ctrl,
            instr,
        } = self.decode_regs.read(ClockCycle::FirstHalf);
        let decoder = self.decoder.write(ClockCycle::FirstHalf);

        let mut issue_regs = IssueRegs {
            pc_ctrl,
            err_ctrl,
            ..Default::default()
        };
        match decoder.decode(instr) {
            DecodeResult::Ok(instr) => {
                issue_regs = IssueRegs::from_instr(instr.as_ref(), pc_ctrl, err_ctrl);
            }
            DecodeResult::NoInstr => {}
            DecodeResult::Err(instr) => {
                err_ctrl.decode_error = Some(DecodeError::InvalidInstruction(instr));
            }
        };

        self.issue_regs.write(ClockCycle::SecondHalf, issue_regs);
    }
}
