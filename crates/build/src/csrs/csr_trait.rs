use super::CsrMeta;

use convert_case::{Case, Casing};
use std::fs::File;
use std::io::Write;
use std::path::Path;

const IMPL_FILE: &str = "csr_impls.rs";

pub fn generate_csr_impls(out_dir: &Path, csrs: &[CsrMeta]) {
    let mut impl_file = File::create(out_dir.join(IMPL_FILE)).unwrap();

    for csr in csrs {
        impl_file.write_all(b"impl Csr for ").unwrap();
        impl_file
            .write_all(csr.name.to_case(Case::Pascal).as_bytes())
            .unwrap();
        impl_file.write_all(b" {\n").unwrap();

        // NAME
        impl_file
            .write_all(b"    const NAME: &'static str = \"")
            .unwrap();
        impl_file.write_all(csr.name.as_bytes()).unwrap();
        impl_file.write_all(b"\";\n").unwrap();

        // ADDRESS
        impl_file
            .write_all(b"    const ADDRESS: CsrAddr = CsrAddr::")
            .unwrap();
        impl_file
            .write_all(csr.name.to_ascii_uppercase().as_bytes())
            .unwrap();
        impl_file.write_all(b";\n").unwrap();

        // PHYSICAL_ADDRESS
        if let Some(alias) = csr.alias {
            impl_file
                .write_all(b"    const PHYSICAL_ADDRESS: CsrAddr = ")
                .unwrap();
            impl_file
                .write_all(alias.to_case(Case::Pascal).as_bytes())
                .unwrap();
            impl_file.write_all(b"::PHYSICAL_ADDRESS;\n").unwrap();
        }

        impl_file.write_all(b"}\n").unwrap();
    }
}
