use self::ui::error::SimError;
use self::ui::App;

use cpu_sim::components::cpu::Cpu;
use cpu_sim::components::memory::Memory;
use cpu_sim::os::loader::Loader;
use std::cell::RefCell;
use std::process::ExitCode;

mod ui;

fn main() -> ExitCode {
    const FILE: &str = "test/res/bin/bubble_sort";

    let mem = RefCell::new(Memory::new());
    //TODO Provide stand-ins for stdin, stdout, and stderr to the CPU to not break the TUI
    let mut cpu = Cpu::new(&mem);
    if let Err(err) = unsafe { Loader.load(FILE, &mut mem.borrow_mut(), &mut cpu) } {
        eprintln!("Load error: {}", err);
        return ExitCode::FAILURE;
    }

    match App::new(&mut cpu).run() {
        Ok(exit_code) => {
            println!("Simulation finished (exit code: {})", exit_code);
            ExitCode::SUCCESS
        }
        Err(err) => {
            match err {
                SimError::EarlyExit => {
                    eprintln!("Simulation stopped early");
                }
                SimError::Tui(err) => eprintln!("TUI error: {}", err),
                SimError::Cpu(errs) => {
                    eprintln!("Simulation errors:");
                    for err in errs {
                        eprintln!("  {}", err);
                    }
                }
            }
            ExitCode::FAILURE
        }
    }

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
