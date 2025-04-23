use crate::ui::component::StatefulComponent;
use crate::ui::events::{EventHandler, EventResult};
use crate::ui::focus::{Focusable, HasFocus};
use crate::ui::{block_style, StdStream};
use ratatui::crossterm::event::{Event, KeyCode, KeyEventKind};

use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders};

pub struct OutStream {
    title: &'static str,
    head: usize,
    focused: bool,
}

impl OutStream {
    pub fn new(title: &'static str) -> Self {
        Self {
            title,
            head: Default::default(),
            focused: Default::default(),
        }
    }
}

impl StatefulComponent for OutStream {
    type Model = StdStream;

    fn render(&mut self, model: &Self::Model, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .title(self.title)
            .borders(Borders::ALL)
            .border_style(block_style(self.focused));
        let stream_area = block.inner(area);
        block.render(area, buf);

        let height = stream_area.height as usize;

        let lines_rev = model
            .inner()
            .split_inclusive('\n')
            .rev()
            .map(Line::from)
            .collect::<Vec<_>>();

        if lines_rev.len().saturating_sub(self.head) < height {
            self.head = self.head.saturating_sub(1);
        }

        let line_areas = Layout::vertical(vec![Constraint::Length(1); height]).split(stream_area);
        for (line, area) in lines_rev
            .iter()
            .skip(self.head)
            .take(height)
            .zip(line_areas.iter().rev().copied())
        {
            line.render(area, buf);
        }
    }
}

impl EventHandler<()> for OutStream {
    type EventResult = ();

    fn handle_event(&mut self, event: Event, _: ()) -> EventResult<Self::EventResult> {
        match event {
            Event::Key(event) if event.kind == KeyEventKind::Press => match event.code {
                KeyCode::Up => {
                    self.head = self.head.saturating_add(1);
                    EventResult::Handled(())
                }
                KeyCode::Down => {
                    self.head = self.head.saturating_sub(1);
                    EventResult::Handled(())
                }
                KeyCode::Home => {
                    self.head = 0;
                    EventResult::Handled(())
                }
                _ => Default::default(),
            },
            _ => Default::default(),
        }
    }
}

impl Focusable for OutStream {
    fn unfocus(&mut self) {
        self.focused = true;
    }

    fn focus_next(&mut self) -> HasFocus {
        self.focused = !self.focused;
        self.focused
    }

    fn focus_prev(&mut self) -> HasFocus {
        self.focused = !self.focused;
        self.focused
    }
}
