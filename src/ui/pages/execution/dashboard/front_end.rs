use super::EXCEPTION_COLOR;
use crate::ui::model::FrontEndModel;
use crate::ui::{block_style, Component, EventHandler, EventResult, Focusable, HasFocus};

use cpu_sim::components::diagnostics::cpu::Prediction;
use cpu_sim::components::diagnostics::fmt_addr;
use ratatui::crossterm::event::Event;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders};

#[derive(Default)]
pub struct FrontEnd {
    focused: bool,
}

impl FrontEnd {
    pub const WIDTH: Constraint = Constraint::Fill(1);

    pub fn height(model: &<Self as Component>::Model) -> u16 {
        Self::block_heights(model).iter().sum()
    }

    fn block_heights(model: &<Self as Component>::Model) -> [u16; 2] {
        let content_height = model.decode_width() as u16;
        [5.max(content_height + 1), content_height + 3]
    }
}

impl Component for FrontEnd {
    type Model = FrontEndModel;

    fn render(&self, model: &Self::Model, area: Rect, buf: &mut Buffer) {
        let first_column_width = Constraint::Length(22);
        let [top, bottom] =
            Layout::vertical(Self::block_heights(model).map(Constraint::Length)).areas(area);

        let [pc_area, fetch_area] =
            Layout::horizontal([first_column_width, Constraint::Fill(1)]).areas(top);
        let [zbp_area, bp_area] =
            Layout::horizontal([first_column_width, Constraint::Fill(1)]).areas(bottom);

        let pc_block = Block::default()
            .title("PC & Diagnostics")
            .borders(Borders::LEFT | Borders::TOP)
            .border_style(block_style(self.focused));

        let zbp_block = Block::default()
            .title("Zero-bubble")
            .borders(Borders::LEFT | Borders::TOP | Borders::BOTTOM)
            .border_set(symbols::border::Set {
                top_left: symbols::line::VERTICAL_RIGHT,
                ..symbols::border::PLAIN
            })
            .border_style(block_style(self.focused));

        let fetch_block = Block::default()
            .title("Fetch")
            .borders(Borders::LEFT | Borders::RIGHT | Borders::TOP)
            .border_set(symbols::border::Set {
                top_left: symbols::line::HORIZONTAL_DOWN,
                ..symbols::border::PLAIN
            })
            .border_style(block_style(self.focused));

        let bp_block = Block::default()
            .title("Branch predictor")
            .borders(Borders::ALL)
            .border_set(symbols::border::Set {
                top_left: symbols::line::CROSS,
                bottom_left: symbols::line::HORIZONTAL_UP,
                top_right: symbols::line::VERTICAL_LEFT,
                ..symbols::border::PLAIN
            })
            .border_style(block_style(self.focused));

        // PC & General diagnostics
        let stats = model.stats();
        let make_line = |header, contents| {
            let header = format!("{}: ", header);
            Line::from(vec![Span::from(header).bold(), Span::from(contents)])
        };
        let pc_line = make_line("PC", fmt_addr(model.pc()));
        let clock_cycle_line = make_line("Clock cycle", stats.clock_cycle.to_string());
        let ipc_line = make_line("IPC", format!("{:.5}", stats.ipc()));
        Text::from(vec![pc_line, Line::default(), clock_cycle_line, ipc_line])
            .render(pc_block.inner(pc_area), buf);
        pc_block.render(pc_area, buf);

        // ZBP
        let entries = model
            .zbp_regs()
            .iter()
            .map(|regs| {
                let predicted = regs.predicted.map(fmt_addr).unwrap_or("-".to_string());
                format!(" ▶ {}: {}", fmt_addr(regs.pc), predicted)
            })
            .map(Line::from);
        let lines = [vec![
            Span::from("Accuracy: ").bold(),
            format!("{:.2}%", model.zb_predictor().accuracy() * 100.0).into(),
        ]]
        .into_iter()
        .map(Line::from)
        .chain(entries)
        .collect::<Vec<_>>();
        Text::from(lines).render(zbp_block.inner(zbp_area), buf);
        zbp_block.render(zbp_area, buf);

        // Fetch
        let lines = model
            .if_regs()
            .iter()
            .map(|regs| {
                let mut line = Line::from(format!(" ▶ {}: ", fmt_addr(regs.pc)));
                line += match regs.instr {
                    Ok(instr) => Span::from(format!("{}", instr)),
                    Err(_) => Span::from("Exception").fg(EXCEPTION_COLOR),
                };
                line
            })
            .collect::<Vec<_>>();
        Text::from(lines).render(fetch_block.inner(fetch_area), buf);
        fetch_block.render(fetch_area, buf);

        // Branch Prediction
        let entries = model
            .bp_regs()
            .iter()
            .map(|regs| {
                format!(
                    " ▶ {}: {}",
                    fmt_addr(regs.pc),
                    match regs.predicted {
                        Prediction::NotTaken => "not taken".to_string(),
                        Prediction::TakenUnknown => "taken".to_string(),
                        Prediction::Taken(addr) => format!("taken ({})", fmt_addr(addr)),
                    }
                )
            })
            .map(Line::from);
        let lines = [vec![
            Span::from("Accuracy: ").bold(),
            format!("{:.2}%", model.branch_predictor().accuracy() * 100.0).into(),
        ]]
        .into_iter()
        .map(Line::from)
        .chain(entries)
        .collect::<Vec<_>>();
        Text::from(lines).render(bp_block.inner(bp_area), buf);
        bp_block.render(bp_area, buf);
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
