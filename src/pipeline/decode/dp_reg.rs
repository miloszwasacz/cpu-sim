//! Data Processing -- Register

use super::{extract_bits, invalid_instr, Instr, REG_MASK};

const OP0_SHIFT: u32 = 30;
const OP0_MASK: u32 = 0b1;

const OP1_SHIFT: u32 = 28;
const OP1_MASK: u32 = 0b1;

const OP2_SHIFT: u32 = 21;
const OP2_MASK: u32 = 0b1111;

pub fn decode(instr: u32) -> Box<dyn Instr> {
    let op0 = extract_bits!(instr, OP0_SHIFT, OP0_MASK);
    let op1 = extract_bits!(instr, OP1_SHIFT, OP1_MASK);
    let op2 = extract_bits!(instr, OP2_SHIFT, OP2_MASK);

    match (op0, op1, op2) {
        (0b0, 0b1, 0b0110) => dp_2src::decode(instr),
        (0b1, 0b1, 0b0110) => dp_1src::decode(instr),
        (_, 0b0, op2) if (op2 >> 3) == 0b0 => logic_shift::decode(instr),
        (_, 0b1, op2) if (op2 & 0b1001) == 0b1000 => add_sub_shift::decode(instr),
        (_, 0b1, op2) if (op2 >> 3) == 0b1 => dp_3src::decode(instr),
        _ => invalid_instr!(),
    }
}

mod dp_2src {
    use super::{extract_bits, invalid_instr, Instr, REG_MASK};
    use crate::instr::dp_reg::*;

    const SF_SHIFT: u32 = 31;
    const SF_MASK: u32 = 0b1;

    const S_SHIFT: u32 = 29;
    const S_MASK: u32 = 0b1;

    const OPCODE_SHIFT: u32 = 10;
    const OPCODE_MASK: u32 = 0b111111;

    const RM_SHIFT: u32 = 16;
    const RM_MASK: u32 = REG_MASK;

    const RN_SHIFT: u32 = 5;
    const RN_MASK: u32 = REG_MASK;

    const RD_SHIFT: u32 = 0;
    const RD_MASK: u32 = REG_MASK;

    pub fn decode(instr: u32) -> Box<dyn Instr> {
        let sf = extract_bits!(instr, SF_SHIFT, SF_MASK);
        let s = extract_bits!(instr, S_SHIFT, S_MASK);
        let opcode = extract_bits!(instr, OPCODE_SHIFT, OPCODE_MASK);
        let rm = extract_bits!(instr, RM_SHIFT, RM_MASK);
        let rn = extract_bits!(instr, RN_SHIFT, RN_MASK);
        let rd = extract_bits!(instr, RD_SHIFT, RD_MASK);

        if s != 0 {
            invalid_instr!()
        }
        match opcode {
            0b000010 => Box::new(Udiv::new(rd, rn, rm, sf)),
            0b000011 => Box::new(Sdiv::new(rd, rn, rm, sf)),
            0b001000 => Box::new(Lslv::new(rd, rn, rm, sf)),
            0b001001 => Box::new(Lsrv::new(rd, rn, rm, sf)),
            0b001010 => Box::new(Asrv::new(rd, rn, rm, sf)),
            0b001011 => Box::new(Rorv::new(rd, rn, rm, sf)),
            0b011000 => Box::new(Smax::new(rd, rn, rm, sf)),
            0b011001 => Box::new(Umax::new(rd, rn, rm, sf)),
            0b011010 => Box::new(Smin::new(rd, rn, rm, sf)),
            0b011011 => Box::new(Umin::new(rd, rn, rm, sf)),
            _ => invalid_instr!(),
        }
    }
}

mod dp_1src {
    use super::{extract_bits, invalid_instr, Instr, REG_MASK};
    use crate::instr::dp_reg::*;

    const SF_SHIFT: u32 = 31;
    const SF_MASK: u32 = 0b1;

    const S_SHIFT: u32 = 29;
    const S_MASK: u32 = 0b1;

    const OPCODE_SHIFT: u32 = 10;
    const OPCODE_MASK: u32 = 0b111111;

    const OPCODE2_SHIFT: u32 = 16;
    const OPCODE2_MASK: u32 = 0b11111;

    const RN_SHIFT: u32 = 5;
    const RN_MASK: u32 = REG_MASK;

    const RD_SHIFT: u32 = 0;
    const RD_MASK: u32 = REG_MASK;

    pub fn decode(instr: u32) -> Box<dyn Instr> {
        let sf = extract_bits!(instr, SF_SHIFT, SF_MASK);
        let s = extract_bits!(instr, S_SHIFT, S_MASK);
        let opcode = extract_bits!(instr, OPCODE_SHIFT, OPCODE_MASK);
        let opcode2 = extract_bits!(instr, OPCODE2_SHIFT, OPCODE2_MASK);
        let rn = extract_bits!(instr, RN_SHIFT, RN_MASK);
        let rd = extract_bits!(instr, RD_SHIFT, RD_MASK);

        if s != 0 {
            invalid_instr!()
        }
        match (opcode, opcode2) {
            (0b001000, 0b00000) => Box::new(Abs::new(rd, rn, sf)),
            _ => invalid_instr!(),
        }
    }
}

mod logic_shift {
    use super::{extract_bits, invalid_instr, Instr, REG_MASK};
    use crate::instr::dp_reg::*;

    const SF_SHIFT: u32 = 31;
    const SF_MASK: u32 = 0b1;

    const OPC_SHIFT: u32 = 29;
    const OPC_MASK: u32 = 0b11;

    const N_SHIFT: u32 = 21;
    const N_MASK: u32 = 0b1;

    const SHIFT_SHIFT: u32 = 22;
    const SHIFT_MASK: u32 = 0b11;

    const RM_SHIFT: u32 = 16;
    const RM_MASK: u32 = REG_MASK;

    const IMM6_SHIFT: u32 = 10;
    const IMM6_MASK: u32 = 0b111111;

    const RN_SHIFT: u32 = 5;
    const RN_MASK: u32 = REG_MASK;

    const RD_SHIFT: u32 = 0;
    const RD_MASK: u32 = REG_MASK;

    pub fn decode(instr: u32) -> Box<dyn Instr> {
        let sf = extract_bits!(instr, SF_SHIFT, SF_MASK);
        let opc = extract_bits!(instr, OPC_SHIFT, OPC_MASK);
        let n = extract_bits!(instr, N_SHIFT, N_MASK);
        let shift = extract_bits!(instr, SHIFT_SHIFT, SHIFT_MASK);
        let rm = extract_bits!(instr, RM_SHIFT, RM_MASK);
        let imm6 = extract_bits!(instr, IMM6_SHIFT, IMM6_MASK);
        let rn = extract_bits!(instr, RN_SHIFT, RN_MASK);
        let rd = extract_bits!(instr, RD_SHIFT, RD_MASK);

        match (opc, n) {
            (0b00, 0b0) => Box::new(And::new(rd, rn, rm, sf, shift, imm6)),
            (0b01, 0b0) => Box::new(Orr::new(rd, rn, rm, sf, shift, imm6)),
            (0b10, 0b0) => Box::new(Eor::new(rd, rn, rm, sf, shift, imm6)),
            (0b11, 0b0) => Box::new(Ands::new(rd, rn, rm, sf, shift, imm6)),
            _ => invalid_instr!(),
        }
    }
}

mod add_sub_shift {
    use super::{extract_bits, invalid_instr, Instr, REG_MASK};
    use crate::instr::dp_reg::*;

    const SF_SHIFT: u32 = 31;
    const SF_MASK: u32 = 0b1;

    const OP_SHIFT: u32 = 30;
    const OP_MASK: u32 = 0b1;

    const S_SHIFT: u32 = 29;
    const S_MASK: u32 = 0b1;

    const SHIFT_SHIFT: u32 = 22;
    const SHIFT_MASK: u32 = 0b11;

    const RM_SHIFT: u32 = 16;
    const RM_MASK: u32 = REG_MASK;

    const IMM6_SHIFT: u32 = 10;
    const IMM6_MASK: u32 = 0b111111;

    const RN_SHIFT: u32 = 5;
    const RN_MASK: u32 = REG_MASK;

    const RD_SHIFT: u32 = 0;
    const RD_MASK: u32 = REG_MASK;

    pub fn decode(instr: u32) -> Box<dyn Instr> {
        let sf = extract_bits!(instr, SF_SHIFT, SF_MASK);
        let op = extract_bits!(instr, OP_SHIFT, OP_MASK);
        let s = extract_bits!(instr, S_SHIFT, S_MASK);
        let shift = extract_bits!(instr, SHIFT_SHIFT, SHIFT_MASK);
        let rm = extract_bits!(instr, RM_SHIFT, RM_MASK);
        let imm6 = extract_bits!(instr, IMM6_SHIFT, IMM6_MASK);
        let rn = extract_bits!(instr, RN_SHIFT, RN_MASK);
        let rd = extract_bits!(instr, RD_SHIFT, RD_MASK);

        match (op, s) {
            (0b0, 0b0) => Box::new(Add::new(rd, rn, rm, sf, shift, imm6)),
            (0b0, 0b1) => Box::new(Adds::new(rd, rn, rm, sf, shift, imm6)),
            (0b1, 0b0) => Box::new(Sub::new(rd, rn, rm, sf, shift, imm6)),
            (0b1, 0b1) => Box::new(Subs::new(rd, rn, rm, sf, shift, imm6)),
            _ => invalid_instr!(),
        }
    }
}

mod dp_3src {
    use super::{extract_bits, invalid_instr, Instr, REG_MASK};
    use crate::instr::dp_reg::*;

    const SF_SHIFT: u32 = 31;
    const SF_MASK: u32 = 0b1;

    const OP54_SHIFT: u32 = 29;
    const OP54_MASK: u32 = 0b11;

    const OP31_SHIFT: u32 = 21;
    const OP31_MASK: u32 = 0b111;

    const RM_SHIFT: u32 = 16;
    const RM_MASK: u32 = REG_MASK;

    const O0_SHIFT: u32 = 15;
    const O0_MASK: u32 = 0b1;

    const RA_SHIFT: u32 = 10;
    const RA_MASK: u32 = REG_MASK;

    const RN_SHIFT: u32 = 5;
    const RN_MASK: u32 = REG_MASK;

    const RD_SHIFT: u32 = 0;
    const RD_MASK: u32 = REG_MASK;

    pub fn decode(instr: u32) -> Box<dyn Instr> {
        let sf = extract_bits!(instr, SF_SHIFT, SF_MASK);
        let op54 = extract_bits!(instr, OP54_SHIFT, OP54_MASK);
        let op31 = extract_bits!(instr, OP31_SHIFT, OP31_MASK);
        let rm = extract_bits!(instr, RM_SHIFT, RM_MASK);
        let o0 = extract_bits!(instr, O0_SHIFT, O0_MASK);
        let ra = extract_bits!(instr, RA_SHIFT, RA_MASK);
        let rn = extract_bits!(instr, RN_SHIFT, RN_MASK);
        let rd = extract_bits!(instr, RD_SHIFT, RD_MASK);

        match (op54, op31, o0) {
            (0b00, 0b000, 0b0) => Box::new(Madd::new(rd, rn, rm, ra, sf)),
            (0b00, 0b000, 0b1) => Box::new(Msub::new(rd, rn, rm, ra, sf)),
            _ => invalid_instr!(),
        }
    }
}
