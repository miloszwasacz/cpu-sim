use super::tags::{missing_tag, tag};
use super::Instr;

use std::fs::File;
use std::io::Write;
use std::path::Path;

const IMPL_FILE: &str = "instr_impls.rs";

const ENUM_TEMPLATE: &str = include_str!("../../res/enum.template.rs");
const ENUM_FILE: &str = "instr_enum.rs";

const TAG_INSTRS: &str = tag!("INSTRS");
const TAG_DISPLAYS: &str = tag!("DISPLAYS");
const TAG_FROMS: &str = tag!("FROMS");

pub fn generate_instr(out_dir: &Path, instrs: &[Instr]) {
    let mut impl_file = File::create(out_dir.join(IMPL_FILE)).unwrap();

    for instr in instrs {
        impl_file
            .write_all(b"impl crate::instr::Instr for ")
            .unwrap();
        impl_file.write_all(instr.name.as_bytes()).unwrap();
        impl_file.write_all(b" {}\n").unwrap();
    }
}

pub fn generate_enum(out_dir: &Path, instrs: &[Instr]) {
    let mut enum_file = File::create(out_dir.join(ENUM_FILE)).unwrap();

    let instrs_tag = ENUM_TEMPLATE
        .find(TAG_INSTRS)
        .unwrap_or_else(|| missing_tag!(TAG_INSTRS));
    let displays_tag = ENUM_TEMPLATE
        .find(TAG_DISPLAYS)
        .unwrap_or_else(|| missing_tag!(TAG_DISPLAYS));
    let froms_tag = ENUM_TEMPLATE
        .find(TAG_FROMS)
        .unwrap_or_else(|| missing_tag!(TAG_FROMS));

    let start = &ENUM_TEMPLATE[..instrs_tag];
    let middle1 = &ENUM_TEMPLATE[instrs_tag + TAG_INSTRS.len()..displays_tag];
    let middle2 = &ENUM_TEMPLATE[displays_tag + TAG_DISPLAYS.len()..froms_tag];
    let end = &ENUM_TEMPLATE[froms_tag + TAG_FROMS.len()..];

    enum_file.write_all(start.as_bytes()).unwrap();
    for instr in instrs {
        enum_file.write_all(instr.name.as_bytes()).unwrap();
        enum_file.write_all(b"(").unwrap();
        enum_file.write_all(instr.name.as_bytes()).unwrap();
        enum_file.write_all(b"),\n").unwrap();
    }
    enum_file.write_all(middle1.as_bytes()).unwrap();
    for instr in instrs {
        enum_file.write_all(b"Self::").unwrap();
        enum_file.write_all(instr.name.as_bytes()).unwrap();
        enum_file
            .write_all(b"(instr) => std::fmt::Display::fmt(instr, f),\n")
            .unwrap();
    }
    enum_file.write_all(middle2.as_bytes()).unwrap();
    for instr in instrs {
        enum_file.write_all(b"FullInstruction::").unwrap();
        enum_file.write_all(instr.name.as_bytes()).unwrap();
        enum_file.write_all(b"(instr) => instr.into(),\n").unwrap();
    }
    enum_file.write_all(end.as_bytes()).unwrap();
}
