use super::scrolling::{Scrolling, ScrollingModel};
use super::{NOT_READY_COLOR, READY_COLOR};
use crate::ui::model::LoadQueueModel;
use crate::ui::StatefulComponent;

use cpu_sim::components::diagnostics::cpu::LoadQueueEntry;
use cpu_sim::components::diagnostics::fmt_addr;
use ratatui::prelude::*;

pub type LoadQueue = Scrolling<LoadQueueModel>;

impl LoadQueue {
    pub const WIDTH: Constraint = Constraint::Length(21);
}

impl StatefulComponent for LoadQueue {
    type Model = LoadQueueModel;

    fn render(&mut self, model: &Self::Model, area: Rect, buf: &mut Buffer) {
        self.render_scrolling("Load queue", model, area, buf, |entries| {
            entries.enumerate().map(|(i, entry)| {
                let dest = format!("#{}", entry.dest);
                let addr = fmt_addr(entry.addr);
                Line::from(vec![
                    format!("{:>2}: ", i).into(),
                    Span::from(format!("{:>3}, {}", dest, addr)).fg(if entry.ready {
                        READY_COLOR
                    } else {
                        NOT_READY_COLOR
                    }),
                ])
            })
        })
    }
}

impl ScrollingModel for LoadQueueModel {
    type Entry = LoadQueueEntry;

    fn entries(&self) -> &[Self::Entry] {
        self.entries()
    }
}
