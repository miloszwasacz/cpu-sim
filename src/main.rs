use cpu_sim::components::cpu::Cpu;
use cpu_sim::components::memory::Memory;
use cpu_sim::os::loader::Loader;
use std::cell::RefCell;

fn main() {
    const FILE: &str = "test/bubble_sort.bin";

    let mem = RefCell::new(Memory::new());
    let mut cpu = Cpu::new(&mem);
    if let Err(err) = Loader.load(FILE, &mut mem.borrow_mut(), &mut cpu) {
        eprintln!("{}", err);
        std::process::exit(2);
    }

    if let Err(err) = cpu.run() {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}
