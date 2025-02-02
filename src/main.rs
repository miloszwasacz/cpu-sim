use cpu_sim::instr::raw::RawInstr;
use cpu_sim::pipeline::decode::DecodeStage;
use std::{fs, io};

fn main() -> io::Result<()> {
    let binary = fs::read("test/bubble_sort.bin")?;
    binary
        .chunks(u32::BITS as usize / 8)
        .enumerate()
        .map(|(i, bytes)| RawInstr::new(bytes, i * 8))
        .skip(4)
        .for_each(|instr| {
            let instr = DecodeStage.decode(instr);
            println!("{:#}", instr);
        });
    Ok(())
}
