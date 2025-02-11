use self::instr::instr_codegen;

use std::env;
use std::path::Path;

const BUILD_DIR: &str = "build";

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let build_files_dir = Path::new(&manifest_dir).join(BUILD_DIR);

    instr_codegen(&out_dir, build_files_dir);

    // println!("cargo::rerun-if-changed=build.rs");
}

mod instr {
    use super::BUILD_DIR;

    use convert_case::{Boundary, Case, Casing};
    use itertools::Itertools;
    use std::ffi::OsString;
    use std::path::{Path, PathBuf};
    use std::{fmt, fs};
    use std::cmp::Ordering;

    macro_rules! tag {
        ($tag:expr) => {
            format!("#{}!", $tag)
        };
        ($tag:expr, $suffix:expr) => {
            format!("#{}_{}!", $tag, $suffix)
        };
    }

    macro_rules! replace_tag {
        ($template:expr, $tag:expr, $contents:expr) => {
            $template.replace($tag.as_str(), format!("{}", $contents).as_str())
        };
    }

    const SPEC_FILE: &str = "decode.spec";
    const TEMPLATE_FILE: &str = "decode.template.rs";
    const OUT_DECODE_FN_FILE: &str = "decode_fn.rs";
    const OUT_DECODE_IMPLS_FILE: &str = "decode_impls.rs";
    const OUT_FROM_FORMAT_IMPLS_FILE: &str = "from_format_impls.rs";
    const OUT_DISPLAY_IMPLS_FILE: &str = "display_impls.rs";

    const TAG_SPECIAL: &str = "SPECIAL";
    const TAG_OPCODES: &str = "OPCODES";
    const TAG_DECODE: &str = "DECODE";

    pub fn instr_codegen(out_dir: &OsString, build_files_dir: PathBuf) {
        macro_rules! write_out {
            ($out:expr, $file:expr, $contents:expr) => {
                fs::write(Path::new($out).join($file), $contents).unwrap()
            };
        }

        let decode_spec_file = build_files_dir.join(SPEC_FILE);
        let decode_template_file = build_files_dir.join(TEMPLATE_FILE);
        let spec = fs::read_to_string(decode_spec_file).unwrap();
        let mut template = fs::read_to_string(decode_template_file).unwrap();

        let instrs = spec.lines().map(Instr::from_spec_line).collect::<Vec<_>>();

        let grouped = instrs
            .iter()
            .filter(|instr| !matches!(instr.format, Format::Special { .. }))
            .into_group_map_by(|instr| instr.format.ty());

        let special = DecodeSpecial(
            instrs
                .iter()
                .filter_map(|instr| match instr.format {
                    Format::Special { encoding } => Some((&instr.name, encoding)),
                    _ => None,
                })
                .collect::<Vec<_>>(),
        );
        template = replace_tag!(template, tag!(TAG_SPECIAL), special);

        let opcodes = grouped.iter().filter_map(|(format, group)| {
            let suffix = match format {
                FormatType::U | FormatType::J => return None,
                format => format.file_suffix(),
            };
            Some((suffix, group))
        });
        for (suffix, group) in opcodes {
            let opcodes = DecodeOpcodes(group);
            template = replace_tag!(template, tag!(TAG_OPCODES, suffix), opcodes);
        }

        let formats = grouped.iter().map(|(format, group)| {
            let names = group.iter().map(|instr| &instr.name).collect::<Vec<_>>();
            (*format, names)
        });
        for (format, names) in formats {
            let decode = DecodeFormat(format, names);
            template = replace_tag!(template, tag!(TAG_DECODE, format.file_suffix()), decode);
        }

        let decode_impls = format!("{}", Impls(&instrs, DecodeImpl));
        let from_format_impls = format!("{}", Impls(&instrs, FromFormatImpl));
        let display_impls = format!("{}", Impls(&instrs, DisplayImpl));

        write_out!(out_dir, OUT_DECODE_FN_FILE, template);
        write_out!(out_dir, OUT_DECODE_IMPLS_FILE, decode_impls);
        write_out!(out_dir, OUT_FROM_FORMAT_IMPLS_FILE, from_format_impls);
        write_out!(out_dir, OUT_DISPLAY_IMPLS_FILE, display_impls);

        println!("cargo::rerun-if-changed={}/{}", BUILD_DIR, SPEC_FILE);
        println!("cargo::rerun-if-changed={}/{}", BUILD_DIR, TEMPLATE_FILE);
    }

    type Group<'i, 's> = Vec<&'i Instr<'s>>;

    struct DecodeSpecial<'a>(Vec<(&'a String, &'a str)>);
    impl fmt::Display for DecodeSpecial<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            writeln!(f, "match instr.encoding() {{")?;
            for (name, encoding) in &self.0 {
                writeln!(
                    f,
                    "    0b{encoding} => return Ok(Rc::new({name}::decode(instr))),"
                )?;
            }
            writeln!(f, "   _ => {{}},")?;
            writeln!(f, "}}")
        }
    }

    struct DecodeOpcodes<'g, 'i, 's>(&'g Group<'i, 's>);
    impl fmt::Display for DecodeOpcodes<'_, '_, '_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let group = &self.0;
            for (i, instr) in group.iter().enumerate() {
                write!(f, "(x == {}::OPCODE)", instr.name)?;
                if i < group.len() - 1 {
                    writeln!(f, " ||")?;
                }
            }
            Ok(())
        }
    }

    struct DecodeFormat<'a>(FormatType, Vec<&'a String>);
    impl fmt::Display for DecodeFormat<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            for name in &self.1 {
                write!(f, "x if (x == ")?;
                match self.0 {
                    FormatType::R => write!(f, "({name}::OPCODE, {name}::FUNCT3, {name}::FUNCT7)")?,
                    FormatType::I(false) | FormatType::S | FormatType::B => {
                        write!(f, "({name}::OPCODE, {name}::FUNCT3)")?
                    }
                    FormatType::I(true) => {
                        write!(f, "({name}::OPCODE, {name}::FUNCT3) && shift_type == {name}::SHIFT_TYPE")?
                    }
                    FormatType::U | FormatType::J => write!(f, "{name}::OPCODE")?,
                }
                writeln!(f, ") => Rc::new({name}::decode(instr)),")?;
            }
            Ok(())
        }
    }

    struct DecodeImpl<'a, 'i>(&'a Instr<'i>);
    impl fmt::Display for DecodeImpl<'_, '_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                r#"
impl {} {{
    const OPCODE: Opcode = Opcode::new(0b{});
"#,
                self.0.name, self.0.opcode,
            )?;
            if let Format::IShift { shift_type, .. } = &self.0.format {
                writeln!(
                    f,
                    "    const SHIFT_TYPE: ShiftType = ShiftType::new(0b{});",
                    shift_type
                )?;
            }
            writeln!(f, "}}")?;

            if let Format::Special { .. } = self.0.format {
                return Ok(());
            }

            writeln!(f, "impl {}Type for {} {{", self.0.format.ty(), self.0.name)?;
            match &self.0.format {
                Format::R { funct3, funct7 } => {
                    write!(
                        f,
                        r#"
    const FUNCT3: Funct3 = Funct3::new(0b{});
    const FUNCT7: Funct7 = Funct7::new(0b{});
"#,
                        funct3, funct7,
                    )?;
                }
                Format::I { funct3 }
                | Format::S { funct3 }
                | Format::B { funct3 }
                | Format::IShift { funct3, .. } => {
                    writeln!(
                        f,
                        "   const FUNCT3: Funct3 = Funct3::new(0b{});",
                        funct3
                    )?;
                }
                Format::U | Format::J => {}
                Format::Special { .. } => unreachable!(),
            }
            write!(f, "}}")?;

            writeln!(
                f,
                r#"
impl Decode for {} {{
    fn decode(instr: RawInstr) -> Self
    where
        Self: Sized
    {{
        {}TypeFormat::{}(instr).into()
    }}
}}
"#,
                self.0.name,
                self.0.format.ty(),
                match self.0.format.ty() {
                    FormatType::I(true) => "decode_shift_instr",
                    _ => "decode",
                }
            )
        }
    }

    struct FromFormatImpl<'a, 'i>(&'a Instr<'i>);
    impl fmt::Display for FromFormatImpl<'_, '_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            if !self.0.auto_from {
                return Ok(());
            }

            write!(
                f,
                r#"
impl From<{format}TypeFormat> for {name} {{
    fn from(value: {format}TypeFormat) -> Self {{
        Self(value)
    }}
}}
"#,
                name = self.0.name,
                format = self.0.format.ty(),
            )
        }
    }

    struct DisplayImpl<'a, 'i>(&'a Instr<'i>);
    impl fmt::Display for DisplayImpl<'_, '_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                r#"
impl {} {{
    const DISPLAY_NAME: &'static str = "{}";
}}
"#,
                self.0.name, self.0.display_name,
            )?;

            if self.0.auto_display {
                write!(
                    f,
                    r#"
impl std::fmt::Display for {} {{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{
        let width = display_width!(f);
        write!(f, "{{:<width$}} ", Self::DISPLAY_NAME)?;
        self.0.fmt(f)
    }}
}}
"#,
                    self.0.name
                )?;
            }
            Ok(())
        }
    }

    struct Impls<'a, 'i, F>(&'a Vec<Instr<'i>>, F);
    impl<'a, 'i, T, F> fmt::Display for Impls<'a, 'i, F>
    where
        T: fmt::Display,
        F: Fn(&'a Instr<'i>) -> T,
    {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            for instr in self.0 {
                let imp = self.1(instr);
                writeln!(f, "{}", imp)?;
            }
            Ok(())
        }
    }

    struct Instr<'a> {
        name: String,
        display_name: String,
        opcode: &'a str,
        format: Format<'a>,
        auto_from: bool,
        auto_display: bool,
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
            macro_rules! auto_derive {
                ($input:expr, $format:expr) => {
                    match $input {
                        "auto" => {
                            if matches!($format, Format::Special { .. }) {
                                panic!("special instructions cannot auto-derive")
                            }
                            true
                        }
                        "custom" => false,
                        specifier => panic!("'{specifier}' is not a valid auto-derive specifier"),
                    }
                };
            }
            let full_line = line;
            let mut line = line.split(',');

            let name_upper = next!(line, full_line);
            let boundaries = &[Boundary::from_delim(".")];
            let display_name = name_upper
                .with_boundaries(boundaries)
                .to_case(Case::Lower)
                .replace(" ", ".");
            let name = name_upper.with_boundaries(boundaries).to_case(Case::Pascal);

            let format = next!(line, full_line);
            let auto_from = next!(line, full_line);
            let auto_display = next!(line, full_line);
            let opcode = next!(line, full_line);

            let format = match format {
                "R" => {
                    let funct3 = next!(line, full_line);
                    let funct7 = next!(line, full_line);
                    Format::R { funct3, funct7 }
                }
                "I" => {
                    let funct3 = next!(line, full_line);
                    Format::I { funct3 }
                }
                "S" => {
                    let funct3 = next!(line, full_line);
                    Format::S { funct3 }
                }
                "B" => {
                    let funct3 = next!(line, full_line);
                    Format::B { funct3 }
                }
                "U" => Format::U,
                "J" => Format::J,
                "I_Shift" => {
                    let funct3 = next!(line, full_line);
                    let shift_type = next!(line, full_line);
                    Format::IShift { funct3, shift_type }
                }
                "Special" => {
                    let encoding = next!(line, full_line);
                    Format::Special { encoding }
                }
                format => panic!("{} is not a valid format", format),
            };
            let auto_from = auto_derive!(auto_from, format);
            let auto_display = auto_derive!(auto_display, format);

            Self {
                name,
                display_name,
                opcode,
                format,
                auto_from,
                auto_display,
            }
        }
    }

    #[derive(Clone, Copy)]
    enum Format<'a> {
        R {
            funct3: &'a str,
            funct7: &'a str,
        },
        I {
            funct3: &'a str,
        },
        S {
            funct3: &'a str,
        },
        B {
            funct3: &'a str,
        },
        U,
        J,
        IShift {
            funct3: &'a str,
            shift_type: &'a str,
        },
        Special {
            encoding: &'a str,
        },
    }

    impl Format<'_> {
        pub fn ty(&self) -> FormatType {
            match self {
                Format::R { .. } => FormatType::R,
                Format::I { .. } => FormatType::I(false),
                Format::S { .. } => FormatType::S,
                Format::B { .. } => FormatType::B,
                Format::U => FormatType::U,
                Format::J => FormatType::J,
                Format::IShift { .. } => FormatType::I(true),
                Format::Special { .. } => panic!("special format"),
            }
        }
    }

    type IsShift = bool;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    enum FormatType {
        R,
        I(IsShift),
        S,
        B,
        U,
        J,
    }
    
    impl FormatType {
        pub fn file_suffix(&self) -> &'static str {
            match self {
                FormatType::R => "r",
                FormatType::I(false) => "i",
                FormatType::I(true) => "i_shift",
                FormatType::S => "s",
                FormatType::B => "b",
                FormatType::U => "u",
                FormatType::J => "j",
            }
        }
    }

    impl fmt::Display for FormatType {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                FormatType::I(_) => write!(f, "I"),
                ty => write!(f, "{:?}", ty),
            }
        }
    }

    impl PartialOrd for FormatType {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            u8::from(self).partial_cmp(&u8::from(other))
        }
    }

    impl From<&FormatType> for u8 {
        fn from(value: &FormatType) -> Self {
            match value {
                FormatType::I(true) => 0,
                FormatType::R => 1,
                FormatType::I(false) => 2,
                FormatType::S => 3,
                FormatType::B => 4,
                FormatType::U => 5,
                FormatType::J => 6,
            }
        }
    }
}
