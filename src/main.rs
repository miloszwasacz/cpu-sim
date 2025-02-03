use cpu_sim::instr::raw::RawInstr;
use cpu_sim::pipeline::decode::DecodeStage;
use elf::endian::LittleEndian;
use elf::ElfBytes;
use std::{fs, io};

const BITS_IN_BYTE: usize = 8;
const U32_BYTES: usize = u32::BITS as usize / BITS_IN_BYTE;

fn main() -> io::Result<()> {
    let binary = fs::read("test/bubble_sort.bin")?;
    let elf = ElfBytes::<LittleEndian>::minimal_parse(&binary).unwrap();
    let text = elf.section_header_by_name(".text").unwrap().unwrap();

    let instrs = binary
        .chunks(U32_BYTES)
        .enumerate()
        .map(|(i, bytes)| RawInstr::new(bytes, i * BITS_IN_BYTE))
        .collect::<Vec<_>>();

    instrs
        .into_iter()
        .skip(text.sh_offset as usize / U32_BYTES)
        .take(text.sh_size as usize / U32_BYTES)
        .for_each(|instr| {
            let instr = DecodeStage.decode(instr);
            println!("{:#}", instr);
        });

    Ok(())
}
