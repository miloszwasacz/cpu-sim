use self::ui::error::SimError;
use self::ui::{App, StdStream};

use cpu_sim::components::cpu::Cpu;
use cpu_sim::components::memory::Memory;
use cpu_sim::os::loader::Loader;
use cpu_sim::os::Os;
use std::cell::RefCell;
use std::process::ExitCode;

mod ui;

fn main() -> ExitCode {
    const FILE: &str = "test/res/bin/bubble_sort";

    let mem = RefCell::new(Memory::new());
    let stdin = StdStream::default();
    let stdout = StdStream::default();
    let stderr = StdStream::default();
    let os = Os::new(stdin, stdout, stderr);
    let mut cpu = Cpu::new(&mem, os);
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
    //         Ok(cpu_sim::components::cpu::CpuRun::Exit(exit_code)) => {
    //             let stdout = cpu.os().stdout();
    //             println!("{}", stdout.inner());
    //
    //             eprintln!("process exited with code: {}", exit_code);
    //             return ExitCode::SUCCESS;
    //         }
    //         Ok(cpu_sim::components::cpu::CpuRun::Break) => {
    //             eprintln!("process break");
    //             continue;
    //         }
    //         Err(errs) => {
    //             for err in errs {
    //                 eprintln!("{}", err);
    //             }
    //             return ExitCode::FAILURE;
    //         }
    //     }
    // }
}
