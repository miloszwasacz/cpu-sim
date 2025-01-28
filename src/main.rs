use std::{fs, io};
use cpu_sim::instr::RawInstr;
use cpu_sim::pipeline::decode::DecodeStage;

fn main() -> io::Result<()> {
    let binary = fs::read("test/bubble_sort.bin")?;
    binary.chunks(u32::BITS as usize / 8).map(RawInstr::new).for_each(|instr| {
        let instr = DecodeStage.decode(instr);
        println!("{:#}", instr);
    });
    Ok(())
}
