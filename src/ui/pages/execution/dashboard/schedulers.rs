use self::focus::Selected;
use self::scheduler::Scheduler;
use crate::ui::focus::HasFocus;
use crate::ui::model::SchedulersModel;
use crate::ui::{block_style, EventHandler, EventResult, Focus, Focusable, StatefulComponent};

use ratatui::crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders};
use std::iter;
use std::num::NonZeroUsize;
use std::ops::Range;

mod focus;
mod scheduler;

pub struct Schedulers {
    schedulers: Box<[Scheduler]>,
    selected: Selected,
    visible: Range<usize>,
    focused: bool,
}

impl Schedulers {
    pub const WIDTH: Constraint = Constraint::Fill(1);

    pub fn new(count: NonZeroUsize) -> Self {
        Self {
            schedulers: vec![Scheduler::default(); count.get()].into_boxed_slice(),
            selected: Selected::new(count),
            visible: Default::default(),
            focused: Default::default(),
        }
    }

    fn focus_new_next(&mut self, next: Selected) {
        self.schedulers[self.selected.get()].unfocus();
        self.selected = next;
        let selected = self.selected.get();
        self.schedulers[selected].focus_next();
        if selected >= self.visible.end {
            self.visible = self.visible.start.saturating_add(1)..self.visible.end.saturating_add(1);
        }
    }

    fn focus_new_prev(&mut self, prev: Selected) {
        self.schedulers[self.selected.get()].unfocus();
        self.selected = prev;
        let selected = self.selected.get();
        self.schedulers[selected].focus_next();
        if selected < self.visible.start {
            self.visible = self.visible.start.saturating_sub(1)..self.visible.end.saturating_sub(1);
        }
    }
}

impl StatefulComponent for Schedulers {
    type Model = SchedulersModel;

    fn render(&mut self, model: &Self::Model, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .title("Schedulers")
            .borders(Borders::ALL)
            .border_style(block_style(self.focused));

        let width = block.inner(area).width;
        let count = (width / Scheduler::MIN_WIDTH) as usize;

        let widths = iter::repeat(Scheduler::WIDTH).take(count);
        let areas = Layout::horizontal(widths).split(block.inner(area));

        let schedulers = &mut self.schedulers;
        if self.visible.len() != count {
            self.visible = self.visible.start..self.visible.start + count;
        }

        let schedulers = model.schedulers().iter().skip(self.visible.start).zip(
            schedulers
                .iter_mut()
                .skip(self.visible.start)
                .zip(areas.iter().copied()),
        );

        block.render(area, buf);
        for (model, (scheduler, area)) in schedulers {
            scheduler.render(model, area, buf);
        }
    }
}

impl EventHandler<()> for Schedulers {
    type EventResult = ();

    fn handle_event(&mut self, event: Event, payload: ()) -> EventResult<Self::EventResult> {
        let count = self.schedulers.len();
        let selected = self.selected.get();
        let result = self.schedulers[selected].handle_event(event.clone(), payload);

        if let EventResult::Ignored = result {
            match event {
                Event::Key(event) if event.kind == KeyEventKind::Press => match event.code {
                    KeyCode::Left if selected > 0 => {
                        self.focus_new_prev(self.selected.prev());
                        return EventResult::Handled(());
                    }
                    KeyCode::Right if selected < count - 1 => {
                        self.focus_new_next(self.selected.next());
                        return EventResult::Handled(());
                    }
                    KeyCode::Home if selected > 0 => {
                        self.focus_new_prev(self.selected.first());
                        return EventResult::Handled(());
                    }
                    KeyCode::End if selected < count - 1 => {
                        self.focus_new_next(self.selected.last());
                        return EventResult::Handled(());
                    }
                    _ => {}
                },
                _ => {}
            }
        }
        result
    }
}

impl Focusable for Schedulers {
    fn unfocus(&mut self) {
        self.schedulers[self.selected.get()].unfocus();
        self.focused = false;
    }

    fn focus_next(&mut self) -> HasFocus {
        let selected = &mut self.schedulers[self.selected.get()];
        self.focused = if self.focused {
            selected.unfocus();
            false
        } else {
            selected.focus_next();
            true
        };
        self.focused
    }

    fn focus_prev(&mut self) -> HasFocus {
        let selected = &mut self.schedulers[self.selected.get()];
        self.focused = if self.focused {
            selected.unfocus();
            false
        } else {
            selected.focus_next();
            true
        };
        self.focused
    }
}
