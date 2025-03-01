use super::instr::{Format, FormatType, Instr};

use itertools::Itertools;
use std::fs::File;
use std::io::Write;
use std::path::Path;

macro_rules! tag {
    ($tag:literal) => {
        const_format::formatcp!("#{}!", $tag)
    };
}

macro_rules! missing_tag {
    ($tag:expr) => {
        panic!("could not find tag `{}`", $tag)
    };
}

const ENCODING_CONST: &str = "ENCODING";
const TEMPLATE: &str = include_str!("../../res/decode.template.rs");
const IMPL_FILE: &str = "decode_impls.rs";

const TAG_SPECIAL: &str = tag!("SPECIAL");
const TAG_ENCODINGS: &str = tag!("ENCODINGS");

pub fn generate_decode(out_dir: &Path, instrs: &[Instr]) {
    let mut impl_file = File::create(out_dir.join(IMPL_FILE)).unwrap();

    let types = instrs
        .iter()
        .into_group_map_by(|instr| u8::from(&instr.format));
    let normal = &types[&FormatType::NORMAL];
    let special = &types[&FormatType::SPECIAL];

    let special_tag = TEMPLATE
        .find(TAG_SPECIAL)
        .unwrap_or_else(|| missing_tag!(TAG_SPECIAL));
    let encodings_tag = TEMPLATE
        .find(TAG_ENCODINGS)
        .unwrap_or_else(|| missing_tag!(TAG_ENCODINGS));

    let start = &TEMPLATE[0..special_tag];
    let middle = &TEMPLATE[special_tag + TAG_SPECIAL.len()..encodings_tag];
    let end = &TEMPLATE[encodings_tag + TAG_ENCODINGS.len()..];

    impl_file.write_all(start.as_bytes()).unwrap();
    for instr in special {
        let encoding = match instr.format {
            FormatType::Special(enc) => enc,
            FormatType::Normal(_) => unreachable!(),
        };

        impl_file.write_all(b"0b").unwrap();
        impl_file.write_all(encoding.as_bytes()).unwrap();
        impl_file.write_all(b" => return Ok(Box::new(").unwrap();
        impl_file.write_all(instr.name.as_bytes()).unwrap();
        impl_file.write_all(b"::decode(instr))),\n").unwrap();
    }
    impl_file.write_all(middle.as_bytes()).unwrap();
    for instr in normal {
        let Format {
            opcode,
            funct3,
            funct7,
        } = match instr.format {
            FormatType::Normal(format) => format,
            FormatType::Special(_) => unreachable!(),
        };
        impl_file.write_all(b"(").unwrap();
        if opcode != "_" {
            impl_file.write_all(b"0b").unwrap();
        }
        impl_file.write_all(opcode.as_bytes()).unwrap();
        impl_file.write_all(b", ").unwrap();
        if funct3 != "_" {
            impl_file.write_all(b"0b").unwrap();
        }
        impl_file.write_all(funct3.as_bytes()).unwrap();
        impl_file.write_all(b", ").unwrap();
        if funct7 != "_" {
            impl_file.write_all(b"0b").unwrap();
        }
        impl_file.write_all(funct7.as_bytes()).unwrap();
        impl_file.write_all(b") => Box::new(").unwrap();
        impl_file.write_all(instr.name.as_bytes()).unwrap();
        impl_file.write_all(b"::decode(instr)),\n").unwrap();
    }
    impl_file.write_all(end.as_bytes()).unwrap();

    for instr in special {
        let encoding = match instr.format {
            FormatType::Special(enc) => enc,
            FormatType::Normal(_) => unreachable!(),
        };

        impl_file
            .write_all(b"\n#[cfg(debug_assertions)]\nimpl ")
            .unwrap();
        impl_file.write_all(instr.name.as_bytes()).unwrap();
        impl_file.write_all(b" {\n    pub(crate) const ").unwrap();
        impl_file.write_all(ENCODING_CONST.as_bytes()).unwrap();
        impl_file
            .write_all(b": crate::instr::raw::RawInstrBits = 0b")
            .unwrap();
        impl_file.write_all(encoding.as_bytes()).unwrap();
        impl_file.write_all(b";\n}\n").unwrap();
    }
}
