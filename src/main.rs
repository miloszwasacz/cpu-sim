use self::ui::error::SimError;
use self::ui::{App, StdStream};

use cpu_sim::components::cpu::Cpu;
use cpu_sim::components::memory::Memory;
use cpu_sim::os::loader::Loader;
use cpu_sim::os::Os;
use std::env;
use std::process::ExitCode;
use std::sync::{Arc, Mutex};

mod ui;

fn main() -> ExitCode {
    // Read arguments
    let args = env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        println!("Usage: cpu-sim <path-to-binary>");
        return ExitCode::SUCCESS;
    }
    let file = &args[1];

    // Set up simulation
    let mem = Arc::new(Mutex::new(Memory::new()));
    let stdin = StdStream::default();
    let stdout = StdStream::default();
    let stderr = StdStream::default();
    let os = Os::new(stdin, stdout, stderr);
    let mut cpu = Cpu::new(mem.clone(), os);
    if let Err(err) = unsafe { Loader.load(file, &mut mem.lock().unwrap(), &mut cpu) } {
        eprintln!("Load error: {}", err);
        return ExitCode::FAILURE;
    }

    // Run simulation
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
}
