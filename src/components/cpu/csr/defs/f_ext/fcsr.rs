super::use_csr_traits!();
use super::{Fflags, Frm};

#[derive(Debug)]
pub struct Fcsr;

impl Fcsr {
    pub(super) const MASK: CsrData = {
        let mask = (Frm::MASK << Frm::SHIFT) | Fflags::MASK;
        if mask.count_ones() != mask.trailing_ones() {
            panic!("invalid `fcsr` mask");
        }
        mask
    };
}

impl CsrAccess for Fcsr {
    fn alias_write(_: CsrData, new: CsrData) -> Result<CsrData, CsrError> {
        Ok(new & Self::MASK)
    }

    fn display_data(f: &mut dyn fmt::Write, data: CsrData) -> fmt::Result {
        const WIDTH: usize = 32;
        const SHIFT: u32 = Fcsr::MASK.count_ones();
        let data = Self::alias_read(data);

        let reserved = data >> SHIFT;
        write!(f, "{:0width$b} ", reserved, width = WIDTH - SHIFT as usize)?;
        Frm::display_data(f, data)?;
        write!(f, " ")?;
        Fflags::display_data(f, data)
    }
}
