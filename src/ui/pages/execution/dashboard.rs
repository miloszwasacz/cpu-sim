use self::front_end::FrontEnd;
use self::load_queue::LoadQueue;
use self::regs::reg_file::RegFile;
use self::regs::reg_stat::RegStat;
use self::rob::Rob;
use self::schedulers::Schedulers;
use super::EventPayload;
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

mod front_end;
mod load_queue;
mod regs;
mod rob;
mod schedulers;
mod scrolling;

const READY_COLOR: Color = Color::Green;
const NOT_READY_COLOR: Color = Color::Red;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Focus)]
enum Focused {
    #[none]
    None,
    FrontEnd,
    Schedulers,
    LoadQueue,
    Rob,
    RegFile,
    RegStat,
}

pub struct CpuDashboard {
    front_end: FrontEnd,
    schedulers: Schedulers,
    reg_file: RegFile,
    reg_stat: RegStat,
    rob: Rob,
    load_queue: LoadQueue,
    focused: Focused,
}

impl CpuDashboard {
    pub fn new(scheduler_count: NonZeroUsize) -> Self {
        Self {
            front_end: Default::default(),
            schedulers: Schedulers::new(scheduler_count),
            reg_file: Default::default(),
            reg_stat: Default::default(),
            rob: Default::default(),
            load_queue: Default::default(),
            focused: Default::default(),
        }
    }
}

impl StatefulComponent for CpuDashboard {
    type Model = CpuModel;

    fn render(&mut self, model: &Self::Model, area: Rect, buf: &mut Buffer) {
        let [top, reg_file, reg_stat] =
            Layout::vertical([Constraint::Fill(1), RegFile::HEIGHT, RegStat::HEIGHT]).areas(area);
        let [left, rob] = Layout::horizontal([Constraint::Fill(1), Rob::WIDTH]).areas(top);
        let [front_end, middle] =
            Layout::vertical([FrontEnd::HEIGHT, Constraint::Fill(1)]).areas(left);
        let [schedulers, load_queue] =
            Layout::horizontal([Schedulers::WIDTH, LoadQueue::WIDTH]).areas(middle);

        self.front_end.render(model.front_end(), front_end, buf);
        self.schedulers.render(model.schedulers(), schedulers, buf);
        self.reg_file.render(model.reg_file(), reg_file, buf);
        self.reg_stat.render(model.reg_stat(), reg_stat, buf);
        self.rob.render(model.rob(), rob, buf);
        self.load_queue.render(model.load_queue(), load_queue, buf);
    }
}

impl EventHandler<EventPayload<'_, '_, '_>> for CpuDashboard {
    type EventResult = Option<ExitCode>;

    fn handle_event(
        &mut self,
        event: Event,
        payload: EventPayload,
    ) -> EventResult<Self::EventResult> {
        let result = match self.focused {
            Focused::None => Default::default(),
            Focused::FrontEnd => self.front_end.handle_event(event.clone(), ()),
            Focused::Schedulers => self.schedulers.handle_event(event.clone(), ()),
            Focused::RegFile => self.reg_file.handle_event(event.clone(), ()),
            Focused::RegStat => self.reg_stat.handle_event(event.clone(), ()),
            Focused::Rob => self.rob.handle_event(event.clone(), ()),
            Focused::LoadQueue => self.load_queue.handle_event(event.clone(), ()),
        };
        match result {
            EventResult::Handled(_) => EventResult::Handled(None),
            EventResult::Err(err) => EventResult::Err(err),
            EventResult::Ignored => match event {
                Event::Key(event) if event.kind == KeyEventKind::Press => {
                    let result = match event.code {
                        KeyCode::Enter => payload.cpu.step(),
                        KeyCode::F(5) => payload.cpu.run(),
                        _ => return EventResult::Ignored,
                    };
                    *payload.model = payload.cpu.diagnostics().into();
                    match result {
                        Ok(CpuRun::Exit(code)) => EventResult::Handled(Some(code)),
                        Ok(CpuRun::Break) => EventResult::Handled(None),
                        Err(err) => EventResult::Err(err.into()),
                    }
                }
                _ => EventResult::Ignored,
            },
        }
    }
}

impl FocusHandler for CpuDashboard {
    fn focused(&mut self) -> Option<&mut dyn Focusable> {
        match self.focused {
            Focused::None => None,
            Focused::FrontEnd => Some(&mut self.front_end as &mut dyn Focusable),
            Focused::Schedulers => Some(&mut self.schedulers as &mut dyn Focusable),
            Focused::RegFile => Some(&mut self.reg_file as &mut dyn Focusable),
            Focused::RegStat => Some(&mut self.reg_stat as &mut dyn Focusable),
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
