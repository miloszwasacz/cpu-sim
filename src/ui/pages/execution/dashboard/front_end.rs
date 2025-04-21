use super::EXCEPTION_COLOR;
use crate::ui::model::FrontEndModel;
use crate::ui::{block_style, Component, EventHandler, EventResult, Focusable, HasFocus};

use cpu_sim::components::diagnostics::fmt_addr;
use ratatui::crossterm::event::Event;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, List, ListItem};

#[derive(Default)]
pub struct FrontEnd {
    focused: bool,
}

impl FrontEnd {
    // pub const WIDTH: Constraint = Constraint::Length(38);
    pub const WIDTH: Constraint = Constraint::Fill(1);

    pub fn height(model: &<Self as Component>::Model) -> u16 {
        const BORDER: u16 = 2;
        const PC: u16 = 1;
        const FETCHED_HEADER: u16 = 1;
        BORDER + PC + FETCHED_HEADER + model.decode_width() as u16
    }
}

impl Component for FrontEnd {
    type Model = FrontEndModel;

    fn render(&self, model: &Self::Model, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .title("Front End")
            .borders(Borders::ALL)
            .border_style(block_style(self.focused));

        let [pc_area, fetched_header_area, fetched_area] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Fill(1),
        ])
        .areas(block.inner(area));

        let pc = fmt_addr(model.pc());
        let fetched = model
            .fetched()
            .iter()
            .map(|fetched| match fetched {
                Ok(instr) => format!("{}", instr).into(),
                Err(_) => Span::from("Exception").fg(EXCEPTION_COLOR),
            })
            .map(|span| Line::from(vec![Span::from(" ▶ "), span]))
            .map(ListItem::from);

        block.render(area, buf);
        Line::from_iter([Span::from("PC: ").bold(), Span::from(pc)]).render(pc_area, buf);
        Line::from(Span::from("Fetched:").bold()).render(fetched_header_area, buf);
        Widget::render(List::new(fetched), fetched_area, buf);
    }
}

impl EventHandler<()> for FrontEnd {
    type EventResult = ();

    fn handle_event(&mut self, _: Event, _: ()) -> EventResult<Self::EventResult> {
        EventResult::Ignored
    }
}

impl Focusable for FrontEnd {
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
