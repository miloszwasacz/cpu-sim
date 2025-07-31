use crate::export_mod as csr_mod;

/// Adds all necessary imports to implement [`CsrAccess`](crate::components::cpu::csr::file::CsrAccess).
macro_rules! use_csr_traits {
    () => {
        #[allow(unused_imports)]
        use crate::components::cpu::csr::file::{Csr, CsrAccess, PhysicalCsr};
        #[allow(unused_imports)]
        use crate::components::cpu::csr::{CsrAddr, CsrData};
        #[allow(unused_imports)]
        use crate::components::cpu::error::CsrError;

        use std::fmt;
    };
}
use use_csr_traits;

csr_mod!(f_ext);
csr_mod!(machine);

mod impls {
    #[allow(unused_imports)]
    use super::*;
    #[allow(unused_imports)]
    use crate::components::cpu::csr::file::Csr;
    #[allow(unused_imports)]
    use crate::components::cpu::csr::CsrAddr;
    use crate::include_generated;

    include_generated!("csr_impls.rs");
}
