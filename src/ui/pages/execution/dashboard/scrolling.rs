use crate::ui::block_style;
use crate::ui::events::{EventHandler, EventResult};
use crate::ui::focus::{Focusable, HasFocus};

use ratatui::crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui::prelude::*;
use ratatui::widgets::block::Title;
use ratatui::widgets::{Block, Borders, List, ListItem};
use std::marker::PhantomData;
use std::slice::Iter;

pub(super) struct Scrolling<M, const ITEM_HEIGHT: u16 = 1> {
    head: usize,
    focused: bool,
    model: PhantomData<M>,
}

impl<M, const H: u16> Default for Scrolling<M, H> {
    fn default() -> Self {
        Self {
            head: Default::default(),
            focused: Default::default(),
            model: PhantomData,
        }
    }
}

impl<M, const H: u16> Clone for Scrolling<M, H> {
    fn clone(&self) -> Self {
        Self {
            head: self.head,
            focused: self.focused,
            model: self.model,
        }
    }
}

impl<E, M: ScrollingModel<Entry = E>, const H: u16> Scrolling<M, H> {
    pub(super) fn render_scrolling<'m, 't, 'l, T, I>(
        &mut self,
        title: T,
        model: &'m M,
        area: Rect,
        buf: &mut Buffer,
        entries_to_items: impl FnOnce(Iter<'m, E>) -> I,
    ) where
        T: Into<Title<'t>>,
        I: Iterator<Item: Into<ListItem<'l>>>,
        E: 'm,
    {
        let block = Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_style(block_style(self.focused));

        let height = block.inner(area).height;
        let count = (height / H) as usize;

        let entries = model.entries();
        if self.head.saturating_add(count) > entries.len() {
            self.head = entries.len().saturating_sub(count);
        }

        let items = entries_to_items(entries.iter()).skip(self.head);
        let list = List::new(items).block(block);
        Widget::render(list, area, buf);
    }
}

impl<M, const H: u16> EventHandler<()> for Scrolling<M, H> {
    type EventResult = ();

    fn handle_event(&mut self, event: Event, _: ()) -> EventResult<Self::EventResult> {
        match event {
            Event::Key(event) if event.kind == KeyEventKind::Press => match event.code {
                KeyCode::Up => {
                    self.head = self.head.saturating_sub(1);
                    EventResult::Handled(())
                }
                KeyCode::Down => {
                    self.head = self.head.saturating_add(1);
                    EventResult::Handled(())
                }
                KeyCode::Home => {
                    self.head = 0;
                    EventResult::Handled(())
                }
                KeyCode::End => {
                    self.head = usize::MAX;
                    EventResult::Handled(())
                }
                _ => EventResult::Ignored,
            },
            _ => EventResult::Ignored,
        }
    }
}

impl<M, const H: u16> Focusable for Scrolling<M, H> {
    fn unfocus(&mut self) {
        self.focused = false;
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

pub(super) trait ScrollingModel {
    type Entry;
    fn entries(&self) -> &[Self::Entry];
}
