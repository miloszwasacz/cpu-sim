//! Data Processing -- Immediate

use super::{decode, extract_bits, invalid_instr, Instr, REG_MASK};

const OP1_SHIFT: u32 = 22;
const OP1_MASK: u32 = 0b1111;

pub fn decode(instr: u32) -> Box<dyn Instr> {
    decode!(instr, OP1_SHIFT, OP1_MASK, match {
        0b0100 | 0b0101 => add_sub,
        0b0111 => min_max,
        0b1000 | 0b1001 => logical,
        0b1010 | 0b1011 => move_wide,
    })
}

mod add_sub {
    use super::{extract_bits, Instr, REG_MASK};
    use crate::instr::dp_imm::*;

    const SF_SHIFT: u32 = 31;
    const SF_MASK: u32 = 0b1;

    const OP_SHIFT: u32 = 30;
    const OP_MASK: u32 = 0b1;

    const S_SHIFT: u32 = 29;
    const S_MASK: u32 = 0b1;

    const SH_SHIFT: u32 = 22;
    const SH_MASK: u32 = 0b1;

    const IMM12_SHIFT: u32 = 10;
    const IMM12_MASK: u32 = 0b1111_1111_1111;

    const RN_SHIFT: u32 = 5;
    const RN_MASK: u32 = REG_MASK;

    const RD_SHIFT: u32 = 0;
    const RD_MASK: u32 = REG_MASK;

    pub fn decode(instr: u32) -> Box<dyn Instr> {
        let sf = extract_bits!(instr, SF_SHIFT, SF_MASK);
        let op = extract_bits!(instr, OP_SHIFT, OP_MASK);
        let s = extract_bits!(instr, S_SHIFT, S_MASK);
        let sh = extract_bits!(instr, SH_SHIFT, SH_MASK);
        let imm12 = extract_bits!(instr, IMM12_SHIFT, IMM12_MASK);
        let rn = extract_bits!(instr, RN_SHIFT, RN_MASK);
        let rd = extract_bits!(instr, RD_SHIFT, RD_MASK);

        match op {
            0 => match s {
                0 => Box::new(Add::new(rd, rn, sf, sh, imm12)),
                1 => Box::new(Adds::new(rd, rn, sf, sh, imm12)),
                _ => unreachable!(),
            },
            1 => match s {
                0 => Box::new(Sub::new(rd, rn, sf, sh, imm12)),
                1 => Box::new(Subs::new(rd, rn, sf, sh, imm12)),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
}

mod min_max {
    use super::{extract_bits, invalid_instr, Instr, REG_MASK};
    use crate::instr::dp_imm::*;

    const SF_SHIFT: u32 = 31;
    const SF_MASK: u32 = 0b1;

    const OP_SHIFT: u32 = 30;
    const OP_MASK: u32 = 0b1;

    const S_SHIFT: u32 = 29;
    const S_MASK: u32 = 0b1;

    const OPC_SHIFT: u32 = 18;
    const OPC_MASK: u32 = 0b1111;

    const IMM8_SHIFT: u32 = 10;
    const IMM8_MASK: u32 = 0b1111_1111;

    const RN_SHIFT: u32 = 5;
    const RN_MASK: u32 = REG_MASK;

    const RD_SHIFT: u32 = 0;
    const RD_MASK: u32 = REG_MASK;

    pub fn decode(instr: u32) -> Box<dyn Instr> {
        let sf = extract_bits!(instr, SF_SHIFT, SF_MASK);
        let op = extract_bits!(instr, OP_SHIFT, OP_MASK);
        let s = extract_bits!(instr, S_SHIFT, S_MASK);
        let opc = extract_bits!(instr, OPC_SHIFT, OPC_MASK);
        let imm8 = extract_bits!(instr, IMM8_SHIFT, IMM8_MASK);
        let rn = extract_bits!(instr, RN_SHIFT, RN_MASK);
        let rd = extract_bits!(instr, RD_SHIFT, RD_MASK);

        if op != 0 || s != 0 {
            invalid_instr!()
        }
        match opc {
            0b0000 => Box::new(Smax::new(rd, rn, sf, imm8)),
            0b0001 => Box::new(Umax::new(rd, rn, sf, imm8)),
            0b0010 => Box::new(Smin::new(rd, rn, sf, imm8)),
            0b0011 => Box::new(Umin::new(rd, rn, sf, imm8)),
            _ => invalid_instr!(),
        }
    }
}

mod logical {
    use super::{extract_bits, invalid_instr, Instr, REG_MASK};
    use crate::instr::dp_imm::*;

    const SF_SHIFT: u32 = 31;
    const SF_MASK: u32 = 0b1;

    const OPC_SHIFT: u32 = 29;
    const OPC_MASK: u32 = 0b11;

    const N_SHIFT: u32 = 22;
    const N_MASK: u32 = 0b1;

    const IMMR_SHIFT: u32 = 16;
    const IMMR_MASK: u32 = 0b111111;

    const IMMS_SHIFT: u32 = 10;
    const IMMS_MASK: u32 = 0b111111;

    const RN_SHIFT: u32 = 5;
    const RN_MASK: u32 = REG_MASK;

    const RD_SHIFT: u32 = 0;
    const RD_MASK: u32 = REG_MASK;

    pub fn decode(instr: u32) -> Box<dyn Instr> {
        let sf = extract_bits!(instr, SF_SHIFT, SF_MASK);
        let opc = extract_bits!(instr, OPC_SHIFT, OPC_MASK);
        let n = extract_bits!(instr, N_SHIFT, N_MASK);
        let immr = extract_bits!(instr, IMMR_SHIFT, IMMR_MASK);
        let imms = extract_bits!(instr, IMMS_SHIFT, IMMS_MASK);
        let rn = extract_bits!(instr, RN_SHIFT, RN_MASK);
        let rd = extract_bits!(instr, RD_SHIFT, RD_MASK);

        if sf == 0 && n != 0 {
            invalid_instr!()
        }
        match opc {
            0b00 => Box::new(And::new(rd, rn, sf, n, immr, imms)),
            0b01 => Box::new(Orr::new(rd, rn, sf, n, immr, imms)),
            0b10 => Box::new(Eor::new(rd, rn, sf, n, immr, imms)),
            0b11 => Box::new(Ands::new(rd, rn, sf, n, immr, imms)),
            _ => invalid_instr!(),
        }
    }
}

mod move_wide {
    use super::{extract_bits, invalid_instr, Instr, REG_MASK};
    use crate::instr::dp_imm::*;

    const SF_SHIFT: u32 = 31;
    const SF_MASK: u32 = 0b1;

    const OPC_SHIFT: u32 = 29;
    const OPC_MASK: u32 = 0b11;

    const HW_SHIFT: u32 = 21;
    const HW_MASK: u32 = 0b11;

    const IMM16_SHIFT: u32 = 5;
    const IMM16_MASK: u32 = 0b1111_1111_1111_1111;

    const RD_SHIFT: u32 = 0;
    const RD_MASK: u32 = REG_MASK;

    pub fn decode(instr: u32) -> Box<dyn Instr> {
        let sf = extract_bits!(instr, SF_SHIFT, SF_MASK);
        let opc = extract_bits!(instr, OPC_SHIFT, OPC_MASK);
        let hw = extract_bits!(instr, HW_SHIFT, HW_MASK);
        let imm16 = extract_bits!(instr, IMM16_SHIFT, IMM16_MASK);
        let rd = extract_bits!(instr, RD_SHIFT, RD_MASK);

        if sf == 0 && (hw & 0b10) != 0b00 {
            invalid_instr!()
        }
        match opc {
            // 0b00 => Box::new(Movn::new(rd, sf, imm16, hw)),
            0b10 => Box::new(Movz::new(rd, sf, imm16, hw)),
            // 0b11 => Box::new(Movk::new(rd, sf, imm16, hw)),
            _ => invalid_instr!(),
        }
    }
}
