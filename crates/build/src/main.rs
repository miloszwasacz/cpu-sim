use std::env;
use std::path::Path;

mod csrs;
mod instr_impls;

const OUT_DIR: &str = "OUT_DIR";

pub fn main() {
    let out_dir = Path::new(&env::var_os(OUT_DIR).unwrap()).to_owned();

    instr_impls::code_gen(&out_dir);
    csrs::code_gen(&out_dir);

    println!("cargo::rerun-if-changed=crates/build");
}
