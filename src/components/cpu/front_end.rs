pub use self::adder::{JumpAgu, PcAdder};
pub use self::branch_predictor::{BranchPredictor, ZeroBubblePredictor};
pub use self::decode_queue::DecodeQueue;
use self::decode_queue::{DecodeQueueEntry, Decoded};
pub use self::decoder::Decoder;
use super::error::FetchError;
use super::reg::pipeline::{BpRegs, IfRegs, Prediction, ZbpRegs};
use super::{Cpu, Pc, Stall};
use crate::components::memory::{Address, MemoryReadAccess};
use crate::instr::full::FullInstruction;
use crate::instr::raw::RawInstr;
use crate::instr::{AluSrcA, Instruction};
use crate::{BITS_IN_BYTE, IALIGN};

use itertools::Itertools;

mod adder;
mod branch_predictor;
pub(super) mod decode_queue;
mod decoder;
pub mod diagnostics;

pub(super) type PcPlus4 = Pc;

impl<I, O, E> Cpu<I, O, E> {
    #[must_use]
    pub(super) fn zb_predict(&mut self) -> Pc {
        let mut pc = *self.pc.read();
        let mut regs = Vec::with_capacity(self.pc_adders.len());
        for pc_adder in &mut self.pc_adders {
            let pc_plus_4 = pc_adder.add(pc);

            let predicted = self.zb_predictor.predict(pc);
            regs.push(ZbpRegs {
                pc,
                pc_plus_4,
                predicted,
            });
            pc = pc_plus_4;

            if predicted.is_some() {
                break;
            }
        }
        debug_assert!(regs.len() <= self.pc_adders.len());
        let next_pc = regs
            .last()
            .expect("there should be at least one new PC")
            .predicted
            .unwrap_or(pc);

        self.zbp_regs.write(regs.into_boxed_slice());
        next_pc
    }

    pub(super) fn fetch(&mut self) -> Stall {
        const ALIGN: Address = (IALIGN / BITS_IN_BYTE) as Address;

        let regs = self
            .zbp_regs
            .read()
            .iter()
            .map(|regs| {
                let pc = regs.pc;
                let instr = if pc % ALIGN == 0 {
                    let bits = self.mem_hierarchy.l1i().read(pc);
                    Ok(RawInstr::new(bits))
                } else {
                    Err(FetchError::MisalignedInstr(pc).into())
                };

                IfRegs {
                    instr,
                    pc,
                    pc_plus_4: regs.pc_plus_4,
                    predicted: regs.predicted,
                }
            })
            .collect();

        self.if_regs.write(regs);

        //TODO Stalling because Memory is slow
        false
    }

    pub(super) fn branch_prediction(&mut self) -> Option<Pc> {
        let mut new_prediction = None;
        let regs = self
            .if_regs
            .read()
            .iter()
            .map(|regs| {
                let taken = self.branch_predictor.predict(regs.instr, regs.pc);
                let predicted = match (regs.predicted, taken) {
                    (predicted, false) => {
                        if predicted.is_some() {
                            debug_assert!(new_prediction.is_none());
                            new_prediction = Some(regs.pc_plus_4);
                        }
                        Prediction::NotTaken
                    }
                    (None, true) => Prediction::TakenUnknown,
                    (Some(predicted), true) => Prediction::Taken(predicted),
                };

                (
                    BpRegs {
                        instr: regs.instr,
                        pc: regs.pc,
                        pc_plus_4: regs.pc_plus_4,
                        predicted,
                    },
                    new_prediction,
                )
            })
            .take_while_inclusive(|(regs, new_predicted)| {
                new_predicted.is_none() && !matches!(regs.predicted, Prediction::TakenUnknown)
            })
            .map(|(regs, _)| regs)
            .collect();

        self.bp_regs.write(regs);
        new_prediction
    }

    pub(super) fn decode(&mut self) -> Result<Stall, Pc> {
        let circuits = self.decoders.iter_mut().zip(self.jump_agus.iter_mut());
        let mut new_prediction = None;
        let decoded = self
            .bp_regs
            .read()
            .iter()
            .copied()
            .zip(circuits)
            .map(|(regs, (decoder, jump_agu))| {
                let BpRegs {
                    instr,
                    pc,
                    pc_plus_4,
                    predicted,
                } = regs;
                let decoded = decoder.decode(instr);
                match decoded {
                    Ok(instr) => {
                        let (decoded, new_prediction) =
                            Self::decode_single(instr, jump_agu, pc, pc_plus_4, predicted);
                        (DecodeQueueEntry::Ok(decoded), new_prediction)
                    }
                    Err(ex) => (DecodeQueueEntry::Exception(ex, pc), None),
                }
            })
            .take_while_inclusive(|(_, new_prediction)| new_prediction.is_none())
            .map(|(decoded, prediction)| {
                debug_assert!(new_prediction.is_none());
                new_prediction = prediction;
                decoded
            })
            .collect::<Vec<_>>();

        let stall = !self.decode_queue.try_push(decoded);
        match new_prediction {
            _ if stall => Ok(true),
            None => Ok(stall),
            Some(new_prediction) => Err(new_prediction),
        }
    }

    fn decode_single(
        full: FullInstruction,
        jump_agu: &mut JumpAgu,
        pc: Pc,
        pc_plus_4: Pc,
        predicted: Prediction,
    ) -> (Decoded, Option<Pc>) {
        let instr = full.into();
        let mut new_prediction = None;
        let (predicted, target) = match instr {
            Instruction::Jump {
                base: AluSrcA::Reg(_),
                ..
            } => {
                let predicted = match predicted {
                    Prediction::Taken(predicted) => predicted,
                    Prediction::TakenUnknown => pc_plus_4,
                    Prediction::NotTaken => unreachable!(),
                };

                (predicted, 0)
            }
            Instruction::Jump {
                offset, apply_mask, ..
            } => {
                let target = jump_agu.jump_target(pc, offset, apply_mask);
                let predicted = match predicted {
                    Prediction::Taken(predicted) if predicted == target => predicted,
                    Prediction::TakenUnknown | Prediction::Taken(_) => {
                        new_prediction = Some(target);
                        target
                    }
                    Prediction::NotTaken => unreachable!(),
                };

                (predicted, target)
            }
            Instruction::Branch { offset, .. } => {
                let target = jump_agu.jump_target(pc, offset, false);
                let predicted = match predicted {
                    Prediction::NotTaken => pc_plus_4,
                    Prediction::Taken(predicted) if predicted == target => predicted,
                    Prediction::TakenUnknown | Prediction::Taken(_) => {
                        new_prediction = Some(target);
                        target
                    }
                };

                (predicted, target)
            }
            Instruction::Alu { .. }
            | Instruction::Load { .. }
            | Instruction::Store { .. }
            | Instruction::EnvTrap(_) => (pc_plus_4, pc_plus_4),
        };

        let decoded = Decoded {
            instr,
            full,
            pc,
            pc_plus_4,
            predicted,
            target,
        };
        (decoded, new_prediction)
    }
}
