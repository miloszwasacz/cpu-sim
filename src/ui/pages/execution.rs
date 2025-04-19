use self::dashboard::CpuDashboard;
use super::Page;
use crate::ui::model::CpuModel;
use crate::ui::{EventHandler, EventResult, FocusHandler, Focusable, StatefulComponent};

use cpu_sim::components::cpu::{Cpu, ExitCode};
use cpu_sim::components::diagnostics::Diagnostics;
use cpu_sim_derive::Focus;
use ratatui::crossterm::event::Event;
use ratatui::prelude::*;
use std::num::NonZeroUsize;

mod dashboard;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Focus)]
enum Focused {
    #[none]
    Dashboard,
    //TODO Console & Tabs
}

pub struct ExecutionPage<'c, 'm> {
    cpu: &'c mut Cpu<'m>,
    model: CpuModel,
    dashboard: CpuDashboard,
    focused: Focused,
}

impl<'c, 'm> ExecutionPage<'c, 'm> {
    pub fn new(cpu: &'c mut Cpu<'m>) -> Self {
        let model = cpu.diagnostics().into();
        let scheduler_count =
            NonZeroUsize::new(cpu.scheduler_count()).expect("the should be at least one scheduler");

        Self {
            cpu,
            model,
            dashboard: CpuDashboard::new(scheduler_count),
            focused: Default::default(),
        }
    }
}

impl<'c, 'm> Page<&'c mut Cpu<'m>> for ExecutionPage<'c, 'm> {
    fn close(self) -> &'c mut Cpu<'m> {
        self.cpu
    }
}

impl Widget for &mut ExecutionPage<'_, '_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self.focused {
            Focused::Dashboard => self.dashboard.render(&self.model, area, buf),
        }
    }
}

impl EventHandler<()> for ExecutionPage<'_, '_> {
    type EventResult = Option<ExitCode>;

    fn handle_event(&mut self, event: Event, _: ()) -> EventResult<Self::EventResult> {
        match self.focused {
            Focused::Dashboard => {
                let payload = EventPayload {
                    cpu: self.cpu,
                    model: &mut self.model,
                };
                self.dashboard.handle_event(event, payload)
            }
        }
    }
}

pub struct EventPayload<'c, 'm, 'cm> {
    cpu: &'c mut Cpu<'m>,
    model: &'cm mut CpuModel,
}

impl FocusHandler for ExecutionPage<'_, '_> {
    fn focused(&mut self) -> Option<&mut dyn Focusable> {
        Some(&mut self.dashboard)
    }

    fn focus_next(&mut self) {}

    fn focus_prev(&mut self) {}
}
