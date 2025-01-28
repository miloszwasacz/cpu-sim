use std::error::Error;
use std::fmt;
use std::fmt::Formatter;
use std::str::FromStr;

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
            }
            31 => match size {
                RegisterSize::W => RegisterId::WZR,
                RegisterSize::X => RegisterId::XZR,
            }
            reg => match size {
                RegisterSize::W => RegisterId::W(reg as GprId),
                RegisterSize::X => RegisterId::X(reg as GprId),
            }
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
        match self {
            RegisterId::WSP => write!(f, "WSP"),
            RegisterId::SP => write!(f, "SP"),
            RegisterId::WZR => write!(f, "WZR"),
            RegisterId::XZR => write!(f, "XZR"),
            RegisterId::W(id) => write!(f, "W{}", id),
            RegisterId::X(id) => write!(f, "X{}", id),
        }
    }
}

impl FromStr for RegisterId {
    type Err = RegisterParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_gpr(s: String, size: char) -> Result<GprId, RegisterParseError> {
            s.trim_start_matches(size)
                .parse()
                .map_err(|_| RegisterParseError::UnknownIdentifier(s))
                .and_then(|reg| {
                    if reg > RegisterId::MAX_GPR {
                        Err(RegisterParseError::GprOutOfBounds(size, reg))
                    } else {
                        Ok(reg)
                    }
                })
        }

        let s = s.to_ascii_uppercase();
        match s.as_str() {
            "WSP" => Ok(RegisterId::WSP),
            "SP" => Ok(RegisterId::SP),
            "WZR" => Ok(RegisterId::WZR),
            "XZR" => Ok(RegisterId::XZR),
            gpr if gpr.starts_with('W') => parse_gpr(s, 'W').map(RegisterId::W),
            gpr if gpr.starts_with('X') => parse_gpr(s, 'X').map(RegisterId::X),
            _ => Err(RegisterParseError::UnknownIdentifier(s)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegisterParseError {
    UnknownIdentifier(String),
    GprOutOfBounds(char, GprId),
}

impl fmt::Display for RegisterParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            RegisterParseError::UnknownIdentifier(s) => {
                write!(f, "unknown register identifier: {s}")
            }
            RegisterParseError::GprOutOfBounds(size, reg) => write!(
                f,
                "GPR out of bounds: {reg} (available registers: {size}0..{size}{})",
                RegisterId::MAX_GPR
            ),
        }
    }
}

impl Error for RegisterParseError {}
