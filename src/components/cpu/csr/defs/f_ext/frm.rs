super::use_csr_traits!();
use super::Fflags;

pub struct Frm;

impl Frm {
    pub(super) const MASK: CsrData = 0b111;
    pub(super) const SHIFT: u32 = Fflags::MASK.count_ones();
}

impl CsrAccess for Frm {
    fn alias_read(data: CsrData) -> CsrData {
        (data >> Self::SHIFT) & Self::MASK
    }

    fn alias_write(current: CsrData, new: CsrData) -> Result<CsrData, CsrError> {
        let current = current & !(Self::MASK << Self::SHIFT);
        let new = (new & Self::MASK) << Self::SHIFT;
        Ok(current | new)
    }

    fn display_data(f: &mut dyn fmt::Write, data: CsrData) -> fmt::Result {
        const WIDTH: usize = Frm::MASK.count_ones() as _;
        let data = Self::alias_read(data);

        write!(f, "{:0width$b}", data, width = WIDTH)
    }
}
