super::use_csr_traits!();

pub struct Minstret;

impl CsrAccess for Minstret {
    fn display_data(f: &mut dyn fmt::Write, data: CsrData) -> fmt::Result {
        write!(f, "{data}")
    }
}
