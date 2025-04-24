use self::decode_queue::DecodeQueue;
use self::front_end::FrontEnd;
use self::load_queue::LoadQueue;
use self::regs::future_file::FutureFile;
use self::regs::reg_file::RegFile;
use self::rob::Rob;
use self::schedulers::Schedulers;
use super::{EventPayload, HelpItem};
use crate::ui::model::CpuModel;
use crate::ui::{
    Component, EventHandler, EventResult, Focus, FocusHandler, Focusable, StatefulComponent,
};

use cpu_sim::components::cpu::{CpuRun, ExitCode};
use cpu_sim::components::diagnostics::Diagnostics;
use cpu_sim_derive::Focus;
use ratatui::buffer::Buffer;
use ratatui::crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui::layout::Rect;
use ratatui::prelude::*;
use std::num::NonZeroUsize;

mod decode_queue;
mod front_end;
mod load_queue;
mod regs;
mod rob;
mod schedulers;
mod scrolling;

const READY_COLOR: Color = Color::Green;
const NOT_READY_COLOR: Color = Color::Red;
const EXCEPTION_COLOR: Color = Color::Yellow;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Focus)]
enum Focused {
    #[none]
    None,
    FrontEnd,
    DecodeQueue,
    Schedulers,
    LoadQueue,
    Rob,
    RegFile,
    FutureFile,
}

pub struct CpuDashboard {
    front_end: FrontEnd,
    decode_queue: DecodeQueue,
    schedulers: Schedulers,
    reg_file: RegFile,
    future_file: FutureFile,
    rob: Rob,
    load_queue: LoadQueue,
    focused: Focused,
    exit_code: Option<ExitCode>,
}

impl CpuDashboard {
    pub fn new(scheduler_count: NonZeroUsize) -> Self {
        Self {
            front_end: Default::default(),
            decode_queue: Default::default(),
            schedulers: Schedulers::new(scheduler_count),
            reg_file: Default::default(),
            future_file: Default::default(),
            rob: Default::default(),
            load_queue: Default::default(),
            focused: Default::default(),
            exit_code: None,
        }
    }

    pub(super) fn help(&self) -> impl IntoIterator<Item = HelpItem> {
        const SCROLL_V: (&str, &str) = ("↑↓", "Scroll");
        const SCROLL_H: (&str, &str) = ("←→", "Scroll");
        const JUMP: (&str, &str) = ("Home/End", "Scroll to start/end");
        let mut tips = match self.exit_code {
            None => vec![("F5", "Run"), ("Enter", "Step")],
            Some(_) => vec![("Q", "Quit")],
        };
        match self.focused {
            Focused::None => {}
            Focused::FrontEnd => {}
            Focused::DecodeQueue => {
                tips.push(SCROLL_V);
                tips.push(JUMP);
            }
            Focused::Schedulers => {
                tips.push(SCROLL_V);
                tips.push(JUMP);
                tips.push(("←→", "Switch scheduler"));
            }
            Focused::LoadQueue => {
                tips.push(SCROLL_V);
                tips.push(JUMP);
            }
            Focused::Rob => {
                tips.push(SCROLL_V);
                tips.push(JUMP);
            }
            Focused::RegFile => {
                tips.push(SCROLL_H);
                tips.push(JUMP);
            }
            Focused::FutureFile => {
                tips.push(SCROLL_H);
                tips.push(JUMP);
            }
        }
        tips
    }
}

impl StatefulComponent for CpuDashboard {
    type Model = CpuModel;

    fn render(&mut self, model: &Self::Model, area: Rect, buf: &mut Buffer) {
        let [top, reg_file, future_file] =
            Layout::vertical([Constraint::Fill(1), RegFile::HEIGHT, FutureFile::HEIGHT])
                .areas(area);
        let [left, rob] = Layout::horizontal([Constraint::Fill(1), Rob::WIDTH]).areas(top);
        let [top, middle] = Layout::vertical([
            Constraint::Length(FrontEnd::height(model.front_end())),
            Constraint::Fill(1),
        ])
        .areas(left);
        let [front_end, decode_queue] =
            Layout::horizontal([FrontEnd::WIDTH, DecodeQueue::WIDTH]).areas(top);
        let [schedulers, load_queue] =
            Layout::horizontal([Schedulers::WIDTH, LoadQueue::WIDTH]).areas(middle);

        self.front_end.render(model.front_end(), front_end, buf);
        self.decode_queue
            .render(model.decode_queue(), decode_queue, buf);
        self.schedulers.render(model.schedulers(), schedulers, buf);
        self.reg_file.render(model.reg_file(), reg_file, buf);
        self.future_file
            .render(model.future_file(), future_file, buf);
        self.rob.render(model.rob(), rob, buf);
        self.load_queue.render(model.load_queue(), load_queue, buf);
    }
}

impl EventHandler<EventPayload<'_, '_>> for CpuDashboard {
    type EventResult = Option<ExitCode>;

    fn handle_event(
        &mut self,
        event: Event,
        payload: EventPayload,
    ) -> EventResult<Self::EventResult> {
        let result = match self.focused {
            Focused::None => Default::default(),
            Focused::FrontEnd => self.front_end.handle_event(event.clone(), ()),
            Focused::DecodeQueue => self.decode_queue.handle_event(event.clone(), ()),
            Focused::Schedulers => self.schedulers.handle_event(event.clone(), ()),
            Focused::RegFile => self.reg_file.handle_event(event.clone(), ()),
            Focused::FutureFile => self.future_file.handle_event(event.clone(), ()),
            Focused::Rob => self.rob.handle_event(event.clone(), ()),
            Focused::LoadQueue => self.load_queue.handle_event(event.clone(), ()),
        };
        match result {
            EventResult::Handled(_) => EventResult::Handled(None),
            EventResult::Err(err) => EventResult::Err(err),
            EventResult::Ignored => match event {
                Event::Key(event) if event.kind == KeyEventKind::Press => {
                    let result = match event.code {
                        KeyCode::Enter if self.exit_code.is_none() => payload.cpu.step(),
                        KeyCode::F(5) if self.exit_code.is_none() => payload.cpu.run(),
                        KeyCode::Char('q') if self.exit_code.is_some() => {
                            return EventResult::Handled(self.exit_code);
                        }
                        _ => return Default::default(),
                    };
                    *payload.model = payload.cpu.diagnostics().into();
                    match result {
                        Ok(CpuRun::Exit(code)) => {
                            self.exit_code = Some(code);
                            EventResult::Handled(None)
                        }
                        Ok(CpuRun::Break) => EventResult::Handled(None),
                        Err(err) => EventResult::Err(err.into()),
                    }
                }
                _ => Default::default(),
            },
        }
    }
}

impl FocusHandler for CpuDashboard {
    fn focused(&mut self) -> Option<&mut dyn Focusable> {
        match self.focused {
            Focused::None => None,
            Focused::FrontEnd => Some(&mut self.front_end as &mut dyn Focusable),
            Focused::DecodeQueue => Some(&mut self.decode_queue as &mut dyn Focusable),
            Focused::Schedulers => Some(&mut self.schedulers as &mut dyn Focusable),
            Focused::RegFile => Some(&mut self.reg_file as &mut dyn Focusable),
            Focused::FutureFile => Some(&mut self.future_file as &mut dyn Focusable),
            Focused::Rob => Some(&mut self.rob as &mut dyn Focusable),
            Focused::LoadQueue => Some(&mut self.load_queue as &mut dyn Focusable),
        }
    }

    fn focus_next(&mut self) {
        self.focused = self.focused.next();
    }

    fn focus_prev(&mut self) {
        self.focused = self.focused.prev();
    }
}
