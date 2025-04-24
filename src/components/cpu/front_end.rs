pub use self::adder::{JumpAgu, PcAdder};
pub use self::decode_queue::DecodeQueue;
use self::decode_queue::{DecodeQueueEntry, Decoded};
pub use self::decoder::Decoder;
use super::error::FetchError;
use super::reg::pipeline::IfIdRegs;
use super::{Cpu, Pc, Stall};
use crate::components::memory::{Address, MemoryReadAccess};
use crate::instr::full::FullInstruction;
use crate::instr::raw::RawInstr;
use crate::instr::{AluSrcA, Instruction};
use crate::{BITS_IN_BYTE, IALIGN};

mod adder;
pub(super) mod decode_queue;
mod decoder;
pub mod diagnostics;

pub(super) type PcPlus4 = Pc;

impl<I, O, E> Cpu<I, O, E> {
    #[must_use]
    pub(super) fn fetch(&mut self) -> Pc {
        const ALIGN: Address = (IALIGN / BITS_IN_BYTE) as Address;

        let mut pc = *self.pc.read();
        let mut regs = Vec::with_capacity(self.decoders.len());
        for _ in 0..self.decoders.len() {
            let pc_plus_4 = self.pc_adder.add(pc);

            let instr = if pc % ALIGN == 0 {
                let bits = self.mem_hierarchy.l1i().read(pc);
                Ok(RawInstr::new(bits))
            } else {
                Err(FetchError::MisalignedInstr(pc).into())
            };

            regs.push(IfIdRegs {
                instr,
                pc,
                pc_plus_4,
            });
            pc = pc_plus_4;
        }
        debug_assert_eq!(regs.len(), self.decoders.len());
        self.if_id_regs.write(regs.into_boxed_slice());
        pc
    }

    #[must_use]
    pub(super) fn decode(&mut self) -> Stall {
        let circuits = self.decoders.iter_mut().zip(self.jump_agus.iter_mut());
        let decoded = self.if_id_regs.read().iter().copied().zip(circuits).map(
            |(regs, (decoder, jump_agu))| {
                let IfIdRegs {
                    instr,
                    pc,
                    pc_plus_4,
                } = regs;
                let decoded = decoder.decode(instr);
                match decoded {
                    Ok(instr) => {
                        DecodeQueueEntry::Ok(Self::decode_single(instr, jump_agu, pc, pc_plus_4))
                    }
                    Err(ex) => DecodeQueueEntry::Exception(ex, pc),
                }
            },
        );

        !self.decode_queue.try_push(decoded)
    }

    fn decode_single(
        full: FullInstruction,
        jump_agu: &mut JumpAgu,
        pc: Pc,
        pc_plus_4: Pc,
    ) -> Decoded {
        let instr = full.into();
        let (predicted, target) = match instr {
            Instruction::Jump {
                base,
                offset,
                apply_mask,
                ..
            } => match base {
                AluSrcA::Reg(_) => {
                    //TODO Add address prediction for register-based jumps
                    (Some(pc_plus_4), None)
                }
                AluSrcA::Pc => {
                    let jump = jump_agu.jump_target(pc, offset, apply_mask);
                    (Some(jump), None)
                }
            },
            Instruction::Branch { offset, .. } => {
                //TODO Branch prediction
                let target = jump_agu.jump_target(pc, offset, false);
                (Some(pc_plus_4), Some(target))
            }
            Instruction::Alu { .. }
            | Instruction::Load { .. }
            | Instruction::Store { .. }
            | Instruction::EnvTrap(_) => (None, None),
        };

        Decoded {
            instr,
            full,
            pc,
            pc_plus_4,
            predicted,
            target,
        }
    }
}
