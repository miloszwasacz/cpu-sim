use self::ui::error::SimError;
use self::ui::App;

use cpu_sim::components::cpu::Cpu;
use cpu_sim::components::memory::Memory;
use cpu_sim::os::loader::Loader;
use std::cell::RefCell;
use std::process::ExitCode;

mod ui;

fn main() -> Result<ExitCode, SimError> {
    const FILE: &str = "test/res/bin/bubble_sort";

    let mem = RefCell::new(Memory::new());
    //TODO Provide stand-ins for stdin, stdout, and stderr to the CPU to not break the TUI
    let mut cpu = Cpu::new(&mem);
    if let Err(err) = unsafe { Loader.load(FILE, &mut mem.borrow_mut(), &mut cpu) } {
        panic!("{}", err);
        // return ExitCode::FAILURE;
    }

    let exit_code = App::new(&mut cpu).run()?;
    Ok(ExitCode::from(exit_code as u8))

    // loop {
    //     match cpu.run() {
    //         Ok(CpuRun::Exit(exit_code)) => {
    //             eprintln!("process exited with code: {}", exit_code);
    //             return Ok(());
    //             // return ExitCode::SUCCESS;
    //         }
    //         Ok(CpuRun::Break) => {
    //             eprintln!("process break");
    //             continue;
    //         }
    //         Err(errs) => {
    //             return Err(errs.into());
    //             // for err in errs {
    //             //     eprintln!("{}", err);
    //             // }
    //             // return ExitCode::FAILURE;
    //         }
    //     }
    // }
}
