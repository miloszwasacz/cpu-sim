use super::scrolling::{Scrolling, ScrollingModel};
use super::EXCEPTION_COLOR;
use crate::ui::component::StatefulComponent;
use crate::ui::model::{DecodeQueueModel, Decoded};

use itertools::{EitherOrBoth, Itertools};
use ratatui::prelude::*;

pub type DecodeQueue = Scrolling<DecodeQueueModel>;

impl DecodeQueue {
    pub const WIDTH: Constraint = Constraint::Length(35);

    #[allow(unused)]
    pub fn height(model: &<Self as StatefulComponent>::Model) -> u16 {
        const BORDER: u16 = 2;
        BORDER + model.size() as u16 / 4 * 3
    }
}

impl StatefulComponent for DecodeQueue {
    type Model = DecodeQueueModel;

    fn render(&mut self, model: &Self::Model, area: Rect, buf: &mut Buffer) {
        self.render_scrolling("Instruction Queue", model, area, buf, |entries| {
            entries.zip_longest(0..model.size()).map(|item| {
                let (decoded, i) = match item {
                    EitherOrBoth::Both(d, i) => (Some(d), i),
                    EitherOrBoth::Right(i) => (None, i),
                    EitherOrBoth::Left(_) => {
                        unreachable!("there can be no more entries than the capacity")
                    }
                };
                Line::from(vec![
                    format!("{:>2}: ", i).into(),
                    match decoded {
                        Some(Ok(instr)) => format!("{:#}", instr).into(),
                        Some(Err(_)) => Span::from("Exception").fg(EXCEPTION_COLOR),
                        None => Span::default(),
                    },
                ])
            })
        })
    }
}

impl ScrollingModel for DecodeQueueModel {
    type Entry = Decoded;

    fn entries(&self) -> &[Self::Entry] {
        self.entries()
    }

    fn count(&self) -> usize {
        self.size()
    }
}
