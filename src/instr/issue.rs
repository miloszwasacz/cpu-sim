use crate::components::cpu::reg::RegData;

pub trait Issue {
    fn branch(&self) -> Branch;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Branch {
    #[default]
    None,
    Jump,
    Eq,
    Ne,
    Lt,
    Ltu,
    Ge,
    Geu,
}

impl Branch {
    pub fn jumps(&self, src1: RegData, src2: RegData) -> bool {
        match self {
            Branch::None => false,
            Branch::Jump => true,
            Branch::Eq => src1.i() == src2.i(),
            Branch::Ne => src1.i() != src2.i(),
            Branch::Lt => src1.i() < src2.i(),
            Branch::Ltu => src1.u() < src2.u(),
            Branch::Ge => src1.i() >= src2.i(),
            Branch::Geu => src1.u() >= src2.u(),
        }
    }
}
