use std::fmt::{self, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RegisterId {
    WSP,
    SP,
    WZR,
    XZR,
    W(GprId),
    X(GprId),
}

pub type GprId = u8;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RegisterSize {
    W,
    X,
}

impl RegisterId {
    pub const MAX_GPR: GprId = 30;

    pub fn decode(encoded: u32, size: RegisterSize, allow_sp: bool) -> Self {
        match encoded {
            reg if reg > 31 => panic!("GPR {reg} is not a valid register"),
            31 if allow_sp => match size {
                RegisterSize::W => RegisterId::WSP,
                RegisterSize::X => RegisterId::SP,
            },
            31 => match size {
                RegisterSize::W => RegisterId::WZR,
                RegisterSize::X => RegisterId::XZR,
            },
            reg => match size {
                RegisterSize::W => RegisterId::W(reg as GprId),
                RegisterSize::X => RegisterId::X(reg as GprId),
            },
        }
    }

    pub fn size(&self) -> RegisterSize {
        match self {
            RegisterId::WSP | RegisterId::WZR | RegisterId::W(_) => RegisterSize::W,
            RegisterId::SP | RegisterId::XZR | RegisterId::X(_) => RegisterSize::X,
        }
    }
}

impl From<u32> for RegisterSize {
    fn from(value: u32) -> Self {
        match value {
            0 => RegisterSize::W,
            1 => RegisterSize::X,
            s => panic!("{s} in not a valid register size"),
        }
    }
}

impl fmt::Display for RegisterId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let width = if f.alternate() { 3 } else { 0 };
        write!(
            f,
            "{:>width$}",
            match self {
                RegisterId::W(id) => format!("W{id}"),
                RegisterId::X(id) => format!("X{id}"),
                id => format!("{:?}", id),
            }
        )
    }
}
