use super::*;

/// A register holding the current value of a CSR.
#[derive(Debug, Default)]
pub struct PhysicalCsr {
    data: CsrData,
    explicit: Option<CsrData>,
    implicit: Option<CsrData>,
}

//TODO Add examples to documentation
impl PhysicalCsr {
    /// Reads the current value of the register.
    pub fn read(&self) -> CsrData {
        self.data
    }

    /// Writes to the register as part of a [CSR instruction](crate::instr::Instruction::Csr).
    pub fn explicit_write(&mut self, value: CsrData) {
        debug_assert!(self.explicit.is_none());
        self.explicit = Some(value);
    }

    /// Writes to the register as a side effect of some operation.
    pub fn implicit_write(&mut self, value: CsrData) {
        debug_assert!(self.implicit.is_none());
        self.implicit = Some(value);
    }
}

impl Sequential for PhysicalCsr {
    fn finish_cycle(&mut self) {
        let explicit = self.explicit.take();
        let implicit = self.implicit.take();
        if let Some(value) = explicit.or(implicit) {
            self.data = value;
        }
    }
}
