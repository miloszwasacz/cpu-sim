use crate::components::cpu::Cpu;
use crate::components::memory::{Address, Memory};

use elf::endian::LittleEndian;
use elf::ElfBytes;
use std::error::Error;
use std::path::Path;
use std::{fmt, fs, io};

pub struct Loader;

impl Loader {
    const ERRNO_PTR_NAME: &'static str = "_impure_ptr";
    const _END_NAME: &'static str = "_end";

    /// Loads the program to memory and initializes the CPU's PC.
    ///
    /// # Safety
    ///
    /// This method directly sets `cpu`'s Program Counter. Calling this while the `cpu`
    /// is in the middle of executing a program might lead to undefined behavior[^1].
    /// After finishing a program, the `cpu` should be [`reset`](Cpu::reset)
    /// before loading another program.
    ///
    /// [^1]: By _undefined behavior_ we mean the simulation might exhibit undefined behavior.
    ///       This method is still _safe_ in the [Rust sense](https://doc.rust-lang.org/reference/unsafety.html).
    pub unsafe fn load<P: AsRef<Path>>(
        &self,
        bin: P,
        mem: &mut Memory,
        cpu: &mut Cpu,
    ) -> Result<(), LoaderError> {
        let bin = fs::read(bin)?;

        let elf = ElfBytes::<LittleEndian>::minimal_parse(&bin)
            .map_err(|err| LoaderError::ParseError("Binary should be a valid ELF file", err))?;
        let segments = elf
            .segments()
            .ok_or(LoaderError::MissingProgramHeaderTable)?;

        for segment in segments {
            let offset = segment.p_offset as usize;
            let addr = segment.p_vaddr as Address;
            let size = segment.p_filesz.min(segment.p_memsz) as usize;

            mem[addr..(addr + size as Address)].copy_from_slice(&bin[offset..(offset + size)])
        }

        unsafe { cpu.set_pc(elf.ehdr.e_entry as Address) };
        cpu.os()
            .set_errno_addr(Self::get_symbol_addr(&elf, Self::ERRNO_PTR_NAME)?);
        cpu.os().set__end_addr(
            Self::get_symbol_addr(&elf, Self::_END_NAME)?.ok_or(LoaderError::Missing_End)?,
        );

        Ok(())
    }

    fn get_symbol_addr(
        elf: &ElfBytes<LittleEndian>,
        symbol_name: &str,
    ) -> Result<Option<Address>, LoaderError> {
        let (symtab, strtab) = elf
            .symbol_table()
            .map_err(|err| LoaderError::ParseError("`shdrs` should parse", err))?
            .ok_or(LoaderError::MissingSymbolTable)?;

        let sym_addr = symtab
            .iter()
            .find(|sym| {
                strtab
                    .get(sym.st_name as usize)
                    .map(|name| name == symbol_name)
                    .unwrap_or(false)
            })
            .map(|sym| sym.st_value as Address);

        Ok(sym_addr)
    }
}

#[derive(Debug)]
pub enum LoaderError {
    Io(io::Error),
    ParseError(&'static str, elf::ParseError),
    MissingProgramHeaderTable,
    #[allow(non_camel_case_types)]
    Missing_End,
    MissingSymbolTable,
}

impl fmt::Display for LoaderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoaderError::Io(err) => write!(f, "IO error: {}", err),
            LoaderError::ParseError(msg, err) => write!(f, "Parse error: {}: {}", msg, err),
            LoaderError::MissingProgramHeaderTable => {
                write!(f, "Parse error: Program header table should be present")
            }
            LoaderError::Missing_End => write!(
                f,
                "Parse error: `{}` symbol should be present",
                Loader::_END_NAME
            ),
            LoaderError::MissingSymbolTable => {
                write!(f, "Parse error: .symtab and .strtab should be present")
            }
        }
    }
}

impl Error for LoaderError {}

impl From<io::Error> for LoaderError {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}
