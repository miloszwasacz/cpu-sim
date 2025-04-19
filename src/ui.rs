use self::component::{Component, StatefulComponent};
use self::error::SimError;
use self::events::{EventHandler, EventResult};
use self::focus::*;
use self::pages::ExecutionPage;

use cpu_sim::components::cpu::{Cpu, ExitCode};
use ratatui::crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::crossterm::{event, execute};
use ratatui::prelude::*;
use std::io;
use std::io::stdout;
use std::panic::{set_hook, take_hook};

mod component;
pub mod error;
mod events;
mod focus;
mod model;
mod pages;

#[derive(Default)]
enum Page<'c, 'm> {
    #[default]
    None,
    ExecPage(ExecutionPage<'c, 'm>),
}

pub struct App<'c, 'm> {
    cpu: Option<&'c mut Cpu<'m>>,
    page: Page<'c, 'm>,
}

impl<'c, 'm> App<'c, 'm> {
    pub fn new(cpu: &'c mut Cpu<'m>) -> Self {
        Self {
            cpu: Some(cpu),
            page: Default::default(),
        }
    }

    pub fn run(mut self) -> Result<ExitCode, SimError> {
        Self::init_panic_hook();
        let mut tui = Self::init_tui()?;
        let result = self.event_loop(&mut tui);
        let tui_result = Self::restore_tui().map_err(Into::into);
        result.and_then(|code| tui_result.map(|_| code))
    }

    fn init_panic_hook() {
        let original_hook = take_hook();
        set_hook(Box::new(move |panic_info| {
            let _ = Self::restore_tui();
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

    fn event_loop(&mut self, tui: &mut Terminal<impl Backend>) -> Result<ExitCode, SimError> {
        //TODO Program loading
        self.page = Page::ExecPage(ExecutionPage::new(self.cpu.take().unwrap()));

        self.render(tui)?;
        loop {
            let event = event::read()?;
            match &mut self.page {
                Page::None => panic!("initialization failed"),
                Page::ExecPage(exec_page) => {
                    let result = match exec_page.handle_event(event.clone(), ()) {
                        EventResult::Ignored => match event {
                            Event::Resize(_, _) => Ok(None),
                            //TODO Constant for early exit
                            Event::Key(event) if is_ctrl_c(event) => Ok(Some(1)),
                            Event::Key(event) if event.kind == KeyEventKind::Press => {
                                match event.code {
                                    KeyCode::Tab => {
                                        Focusable::focus_next(exec_page);
                                        Ok(None)
                                    }
                                    KeyCode::BackTab => {
                                        Focusable::focus_prev(exec_page);
                                        Ok(None)
                                    }
                                    _ => continue,
                                }
                            }
                            _ => continue,
                        },
                        EventResult::Handled(code) => Ok(code),
                        EventResult::Err(err) => Err(err),
                    };
                    let render_result = self.render(tui).map_err(Into::into);
                    let code = result.and_then(|code| render_result.map(|_| code))?;
                    if let Some(code) = code {
                        //TODO Handle finished program better
                        std::thread::sleep(std::time::Duration::from_secs(2));
                        return Ok(code);
                    }
                }
            }
        }
    }

    fn render(&mut self, tui: &mut Terminal<impl Backend>) -> io::Result<()> {
        tui.draw(|frame| {
            let area = frame.area();
            match &mut self.page {
                Page::None => panic!("initialization failed"),
                Page::ExecPage(exec_page) => frame.render_widget(exec_page, area),
            }
        })
        .map(|_| ())
    }
}

fn block_style(focused: bool) -> Style {
    if focused {
        Style::default().fg(Color::LightBlue)
    } else {
        Style::default()
    }
}

fn is_ctrl_c(event: KeyEvent) -> bool {
    event.kind == KeyEventKind::Press
        && event.code == KeyCode::Char('c')
        && event.modifiers.contains(KeyModifiers::CONTROL)
        && event.modifiers.bits().count_ones() == 1
}
