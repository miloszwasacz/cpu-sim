use super::CsrMeta;

use std::fs::File;
use std::io::Write;
use std::path::Path;

const CONST_FILE: &str = "csr_addr_consts.rs";

pub fn generate_addr_consts(out_dir: &Path, csrs: &[CsrMeta]) {
    let mut const_file = File::create(out_dir.join(CONST_FILE)).unwrap();

    const_file.write_all(b"impl CsrAddr {\n").unwrap();
    for csr in csrs {
        let name = csr.name.to_ascii_uppercase();
        const_file.write_all(b"    pub const ").unwrap();
        const_file.write_all(name.as_bytes()).unwrap();
        const_file.write_all(b": Self = Self(").unwrap();
        const_file.write_all(csr.addr.as_bytes()).unwrap();
        const_file.write_all(b");\n").unwrap();
    }
    const_file.write_all(b"}\n").unwrap();
}
