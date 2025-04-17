pub use self::adder::{JumpAgu, PcAdder};
pub use self::decoder::Decoder;
use super::error::{DecodeError, FetchError};
use super::reg::pipeline::{IdIsRegs, IfIdRegs};
use super::{Cpu, Pc};
use crate::components::memory::{Address, MemoryAccess};
use crate::instr::{AluSrcA, Instruction};
use crate::{BITS_IN_BYTE, IALIGN};
use crate::instr::raw::RawInstr;

mod adder;
mod decoder;

pub(super) type PcPlus4 = Pc;

impl Cpu<'_> {
    pub(super) fn fetch(&mut self) -> PcPlus4 {
        const ALIGN: Address = (IALIGN / BITS_IN_BYTE) as Address;
        let pc = *self.pc.read();
        let pc_plus_4 = self.pc_adder.add(pc);

        let instr = if pc % ALIGN == 0 {
            Ok(RawInstr::new(self.instr_mem.borrow().get(pc)))
        } else {
            Err(FetchError::MisalignedInstr(pc).into())
        };

        self.if_id_regs.write(Some(IfIdRegs {
            instr,
            pc,
            pc_plus_4,
        }));
        pc_plus_4
    }

    /// Returns a jump target if it could be pre-computed.
    pub(super) fn decode(&mut self) -> Option<Address> {
        let IfIdRegs {
            instr,
            pc,
            pc_plus_4,
        } = *self.if_id_regs.read().as_ref()?;
        let instr = match instr {
            Ok(instr) => instr,
            Err(ex) => {
                let regs = IdIsRegs {
                    instr: Err(ex),
                    pc,
                    pc_plus_4,
                    predicted: None,
                    target: None,
                };
                self.id_is_regs.write(Some(regs));
                return None;
            }
        };

        let (regs, jump) = match self.decoder.decode(instr) {
            Ok(full_instr) => {
                let instr = full_instr.into();
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
                            let jump = self.jump_agu.jump_target(pc, offset, apply_mask);
                            (Some(jump), None)
                        }
                    },
                    Instruction::Branch { offset, .. } => {
                        //TODO Branch prediction
                        let target = self.jump_agu.jump_target(pc, offset, false);
                        (Some(pc_plus_4), Some(target))
                    }
                    Instruction::Alu { .. }
                    | Instruction::Load { .. }
                    | Instruction::Store { .. }
                    | Instruction::EnvTrap(_) => (None, None),
                };

                let regs = IdIsRegs {
                    instr: Ok((instr, full_instr)),
                    pc,
                    pc_plus_4,
                    predicted,
                    target,
                };
                // TODO Uncomment when branch prediction is implemented
                // (Some(regs), predicted)
                (regs, None)
            }
            Err(instr) => {
                let err = DecodeError::InvalidInstruction(instr);
                let regs = IdIsRegs {
                    instr: Err(err.into()),
                    pc,
                    pc_plus_4,
                    predicted: None,
                    target: None,
                };
                (regs, None)
            }
        };

        self.id_is_regs.write(Some(regs));
        jump
    }
}
