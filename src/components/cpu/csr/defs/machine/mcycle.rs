super::use_csr_traits!();

#[derive(Debug)]
pub struct Mcycle;

impl CsrAccess for Mcycle {
    fn display_data(f: &mut dyn fmt::Write, data: CsrData) -> fmt::Result {
        write!(f, "{data}")
    }
}
