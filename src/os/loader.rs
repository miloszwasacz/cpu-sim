//! Functionality for creating a simulated process from an ELF file.

use crate::components::cpu::Cpu;
use crate::components::memory::{Address, MemSize, Memory};

use elf::endian::LittleEndian;
use elf::ElfBytes;
use std::error::Error;
use std::path::Path;
use std::{fmt, fs, io};

const STACK_SIZE: usize = MemSize(1).GiB();

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
    /// [^1]: By _undefined behavior_ we mean the simulation might exhibit unexpected behavior.
    ///       This method is still _safe_ in the [Rust sense](https://doc.rust-lang.org/reference/unsafety.html).
    pub unsafe fn load<P: AsRef<Path>, I, O, E>(
        &self,
        bin: P,
        mem: &mut Memory,
        cpu: &mut Cpu<I, O, E>,
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

        let errno_addr = Self::get_symbol_addr(&elf, Self::ERRNO_PTR_NAME)?;
        let _end_addr =
            Self::get_symbol_addr(&elf, Self::_END_NAME)?.ok_or(LoaderError::Missing_End)?;
        unsafe {
            cpu.init(
                elf.ehdr.e_entry as Address,
                _end_addr + STACK_SIZE as Address,
            )
        };
        cpu.os().set_errno_addr(errno_addr);
        cpu.os().set__end_addr(_end_addr);

        Ok(())
    }

    /// Tries to find a symbol in the `elf` file and extract the address where
    /// it should be placed in the process memory.
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

/// Possible errors when loading a program.
#[derive(Debug)]
pub enum LoaderError {
    /// The loader could not open the file.
    Io(io::Error),
    /// The loader could not parse the ELF file.
    ParseError(&'static str, elf::ParseError),
    /// The Program Header Table was not present in the ELF file.
    MissingProgramHeaderTable,
    /// The `_end` symbol was not present in the ELF file.
    #[allow(non_camel_case_types)]
    Missing_End,
    /// `.symtab` or `.strtab` were not present in the ELF file.
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
