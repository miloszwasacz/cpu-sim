super::use_csr_traits!();

pub struct Fflags;

impl Fflags {
    pub(super) const MASK: CsrData = 0b11111;
}

impl CsrAccess for Fflags {
    fn alias_read(data: CsrData) -> CsrData {
        data & Self::MASK
    }

    fn alias_write(current: CsrData, new: CsrData) -> Result<CsrData, CsrError> {
        let current = current & !Self::MASK;
        let new = new & Self::MASK;
        Ok(current | new)
    }

    fn display_data(f: &mut dyn fmt::Write, data: CsrData) -> fmt::Result {
        const WIDTH: usize = Fflags::MASK.count_ones() as _;
        let data = Self::alias_read(data);

        write!(f, "{:0width$b}", data, width = WIDTH)
    }
}
