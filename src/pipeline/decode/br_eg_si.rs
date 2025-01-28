//! Branches, Exception Generation and System Instructions

use super::{extract_bits, invalid_instr, Instr, REG_MASK};

const OP0_SHIFT: u32 = 29;
const OP0_MASK: u32 = 0b111;

const OP1_SHIFT: u32 = 12;
const OP1_MASK: u32 = 0b11_1111_1111_1111;

const OP2_SHIFT: u32 = 0;
const OP2_MASK: u32 = 0b11111;

pub fn decode(instr: u32) -> Box<dyn Instr> {
    let op0 = extract_bits!(instr, OP0_SHIFT, OP0_MASK);
    let op1 = extract_bits!(instr, OP1_SHIFT, OP1_MASK);
    let op2 = extract_bits!(instr, OP2_SHIFT, OP2_MASK);

    match (op0, op1) {
        (0b010, op1) if (op1 >> 12) == 0b00 => cond_br_imm::decode(instr),
        (0b110, 0b01_0000_0011_0010) if op2 == 0b11111 => hints::decode(instr),
        (0b110, op1) if (op1 >> 13) == 0b1 => uncond_br_reg::decode(instr),
        (0b000, _) | (0b100, _) => uncond_br_imm::decode(instr),
        _ => invalid_instr!(instr),
    }
}

mod cond_br_imm {
    use super::{extract_bits, invalid_instr, Instr};
    use crate::instr::br_eg_si::*;

    const IMM19_SHIFT: u32 = 5;
    const IMM19_MASK: u32 = 0b111_1111_1111_1111_1111;

    const O0_SHIFT: u32 = 4;
    const O0_MASK: u32 = 0b1;

    const COND_SHIFT: u32 = 0;
    const COND_MASK: u32 = 0b1111;

    pub fn decode(instr: u32) -> Box<dyn Instr> {
        let imm19 = extract_bits!(instr, IMM19_SHIFT, IMM19_MASK);
        let o0 = extract_bits!(instr, O0_SHIFT, O0_MASK);
        let cond = extract_bits!(instr, COND_SHIFT, COND_MASK);

        match o0 {
            0b0 => Box::new(Bcond::new(cond, imm19)),
            _ => invalid_instr!(instr),
        }
    }
}

mod hints {
    use super::{extract_bits, invalid_instr, Instr};
    use crate::instr::br_eg_si::*;

    const CRM_SHIFT: u32 = 8;
    const CRM_MASK: u32 = 0b1111;

    const OP2_SHIFT: u32 = 5;
    const OP2_MASK: u32 = 0b111;

    pub fn decode(instr: u32) -> Box<dyn Instr> {
        let crm = extract_bits!(instr, CRM_SHIFT, CRM_MASK);
        let op2 = extract_bits!(instr, OP2_SHIFT, OP2_MASK);

        match (crm, op2) {
            (0b0000, 0b000) => Box::new(Nop::new()),
            _ => invalid_instr!(instr),
        }
    }
}

mod uncond_br_reg {
    use super::{extract_bits, invalid_instr, Instr, REG_MASK};
    use crate::instr::br_eg_si::*;

    const OPC_SHIFT: u32 = 21;
    const OPC_MASK: u32 = 0b1111;

    const OP2_SHIFT: u32 = 16;
    const OP2_MASK: u32 = 0b11111;

    const OP3_SHIFT: u32 = 10;
    const OP3_MASK: u32 = 0b111111;

    const RN_SHIFT: u32 = 5;
    const RN_MASK: u32 = REG_MASK;

    const OP4_SHIFT: u32 = 0;
    const OP4_MASK: u32 = 0b11111;

    pub fn decode(instr: u32) -> Box<dyn Instr> {
        let opc = extract_bits!(instr, OPC_SHIFT, OPC_MASK);
        let op2 = extract_bits!(instr, OP2_SHIFT, OP2_MASK);
        let op3 = extract_bits!(instr, OP3_SHIFT, OP3_MASK);
        let rn = extract_bits!(instr, RN_SHIFT, RN_MASK);
        let op4 = extract_bits!(instr, OP4_SHIFT, OP4_MASK);

        match (opc, op2, op3, rn, op4) {
            (0b0000, 0b11111, 0b000000, _, 0b00000) => Box::new(Br::new(rn)),
            (0b0010, 0b11111, 0b000000, _, 0b00000) => Box::new(Ret::new(rn)),
            _ => invalid_instr!(instr),
        }
    }
}

mod uncond_br_imm {
    use super::{extract_bits, invalid_instr, Instr};
    use crate::instr::br_eg_si::*;

    const OP_SHIFT: u32 = 31;
    const OP_MASK: u32 = 0b1;

    const IMM26_SHIFT: u32 = 0;
    const IMM26_MASK: u32 = 0b11_1111_1111_1111_1111_1111_1111;

    pub fn decode(instr: u32) -> Box<dyn Instr> {
        let op = extract_bits!(instr, OP_SHIFT, OP_MASK);
        let imm26 = extract_bits!(instr, IMM26_SHIFT, IMM26_MASK);

        match op {
            0b0 => Box::new(B::new(imm26)),
            // 0b1 => Box::new(Bl::new(imm26)),
            _ => invalid_instr!(instr),
        }
    }
}
