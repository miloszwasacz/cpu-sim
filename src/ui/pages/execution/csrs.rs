use super::HelpItem;
use crate::ui::component::StatefulComponent;
use crate::ui::events::{EventHandler, EventResult};
use crate::ui::focus::{Focusable, HasFocus};
use crate::ui::model::CsrFileModel;

use ratatui::crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui::layout::Flex;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Cell, Row, Table};

#[derive(Clone, Default)]
pub struct CsrDashboard {
    head: usize,
    focused: bool,
}

impl CsrDashboard {
    const ITEM_HEIGHT: u16 = 1;

    pub(super) fn help(&self) -> impl IntoIterator<Item = HelpItem<'_, '_>> {
        const SCROLL_V: (&str, &str) = ("↑↓", "Scroll");
        const JUMP: (&str, &str) = ("Home/End", "Scroll to start/end");
        [SCROLL_V, JUMP]
    }
}

impl StatefulComponent for CsrDashboard {
    type Model = CsrFileModel;

    fn render(&mut self, model: &Self::Model, area: Rect, buf: &mut Buffer) {
        let height = area.height;
        let count = (height / Self::ITEM_HEIGHT) as usize;

        let csr_count = model.csrs().len();
        if self.head.saturating_add(count) > csr_count {
            self.head = csr_count.saturating_sub(count);
        }

        const COLUMN_SPACING: u16 = 2;
        const FLEX: Flex = Flex::End;
        let widths = [
            Constraint::Length(7),
            Constraint::Length(14),
            Constraint::Length(6),
            Constraint::Fill(1),
        ];

        let [header_area, content_area] =
            Layout::vertical([Constraint::Length(2), Constraint::Fill(1)]).areas(area);

        let header =
            [Row::from_iter(["Address", "Name", "Access", "Value"]).style(Style::new().bold())]
                .into_iter()
                .collect::<Table>()
                .widths(widths)
                .column_spacing(COLUMN_SPACING)
                .flex(FLEX)
                .block(Block::default().borders(Borders::BOTTOM).border_set(
                    symbols::border::Set {
                        bottom_left: "-",
                        bottom_right: "-",
                        horizontal_bottom: "-",
                        ..Default::default()
                    },
                ));

        let table = model
            .csrs()
            .iter()
            .map(|csr| {
                let addr = Cell::new(format!("{}", csr.addr));
                let name = Cell::new(csr.name);
                let access = Cell::new(format!("{}", csr.access));
                let data = Cell::new(csr.data.as_str());
                [addr, name, access, data]
            })
            .map(Row::from_iter)
            .collect::<Table>()
            .widths(widths)
            .column_spacing(COLUMN_SPACING)
            .flex(FLEX);

        Widget::render(header, header_area, buf);
        Widget::render(table, content_area, buf);
    }
}

impl EventHandler<()> for CsrDashboard {
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

impl Focusable for CsrDashboard {
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
