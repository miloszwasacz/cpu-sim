use super::Instr;

use std::fs::File;
use std::io::Write;
use std::path::Path;

const DISPLAY_NAME_CONST: &str = "DISPLAY_NAME";
const IMPL_FILE: &str = "display_name_impls.rs";

pub fn generate_display_names(out_dir: &Path, instrs: &[Instr]) {
    let mut impl_file = File::create(out_dir.join(IMPL_FILE)).unwrap();

    for instr in instrs {
        impl_file.write_all(b"impl ").unwrap();
        impl_file.write_all(instr.name.as_bytes()).unwrap();
        impl_file.write_all(b" {\n  pub const ").unwrap();
        impl_file.write_all(DISPLAY_NAME_CONST.as_bytes()).unwrap();
        impl_file.write_all(b": &'static str = \"").unwrap();
        impl_file.write_all(instr.display_name.as_bytes()).unwrap();
        impl_file.write_all(b"\";\n}\n").unwrap();
    }
}
