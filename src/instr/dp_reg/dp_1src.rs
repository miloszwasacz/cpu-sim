use crate::instr::Instr;
use crate::reg::RegisterId;

struct Dp1SrcInstr {
    d: RegisterId,
    n: RegisterId,
}

impl Dp1SrcInstr {
    fn new(d: u32, n: u32, sf: u32) -> Self {
        let size = sf.into();
        let d = RegisterId::decode(d, size, false);
        let n = RegisterId::decode(n, size, false);

        Self { d, n }
    }
}

macro_rules! dp_1src_instr {
    ($name:ident) => {
        pub struct $name(Dp1SrcInstr);

        impl $name {
            pub fn new(d: u32, n: u32, sf: u32) -> Self {
                Self(Dp1SrcInstr::new(d, n, sf))
            }
        }

        impl Instr for $name {}
    };
}

//#region ABS

dp_1src_instr!(Abs);

//#endregion
