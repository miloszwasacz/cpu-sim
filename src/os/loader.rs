use crate::components::cpu::Cpu;
use crate::components::memory::{Address, Memory};

use elf::endian::LittleEndian;
use elf::ElfBytes;
use std::fs;
use std::path::Path;

pub struct Loader;

impl Loader {
    /// Loads the program to memory and initializes the CPU's PC.
    pub fn load<P: AsRef<Path>>(&self, bin: P, mem: &mut Memory, cpu: &mut Cpu) {
        let bin = fs::read(bin).expect("file should exist and be readable");

        let elf = ElfBytes::<LittleEndian>::minimal_parse(&bin)
            .expect("binary should be a valid ELF file");
        let segments = elf
            .segments()
            .expect("program header table should be present");

        for segment in segments {
            let offset = segment.p_offset as usize;
            let addr = segment.p_vaddr as Address;
            let size = segment.p_filesz.min(segment.p_memsz) as usize;

            mem[addr..(addr + size as Address)].copy_from_slice(&bin[offset..(offset + size)])
        }

        cpu.set_entrypoint(elf.ehdr.e_entry as Address);
    }
}
