use cpu_sim::components::cpu::decoder::Decoder;
use cpu_sim::components::memory::{Address, Memory, MemoryAccess};
use cpu_sim::instr::raw::{RawInstr, RawInstrBits};
use cpu_sim::os::Loader;
use elf::endian::LittleEndian;
use elf::ElfBytes;
use std::fs;
use std::path::Path;

fn main() {
    let file = "test/bubble_sort.bin";

    println!("Decode:");
    test_decode(file);

    println!("\nMemory:");
    test_mem(file);
}

fn test_decode<P: AsRef<Path>>(file: P) {
    const BITS_IN_BYTE: usize = 8;
    const U32_BYTES: usize = u32::BITS as usize / BITS_IN_BYTE;
    let binary = fs::read(file).unwrap();
    let elf = ElfBytes::<LittleEndian>::minimal_parse(&binary).unwrap();
    let text = elf.section_header_by_name(".text").unwrap().unwrap();

    let instrs = binary
        .chunks(U32_BYTES)
        .enumerate()
        .map(|(i, bytes)| {
            RawInstr::from_bytes(
                bytes.try_into().unwrap(),
                i as Address * BITS_IN_BYTE as Address,
            )
        })
        .collect::<Vec<_>>();

    instrs
        .into_iter()
        .skip(text.sh_offset as usize / U32_BYTES)
        .take(text.sh_size as usize / U32_BYTES)
        .for_each(|instr| {
            let instr = Decoder.decode(instr);
            println!("{:#}", instr);
        });
}

fn test_mem<P: AsRef<Path>>(file: P) {
    let mut mem = Memory::new();
    let entry = Loader.load(file, &mut mem);

    let i: RawInstrBits = mem.get(entry);
    let i = RawInstr::new(i, entry);
    let i = Decoder.decode(i);
    println!("{:#}", i);
}
