use cpu_sim::components::diagnostics::cpu::{RobEntrySnapshot, RobSnapshot};
use cpu_sim::components::diagnostics::fmt_addr;
use ratatui::prelude::*;
use ratatui::style::palette::tailwind::{GREEN, RED};
use ratatui::widgets::{Block, Borders, List, ListItem};

pub struct Rob<'a>(&'a RobSnapshot);

impl<'a> Rob<'a> {
    pub fn new(rob: &'a RobSnapshot) -> Self {
        Self(rob)
    }
}

impl Widget for Rob<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .title("Reorder Buffer")
            .borders(Borders::ALL);

        let entries = self.0.entries.iter().map(|(i, entry)| {
            const NOT_READY_COLOR: Color = RED.c200;
            const READY_COLOR: Color = GREEN.c200;

            let mut line = Line::from(format!("{:2}: ", i));
            match entry {
                RobEntrySnapshot::Empty => {}
                RobEntrySnapshot::NotReady(entry) => {
                    line += Span::from(format!("{}: {:?}", fmt_addr(entry.addr), entry.data))
                        .fg(NOT_READY_COLOR);
                }
                RobEntrySnapshot::Ready(entry) => {
                    line += Span::from(format!("{}: {:?}", fmt_addr(entry.addr), entry.data))
                        .fg(READY_COLOR);
                }
            }

            ListItem::new(line)
        });

        let list = List::new(entries).block(block);
        Widget::render(list, area, buf);
    }
}
