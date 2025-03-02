use cpu_sim::components::cpu::{Cpu, CpuRun};
use cpu_sim::components::memory::Memory;
use cpu_sim::os::loader::Loader;
use std::cell::RefCell;
use std::process::ExitCode;

fn main() -> ExitCode {
    const FILE: &str = "test/res/bin/bubble_sort";

    let mem = RefCell::new(Memory::new());
    let mut cpu = Cpu::new(&mem);
    if let Err(err) = Loader.load(FILE, &mut mem.borrow_mut(), &mut cpu) {
        eprintln!("{}", err);
        return ExitCode::FAILURE;
    }

    loop {
        match cpu.run() {
            Ok(CpuRun::Exit(exit_code)) => {
                eprintln!("process exited with code: {}", exit_code);
                return ExitCode::SUCCESS;
            }
            Ok(CpuRun::Break) => {
                eprintln!("process break");
                continue;
            }
            Err(err) => {
                eprintln!("{}", err);
                return ExitCode::FAILURE;
            }
        }
    }
}
