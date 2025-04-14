use cpu_sim::components::cpu::{Cpu, CpuRun};
use cpu_sim::components::memory::Memory;
use cpu_sim::os::loader::Loader;

use ratatui::crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::crossterm::{event, execute};
use ratatui::prelude::*;
use ratatui::Terminal;

use cpu_sim::components::diagnostics::Diagnostics;
use ratatui::crossterm::event::{Event, KeyCode, KeyEventKind};
use std::cell::RefCell;
use std::io;
use std::io::stdout;
use std::panic::{set_hook, take_hook};

mod ui;

fn main() -> Result<(), SimErr> {
    const FILE: &str = "test/res/bin/bubble_sort";

    let mem = RefCell::new(Memory::new());
    let mut cpu = Cpu::new(&mem);
    if let Err(err) = unsafe { Loader.load(FILE, &mut mem.borrow_mut(), &mut cpu) } {
        panic!("{}", err);
        // return ExitCode::FAILURE;
    }

    run_tui(&mut cpu)

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

fn run_tui(cpu: &mut Cpu) -> Result<(), SimErr> {
    init_panic_hook();
    let mut tui = init_tui()?;
    let update = |tui: &mut Terminal<_>, cpu: &Cpu| {
        let snapshot = cpu.diagnostics();
        let app = ui::App(snapshot);
        tui.draw(|frame| frame.render_widget(app, frame.area()))
            .map(|_| ())
    };

    update(&mut tui, cpu)?;
    let result = loop {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                match key_event.code {
                    KeyCode::Enter => {
                        let r = match cpu.step() {
                            Ok(r) => r,
                            // Err(errs) => break Err(errs.into()),
                            Err(errs) => {
                                panic!("PC: {:05x}\n{:?}", cpu.diagnostics().pc, errs)
                            }
                        };
                        if let Err(err) = update(&mut tui, cpu) {
                            break Err(err.into());
                        }
                        match r {
                            CpuRun::Break => {
                                if cpu.diagnostics().pc == 0x119c4 {
                                    panic!("Program end");
                                }
                                continue;
                            }
                            //TODO Exit codes
                            CpuRun::Exit(_) => break Ok(()),
                        }
                    }
                    KeyCode::Char('q') => break Ok(()),
                    _ => {}
                }
            }
            _ => {}
        }
    };

    let tui_result = restore_tui().map_err(Into::into);
    result.and(tui_result)
}

fn init_panic_hook() {
    let original_hook = take_hook();
    set_hook(Box::new(move |panic_info| {
        let _ = restore_tui();
        original_hook(panic_info);
    }))
}

fn init_tui() -> io::Result<Terminal<impl Backend>> {
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;
    Terminal::new(CrosstermBackend::new(stdout()))
}

fn restore_tui() -> io::Result<()> {
    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen)
}

#[derive(Debug)]
enum SimErr {
    Io(io::Error),
    Sim(Vec<Box<dyn std::error::Error>>),
}

impl std::fmt::Display for SimErr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SimErr::Io(err) => std::fmt::Display::fmt(err, f),
            SimErr::Sim(errs) => {
                for err in errs {
                    std::fmt::Display::fmt(err, f)?;
                    std::fmt::Display::fmt("\n", f)?;
                }
                Ok(())
            }
        }
    }
}

impl std::error::Error for SimErr {}

impl From<io::Error> for SimErr {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<Vec<Box<dyn std::error::Error>>> for SimErr {
    fn from(err: Vec<Box<dyn std::error::Error>>) -> Self {
        Self::Sim(err)
    }
}
