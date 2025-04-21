use crate::ui::block_style;
use crate::ui::events::{EventHandler, EventResult};
use crate::ui::focus::{Focusable, HasFocus};
use crate::ui::model::{FutureFileModel, RegFileModel};

use cpu_sim::components::diagnostics::cpu::{RegData, RegName, RegStatSnapshot};
use ratatui::crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui::prelude::*;
use ratatui::widgets::block::Title;
use ratatui::widgets::{Block, Borders, Row, Table};
use std::iter;
use std::marker::PhantomData;

pub mod future_file;
pub mod reg_file;

pub struct Regs<M, const HEIGHT: u16> {
    head: usize,
    focused: bool,
    model: PhantomData<M>,
}

impl<M, const H: u16> Regs<M, H> {
    pub const HEIGHT: Constraint = Constraint::Length(H);
}

#[allow(private_bounds)]
impl<M: RegModel, const H: u16> Regs<M, H> {
    fn render_registers<'a, 'm, 'r, T, R, F>(
        &mut self,
        title: T,
        model: &'m M,
        area: Rect,
        buf: &mut Buffer,
        rows: R,
    ) where
        T: Into<Title<'a>>,
        R: IntoIterator<Item = F>,
        F: FnOnce(&mut dyn Iterator<Item = &'m M::Data>) -> Row<'r>,
    {
        const ITEM_WIDTH: u16 = 6;
        const ITEM_SPACING: u16 = 2;
        const ITEM_WITH_SPACING: u16 = ITEM_WIDTH + ITEM_SPACING;

        let block = Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_style(block_style(self.focused));

        let width = block.inner(area).width;
        let count = (width + ITEM_SPACING) / ITEM_WITH_SPACING;
        let last_width = width.saturating_sub(count * ITEM_WITH_SPACING);
        let count = count as usize;

        let regs = model.regs();
        if self.head.saturating_add(count) > regs.len() {
            self.head = regs.len().saturating_sub(count);
        }
        let regs = &regs[self.head..self.head + count];

        let names = Row::new(regs.iter().map(|(name, _)| format!("{}:", name))).bold();
        let make_iter = || regs.iter().map(|(_, data)| data);
        let rows = rows.into_iter().map(|f| f(&mut make_iter()));
        let widths = iter::repeat(Constraint::Length(ITEM_WIDTH))
            .take(count)
            .chain([Constraint::Length(last_width)]);

        let table = Table::new(iter::once(names).chain(rows), widths)
            .column_spacing(ITEM_SPACING)
            .block(block);

        Widget::render(table, area, buf);
    }
}

impl<M: RegModel, const H: u16> Default for Regs<M, H> {
    fn default() -> Self {
        Self {
            head: Default::default(),
            focused: Default::default(),
            model: PhantomData,
        }
    }
}

impl<M, const H: u16> EventHandler<()> for Regs<M, H> {
    type EventResult = ();

    fn handle_event(&mut self, event: Event, _: ()) -> EventResult<Self::EventResult> {
        match event {
            Event::Key(event) if event.kind == KeyEventKind::Press => match event.code {
                KeyCode::Left => {
                    self.head = self.head.saturating_sub(1);
                    return EventResult::Handled(());
                }
                KeyCode::Right => {
                    self.head = self.head.saturating_add(1);
                    return EventResult::Handled(());
                }
                KeyCode::Home => {
                    self.head = 0;
                    return EventResult::Handled(());
                }
                KeyCode::End => {
                    self.head = usize::MAX;
                    return EventResult::Handled(());
                }
                _ => {}
            },
            _ => {}
        }
        EventResult::Ignored
    }
}

impl<M, const H: u16> Focusable for Regs<M, H> {
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

trait RegModel {
    type Data;

    fn regs(&self) -> &[(RegName, Self::Data)];
}

impl RegModel for RegFileModel {
    type Data = RegData;

    #[inline(always)]
    fn regs(&self) -> &[(RegName, Self::Data)] {
        self.regs()
    }
}

impl RegModel for FutureFileModel {
    type Data = RegStatSnapshot;

    #[inline(always)]
    fn regs(&self) -> &[(RegName, Self::Data)] {
        self.regs()
    }
}
