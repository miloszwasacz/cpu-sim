use super::NonZeroLatency;
use crate::components::cpu::reg::RegData;
use crate::components::memory::Address;

//TODO Variable latencies for division/remainder
const MUL_LATENCY: NonZeroLatency = NonZeroLatency::new(2).unwrap();
const MULH_LATENCY: NonZeroLatency = NonZeroLatency::new(3).unwrap();
const DIV_LATENCY_MIN: NonZeroLatency = NonZeroLatency::new(5).unwrap();
#[allow(dead_code)] //TODO Remove
const DIV_LATENCY_MAX: NonZeroLatency = NonZeroLatency::new(20).unwrap();
const DIVW_LATENCY_MIN: NonZeroLatency = NonZeroLatency::new(5).unwrap();
#[allow(dead_code)] //TODO Remove
const DIVW_LATENCY_MAX: NonZeroLatency = NonZeroLatency::new(12).unwrap();

#[derive(Clone)]
pub(in crate::components::cpu) struct Mul;

impl Mul {
    const MULH_SHIFT: u32 = Address::BITS;

    pub fn process(
        &mut self,
        op: MulControl,
        src_a: RegData,
        src_b: RegData,
    ) -> (RegData, NonZeroLatency) {
        match op {
            // 64-bit
            MulControl::Mul => (
                RegData::signed(src_a.i().wrapping_mul(src_b.i())),
                MUL_LATENCY,
            ),
            MulControl::Mulh => {
                let src_a = src_a.i() as i128;
                let src_b = src_b.i() as i128;
                let out = (src_a * src_b) >> Self::MULH_SHIFT;
                (RegData::signed(out as _), MULH_LATENCY)
            }
            MulControl::Mulhsu => {
                let src_a = src_a.i() as i128;
                let src_b = src_b.u() as u128 as i128;
                let out = (src_a * src_b) >> Self::MULH_SHIFT;
                (RegData::signed(out as _), MULH_LATENCY)
            }
            MulControl::Mulhu => {
                let src_a = src_a.u() as u128;
                let src_b = src_b.u() as u128;
                let out = (src_a * src_b) >> Self::MULH_SHIFT;
                (RegData::unsigned(out as _), MULH_LATENCY)
            }
            MulControl::Div => (
                RegData::signed(match src_b.i() {
                    0 => -1,
                    src_b => src_a.i().overflowing_div(src_b).0,
                }),
                DIV_LATENCY_MIN,
            ),
            MulControl::Divu => (
                RegData::unsigned(src_a.u().checked_div(src_b.u()).unwrap_or(u64::MAX)),
                DIV_LATENCY_MIN,
            ),
            MulControl::Rem => (
                match src_b.i() {
                    0 => src_a,
                    src_b => RegData::signed(src_a.i().checked_rem(src_b).unwrap_or(0)),
                },
                DIV_LATENCY_MIN,
            ),
            MulControl::Remu => (
                src_a
                    .u()
                    .checked_div(src_b.u())
                    .map(RegData::unsigned)
                    .unwrap_or(src_a),
                DIV_LATENCY_MIN,
            ),

            // 32-bit
            MulControl::Mulw => {
                let src_a = src_a.i() as i32;
                let src_b = src_b.i() as i32;
                (RegData::signed(src_a.wrapping_mul(src_b) as _), MUL_LATENCY)
            }
            MulControl::Divw => {
                let src_a = src_a.i() as i32;
                let src_b = src_b.i() as i32;
                (
                    RegData::signed(match src_b {
                        0 => -1,
                        src_b => src_a.overflowing_div(src_b).0,
                    } as _),
                    DIVW_LATENCY_MIN,
                )
            }
            MulControl::Divuw => {
                let src_a = src_a.u() as u32;
                let src_b = src_b.u() as u32;
                (
                    RegData::unsigned(src_a.checked_div(src_b).unwrap_or(u32::MAX) as _),
                    DIVW_LATENCY_MIN,
                )
            }
            MulControl::Remw => {
                let src_a = src_a.i() as i32;
                let src_b = src_b.i() as i32;
                (
                    RegData::signed(match src_b {
                        0 => src_a,
                        src_b => src_a.checked_rem(src_b).unwrap_or(0),
                    } as _),
                    DIVW_LATENCY_MIN,
                )
            }
            MulControl::Remuw => {
                let src_a = src_a.u() as u32;
                let src_b = src_b.u() as u32;
                (
                    RegData::unsigned(src_a.checked_rem(src_b).unwrap_or(src_a) as _),
                    DIVW_LATENCY_MIN,
                )
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MulControl {
    // 64-bit
    Mul,
    Mulh,
    Mulhsu,
    Mulhu,
    Div,
    Divu,
    Rem,
    Remu,

    // 32-bit
    Mulw,
    Divw,
    Divuw,
    Remw,
    Remuw,
}
