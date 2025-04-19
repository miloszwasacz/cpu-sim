use crate::ui::block_style;
use crate::ui::events::{EventHandler, EventResult};
use crate::ui::focus::{Focusable, HasFocus};
use crate::ui::model::{RegFileModel, RegStatModel};

use cpu_sim::components::diagnostics::cpu::{RegData, RegName, RegStatus};
use ratatui::crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui::prelude::*;
use ratatui::widgets::block::Title;
use ratatui::widgets::{Block, Borders, Row, Table};
use std::iter;
use std::marker::PhantomData;

pub mod reg_file;
pub mod reg_stat;

pub struct Regs<M> {
    head: usize,
    focused: bool,
    model: PhantomData<M>,
}

impl<M> Regs<M> {
    pub const HEIGHT: Constraint = Constraint::Length(4);
}

#[allow(private_bounds)]
impl<M: RegModel> Regs<M> {
    fn render_registers<'a, T: Into<Title<'a>>>(
        &mut self,
        title: T,
        model: &M,
        area: Rect,
        buf: &mut Buffer,
    ) {
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
        let data = Row::new(regs.iter().map(|(_, data)| data.to_string()));
        let widths = iter::repeat(Constraint::Length(ITEM_WIDTH))
            .take(count)
            .chain([Constraint::Length(last_width)]);

        let table = Table::new([names, data], widths)
            .column_spacing(ITEM_SPACING)
            .block(block);

        Widget::render(table, area, buf);
    }
}

impl<M: RegModel> Default for Regs<M> {
    fn default() -> Self {
        Self {
            head: Default::default(),
            focused: Default::default(),
            model: PhantomData,
        }
    }
}

impl<M> EventHandler<()> for Regs<M> {
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

impl<M> Focusable for Regs<M> {
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
    type Data: ToString;

    fn regs(&self) -> &[(RegName, Self::Data)];
}

impl RegModel for RegFileModel {
    type Data = RegData;

    #[inline(always)]
    fn regs(&self) -> &[(RegName, Self::Data)] {
        self.regs()
    }
}

impl RegModel for RegStatModel {
    type Data = RegStatus;

    #[inline(always)]
    fn regs(&self) -> &[(RegName, Self::Data)] {
        self.regs()
    }
}
