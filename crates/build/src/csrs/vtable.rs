use super::{AddrType, CsrMeta};

use convert_case::{Case, Casing};
use std::fs::File;
use std::io::Write;
use std::path::Path;

const VTABLE_FILE: &str = "csr_vtable.rs";

pub fn generate_csr_vtable(out_dir: &Path, csrs: &[CsrMeta]) {
    let mut vtable_file = File::create(out_dir.join(VTABLE_FILE)).unwrap();

    let mut builder = phf_codegen::Map::<AddrType>::new();
    for csr in csrs {
        let info = format!("CsrInfo::new::<{}>()", csr.name.to_case(Case::Pascal));
        builder.entry(csr.addr_parsed, info);
    }
    let map = builder.build();

    write!(vtable_file, "{}", map).unwrap();
}
