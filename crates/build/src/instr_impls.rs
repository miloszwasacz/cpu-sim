use self::decode::generate_decode;
use self::display::generate_display_names;
use self::instr::{generate_enum, generate_instr};
use self::instr_meta::Instr;

use std::path::Path;

mod decode;
mod display;
mod instr;
mod tags;

const SPEC: &str = include_str!("../res/decode.spec");

pub fn code_gen(out_dir: &Path) {
    let instrs = SPEC.lines().map(Instr::from_spec_line).collect::<Vec<_>>();

    generate_enum(out_dir, &instrs);
    generate_decode(out_dir, &instrs);
    generate_instr(out_dir, &instrs);
    generate_display_names(out_dir, &instrs);
}

mod instr_meta {
    use convert_case::{Boundary, Case, Casing};

    pub struct Instr<'a> {
        pub name: String,
        pub display_name: String,
        pub format: FormatType<'a>,
    }

    impl<'a> Instr<'a> {
        pub fn from_spec_line(line: &'a str) -> Self {
            macro_rules! next {
                ($line_iter:expr,$full_line:expr) => {
                    match $line_iter.next() {
                        Some(next) => next,
                        None => panic!("parsing error: {}", $full_line),
                    }
                };
            }
            let full_line = line;
            let mut line = line.split(';');

            let name_upper = next!(line, full_line);
            let boundaries = &[Boundary::from_delim(".")];
            let display_name = name_upper
                .with_boundaries(boundaries)
                .to_case(Case::Lower)
                .replace(" ", ".");
            let name = name_upper.with_boundaries(boundaries).to_case(Case::Pascal);

            let mut format = next!(line, full_line).split(',');
            let opcode = next!(format, full_line);
            assert!(!opcode.trim().is_empty(), "OPCODE must not be empty");

            let format = match format.next() {
                Some(funct3) => {
                    assert!(!funct3.trim().is_empty(), "FUNCT3 must not be empty");
                    let funct7 = next!(format, full_line);
                    assert!(!funct7.trim().is_empty(), "FUNCT7 must not be empty");
                    let funct6 = next!(format, full_line);
                    assert!(!funct6.trim().is_empty(), "FUNCT6 must not be empty");
                    FormatType::Normal(Format {
                        opcode,
                        funct3,
                        funct7,
                        funct6,
                    })
                }
                None => FormatType::Special(opcode),
            };

            Self {
                name,
                display_name,
                format,
            }
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub enum FormatType<'a> {
        Normal(Format<'a>),
        Special(&'a str),
    }

    impl FormatType<'_> {
        pub const NORMAL: u8 = 0;
        pub const SPECIAL: u8 = 1;
    }

    impl From<&FormatType<'_>> for u8 {
        fn from(value: &FormatType<'_>) -> Self {
            match value {
                FormatType::Normal(_) => FormatType::NORMAL,
                FormatType::Special(_) => FormatType::SPECIAL,
            }
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub struct Format<'a> {
        pub opcode: &'a str,
        pub funct3: &'a str,
        pub funct7: &'a str,
        pub funct6: &'a str,
    }
}
