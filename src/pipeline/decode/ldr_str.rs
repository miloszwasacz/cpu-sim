//! Loads and Stores

use super::{extract_bits, invalid_instr, Instr, REG_MASK};
use crate::instr::ldr_str::addr_mode::AddressingMode;

const OP0_SHIFT: u32 = 28;
const OP0_MASK: u32 = 0b1111;

const OP1_SHIFT: u32 = 26;
const OP1_MASK: u32 = 0b1;

const OP2_SHIFT: u32 = 25;
const OP2_MASK: u32 = 0b111_1111_1111_1111;

pub fn decode(instr: u32) -> Box<dyn Instr> {
    let op0 = extract_bits!(instr, OP0_SHIFT, OP0_MASK);
    let _op1 = extract_bits!(instr, OP1_SHIFT, OP1_MASK);
    let op2 = extract_bits!(instr, OP2_SHIFT, OP2_MASK);

    if op0 & 0b11 != 0b11 {
        invalid_instr!()
    }
    match op2 & 0b100_1000_0000_0011 {
        0b000_0000_0000_0001 => ldr_str_reg_imm::decode(instr, AddressingMode::PostIndex),
        0b000_0000_0000_0011 => ldr_str_reg_imm::decode(instr, AddressingMode::PreIndex),
        0b000_1000_0000_0010 => ldr_str_reg_reg_off::decode(instr),
        op2 if op2 & 0b100_0000_0000_0000 == 0b100_0000_0000_0000 => {
            ldr_str_reg_imm::decode(instr, AddressingMode::UnsignedOffset)
        }
        _ => invalid_instr!(),
    }
}

mod ldr_str_reg_imm {
    use super::{extract_bits, invalid_instr, REG_MASK};
    use crate::instr::ldr_str::addr_mode::AddressingMode;
    use crate::instr::ldr_str::ldr_str_reg_imm::*;
    use crate::instr::Instr;

    const SIZE_SHIFT: u32 = 30;
    const SIZE_MASK: u32 = 0b11;

    const VR_SHIFT: u32 = 26;
    const VR_MASK: u32 = 0b1;

    const OPC_SHIFT: u32 = 22;
    const OPC_MASK: u32 = 0b11;

    const IMM9_SHIFT: u32 = 12;
    const IMM9_MASK: u32 = 0b1_1111_1111;

    const IMM12_SHIFT: u32 = 10;
    const IMM12_MASK: u32 = 0b1111_1111_1111;

    const RN_SHIFT: u32 = 5;
    const RN_MASK: u32 = REG_MASK;

    const RT_SHIFT: u32 = 0;
    const RT_MASK: u32 = REG_MASK;

    pub fn decode(instr: u32, addr_mode: AddressingMode) -> Box<dyn Instr> {
        let size = extract_bits!(instr, SIZE_SHIFT, SIZE_MASK);
        let vr = extract_bits!(instr, VR_SHIFT, VR_MASK);
        let opc = extract_bits!(instr, OPC_SHIFT, OPC_MASK);
        let imm = match addr_mode {
            AddressingMode::PostIndex | AddressingMode::PreIndex => {
                extract_bits!(instr, IMM9_SHIFT, IMM9_MASK)
            }
            AddressingMode::UnsignedOffset => extract_bits!(instr, IMM12_SHIFT, IMM12_MASK),
        };
        let rn = extract_bits!(instr, RN_SHIFT, RN_MASK);
        let rt = extract_bits!(instr, RT_SHIFT, RT_MASK);

        match (size, vr, opc) {
            (0b00, 0b0, 0b00) => Box::new(Strb::new(rt, rn, imm, addr_mode)),
            (0b00, 0b0, 0b01) => Box::new(Ldrb::new(rt, rn, imm, addr_mode)),
            (0b00, 0b0, opc) => Box::new(Ldrsb::new(rt, rn, imm, opc, addr_mode)),
            (0b01, 0b0, 0b00) => Box::new(Strh::new(rt, rn, imm, addr_mode)),
            (0b01, 0b0, 0b01) => Box::new(Ldrh::new(rt, rn, imm, addr_mode)),
            (0b01, 0b0, opc) => Box::new(Ldrsh::new(rt, rn, imm, opc, addr_mode)),
            (0b10, 0b0, 0b00) | (0b11, 0b0, 0b00) => {
                Box::new(Str::new(rt, rn, imm, size, addr_mode))
            }
            (0b10, 0b0, 0b01) | (0b11, 0b0, 0b01) => {
                Box::new(Ldr::new(rt, rn, imm, size, addr_mode))
            }
            (0b10, 0b0, 0b10) => Box::new(Ldrsw::new(rt, rn, imm, addr_mode)),
            _ => invalid_instr!(),
        }
    }
}
mod ldr_str_reg_reg_off {
    use super::{extract_bits, invalid_instr, REG_MASK};
    use crate::instr::ldr_str::ldr_str_reg_reg_off::*;
    use crate::instr::Instr;

    const SIZE_SHIFT: u32 = 30;
    const SIZE_MASK: u32 = 0b11;

    const VR_SHIFT: u32 = 26;
    const VR_MASK: u32 = 0b1;

    const OPC_SHIFT: u32 = 22;
    const OPC_MASK: u32 = 0b11;
    
    const RM_SHIFT: u32 = 16;
    const RM_MASK: u32 = REG_MASK;

    const OPTION_SHIFT: u32 = 13;
    const OPTION_MASK: u32 = 0b111;

    const S_SHIFT: u32 = 12;
    const S_MASK: u32 = 0b1;

    const RN_SHIFT: u32 = 5;
    const RN_MASK: u32 = REG_MASK;

    const RT_SHIFT: u32 = 0;
    const RT_MASK: u32 = REG_MASK;

    pub fn decode(instr: u32) -> Box<dyn Instr> {
        let size = extract_bits!(instr, SIZE_SHIFT, SIZE_MASK);
        let vr = extract_bits!(instr, VR_SHIFT, VR_MASK);
        let opc = extract_bits!(instr, OPC_SHIFT, OPC_MASK);
        let rm = extract_bits!(instr, RM_SHIFT, RM_MASK);
        let option = extract_bits!(instr, OPTION_SHIFT, OPTION_MASK);
        let s = extract_bits!(instr, S_SHIFT, S_MASK);
        let rn = extract_bits!(instr, RN_SHIFT, RN_MASK);
        let rt = extract_bits!(instr, RT_SHIFT, RT_MASK);

        match (size, vr, opc) {
            (0b00, 0b0, 0b00) => Box::new(Strb::new(rt, rn, rm, option)),
            (0b00, 0b0, 0b01) => Box::new(Ldrb::new(rt, rn, rm, option)),
            (0b00, 0b0, opc) => Box::new(Ldrsb::new(rt, rn, rm, opc, option)),
            (0b01, 0b0, 0b00) => Box::new(Strh::new(rt, rn, rm, option, s)),
            (0b01, 0b0, 0b01) => Box::new(Ldrh::new(rt, rn, rm, option, s)),
            (0b01, 0b0, opc) => Box::new(Ldrsh::new(rt, rn, rm, opc, option, s)),
            (0b10, 0b0, 0b00) | (0b11, 0b0, 0b00) => {
                Box::new(Str::new(rt, rn, rm, option, s, size))
            }
            (0b10, 0b0, 0b01) | (0b11, 0b0, 0b01) => {
                Box::new(Ldr::new(rt, rn, rm, option, s, size))
            }
            (0b10, 0b0, 0b10) => Box::new(Ldrsw::new(rt, rn, rm, option, s)),
            _ => invalid_instr!(),
        }
    }
}
