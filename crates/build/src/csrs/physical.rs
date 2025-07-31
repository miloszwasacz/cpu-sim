use super::CsrMeta;

use std::fs::File;
use std::io::Write;
use std::path::Path;

const MAP_FILE: &str = "csr_physical_map.rs";

pub fn generate_physical_csr_map(out_dir: &Path, csrs: &[CsrMeta]) {
    let mut map_file = File::create(out_dir.join(MAP_FILE)).unwrap();

    map_file.write_all(b"[\n").unwrap();
    for csr in csrs.iter().filter(|csr| csr.alias.is_none()) {
        map_file.write_all(b"(CsrAddr::").unwrap();
        map_file
            .write_all(csr.name.to_ascii_uppercase().as_bytes())
            .unwrap();
        map_file.write_all(b", Default::default()),\n").unwrap();
    }
    map_file.write_all(b"]").unwrap();
}
