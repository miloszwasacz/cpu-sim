use crate::ui::model::FrontEndModel;
use crate::ui::{block_style, Component, EventHandler, EventResult, Focusable, HasFocus};

use cpu_sim::components::diagnostics::cpu::Exception;
use cpu_sim::components::diagnostics::fmt_addr;
use ratatui::crossterm::event::Event;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders};

#[derive(Default)]
pub struct FrontEnd {
    focused: bool,
}

impl FrontEnd {
    pub const HEIGHT: Constraint = Constraint::Length(5);
}

impl Component for FrontEnd {
    type Model = FrontEndModel;

    fn render(&self, model: &Self::Model, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .title("Front End")
            .borders(Borders::ALL)
            .border_style(block_style(self.focused));

        let [pc_area, fetched_area, decoded_area] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .areas(block.inner(area));

        let pc = fmt_addr(model.pc());
        let fetched = model
            .fetched()
            .map(|fetched| match fetched {
                Ok(instr) => format!("{}", instr),
                Err(ex) => fmt_exception(ex),
            })
            .unwrap_or_default();
        let decoded = model
            .decoded()
            .map(|decoded| match decoded {
                Ok(instr) => format!("{}", instr),
                Err(ex) => fmt_exception(ex),
            })
            .unwrap_or_default();

        block.render(area, buf);
        Line::from_iter([Span::from("PC:      ").bold(), Span::from(pc)]).render(pc_area, buf);
        Line::from_iter([Span::from("Fetched: ").bold(), Span::from(fetched)]).render(fetched_area, buf);
        Line::from_iter([Span::from("Decoded: ").bold(), Span::from(decoded)]).render(decoded_area, buf);
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

fn fmt_exception(exception: Exception) -> String {
    format!("Exception({})", exception)
}
